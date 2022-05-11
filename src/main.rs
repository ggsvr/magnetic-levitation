use std::sync::{mpsc, Arc, Mutex};

use cv::highgui as cv_gui;
use cv::prelude::*;
use opencv as cv;

use levitation::gui::*;
use levitation::{isolate_object, rgb_to_hsv, Hsv};

#[cfg(target_os = "linux")]
const CAP_BACKEND: i32 = cv::videoio::CAP_V4L2;
#[cfg(not(target_os = "linux"))]
const CAP_BACKEND: i32 = cv::videoio::CAP_ANY;

fn main() {
    cv_gui::named_window(WINDOW_NAME, cv_gui::WINDOW_NORMAL).expect("failed to create window");

    let mut cap = cv::videoio::VideoCapture::new(0, CAP_BACKEND).expect("failed to open camera");
    let mut cam_frame = cv::core::Mat::default();

    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    cv_gui::set_mouse_callback(WINDOW_NAME, mouse_pos_callback(tx.clone())).unwrap();

    cv_gui::create_button(
        "Select Object",
        create_button_callback(tx.clone(), Message::SelectObject),
        cv_gui::QT_PUSH_BUTTON,
        false,
    )
    .unwrap();

    cv_gui::create_button(
        "Select Magnet",
        create_button_callback(tx.clone(), Message::SelectMagnet),
        cv_gui::QT_PUSH_BUTTON,
        false,
    )
    .unwrap();

    cv_gui::create_button(
        "Raw Image",
        raw_image_callback(tx.clone()),
        cv_gui::QT_CHECKBOX,
        true,
    )
    .unwrap();

    let hsv_tolerance = Arc::new(Mutex::new(Hsv::new()));
    create_tolerance_trackbars(hsv_tolerance.clone());

    let mut object_color = None;
    let mut select_object = false;
    let mut magnet_color = None;
    let mut select_magnet = false;
    let mut is_raw = true;

    let mut obj_frame = Mat::default();

    loop {
        if !cap.read(&mut cam_frame).unwrap() {
            eprintln!("NO FRAMES GRABBED");
        }
        //cv_gui::imshow(WINDOW_NAME, &cam_frame).unwrap();
        //convert_mat(&mut cam_frame);

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
                Message::Position(x, y) => {
                    let col: rgb::RGB8 = *cam_frame.at_2d(y, x).unwrap();

                    if select_magnet {
                        magnet_color = Some(col);
                        select_magnet = false;
                    } else if select_object {
                        object_color = Some(col);
                        select_object = false;
                    }
                }
                Message::ToggleRaw(b) => is_raw = b,
            }
        }

        if let Some(col) = object_color {
            if is_raw {
                cv_gui::imshow(WINDOW_NAME, &cam_frame).unwrap();
            } else {
                let hsv_tol = hsv_tolerance.lock().unwrap();
                let hsv = rgb_to_hsv(col.r, col.g, col.b);

                isolate_object(
                    &cam_frame,
                    hsv,
                    hsv_tol.h,
                    hsv_tol.s,
                    hsv_tol.v,
                    &mut obj_frame,
                );
                cv_gui::imshow(WINDOW_NAME, &obj_frame).unwrap();
            }
        }

        let key = cv_gui::poll_key().unwrap();
        if key == 27 {
            break;
        }
    }
}
