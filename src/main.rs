use std::sync::{
    atomic::{AtomicU8, Ordering::SeqCst},
    mpsc, Arc, Mutex,
};

use cv::highgui as cv_gui;
use cv::prelude::*;
use opencv as cv;

use levitation::color::Color;
use levitation::gui::*;
use levitation::isolate_obj;
use levitation::process_image;
use levitation::serial;

#[cfg(target_os = "linux")]
const CAP_BACKEND: i32 = cv::videoio::CAP_V4L2;
#[cfg(not(target_os = "linux"))]
const CAP_BACKEND: i32 = cv::videoio::CAP_ANY;

fn main() {
    // setup
    let mut port = serial::select_port().unwrap();
    cv_gui::named_window(WINDOW_NAME, cv_gui::WINDOW_NORMAL).expect("failed to create window");

    let mut cap = cv::videoio::VideoCapture::new(0, CAP_BACKEND).expect("failed to open camera");
    let mut cam_frame = cv::core::Mat::default();

    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    cv_gui::set_mouse_callback(WINDOW_NAME, mouse_pos_callback(tx.clone())).unwrap();

    levitation::gui::create_buttons(tx.clone());

    let tolerance = Arc::new(AtomicU8::new(0));
    levitation::gui::create_tolerance_trackbar(tolerance.clone());

    // object uses detection by color,
    // while the magnet just uses the position,
    // since the magnet will stay still relative to the camera
    let mut object_color = None;
    let mut select_object = false;
    let mut magnet_pos = None;
    let mut select_magnet = false;
    let mut is_raw = true;

    let mut ball = None;

    let mut obj_frame = Mat::default();

    port.set_timeout(std::time::Duration::from_millis(500))
        .unwrap();

    //let port = levitation::select_port().unwrap();

    loop {
        // get camera frame
        if !cap.read(&mut cam_frame).unwrap() {
            eprintln!("NO FRAMES GRABBED");
        }

        // listen for message from UI elements
        if let Ok(msg) = rx.try_recv() {
            match msg {
                Message::SelectObject => {
                    select_object = true;
                    select_magnet = false;
                }
                Message::SelectMagnet => {
                    select_magnet = true;
                    select_object = false;
                }

                // mouse pointer position
                Message::Position(x, y) => {
                    // convert MAT element to Color
                    let col: cv::core::Vec3b = *cam_frame.at_2d(y, x).unwrap();
                    let col = Color::new(col.0[2], col.0[1], col.0[0]);

                    if select_magnet {
                        magnet_pos = Some((x, y));
                        select_magnet = false;
                    } else if select_object {
                        object_color = Some(col);
                        select_object = false;
                    }
                }
                Message::ToggleRaw(b) => is_raw = b,
                Message::SaveImg => {
                    if let Some(b) = ball {
                        levitation::save_img(&cam_frame, b);
                    }
                }
            }
        }

        if let Some(col) = object_color {
            if is_raw {
                cv_gui::imshow(WINDOW_NAME, &cam_frame).unwrap();
            } else {
                let tol = tolerance.load(SeqCst);

                //isolate_obj(&cam_frame, col, tol, &mut obj_frame);
                ball = process_image(&cam_frame, col, tol, &mut obj_frame);
                if let Some(b) = &ball {
                    //println!("{b:?}");
                    serial::send_data(&mut *port, b.y);
                }
                cv_gui::imshow(WINDOW_NAME, &obj_frame).unwrap();
            }
        } else {
            cv_gui::imshow(WINDOW_NAME, &cam_frame).unwrap();
        }

        let key = cv_gui::poll_key().unwrap();
        if key == 27 {
            break;
        }
    }
}
