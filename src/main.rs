use std::sync::mpsc;

use cv::highgui as gui;
use cv::prelude::*;
use opencv as cv;

#[cfg(target_os = "linux")]
const CAP_BACKEND: i32 = cv::videoio::CAP_V4L2;
#[cfg(not(target_os = "linux"))]
const CAP_BACKEND: i32 = cv::videoio::CAP_ANY;

const WINDOW_NAME: &str = "main_window";

#[derive(Debug, Clone, Copy)]
enum Message {
    SelectObject,
    Position(i32, i32),
    //Object(i32, i32),
    SelectMagnet,
    //Magnet(i32, i32),
}

fn main() {
    gui::named_window(WINDOW_NAME, gui::WINDOW_NORMAL).expect("failed to create window");

    let mut cap = cv::videoio::VideoCapture::new(0, CAP_BACKEND).expect("failed to open camera");
    let mut cam_frame = cv::core::Mat::default();

    let (tx, rx) = mpsc::sync_channel(10);

    gui::set_mouse_callback(WINDOW_NAME, create_mouse_callback(tx.clone())).unwrap();
    //gui::create_trackbar("Object(0), Magnet(2) Selection", WINDOW_NAME, None, 2, None).unwrap();

    gui::create_button(
        "Select Object",
        create_button_callback(tx.clone(), Message::SelectObject),
        gui::QT_PUSH_BUTTON,
        false,
    )
    .unwrap();

    gui::create_button(
        "Select Magnet",
        create_button_callback(tx.clone(), Message::SelectMagnet),
        gui::QT_PUSH_BUTTON,
        false,
    )
    .unwrap();

    let mut object_color = None;
    let mut select_object = false;
    let mut magnet_color = None;
    let mut select_magnet = false;

    loop {
        if !cap.read(&mut cam_frame).unwrap() {
            eprintln!("NO FRAMES GRABBED");
        }
        gui::imshow(WINDOW_NAME, &cam_frame).unwrap();
        let key = gui::poll_key().unwrap();
        if key == 27 {
            break;
        }
        convert_mat(&mut cam_frame);

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
                    let value: rgb::alt::BGR8 = *cam_frame.at_2d(y, x).unwrap();

                    if select_magnet {
                        magnet_color = Some(value);
                        println!("magnet color: {magnet_color:?}");
                        select_magnet = false;
                    } else if select_object {
                        object_color = Some(value);
                        println!("object color: {object_color:?}");
                        select_object = false;
                    }
                }
            }
            //let (target_color, x, y) = match msg {
            //    PositionMessage::Object(x, y) => (&mut object_color, x, y),
            //    PositionMessage::Magnet(x, y) => (&mut magnet_color, x, y),
            //};

            //let value: rgb::alt::BGR8 = *cam_frame.at_2d(y, x).unwrap();
            //*target_color = Some(value);
            //println!("{:?}", *target_color);
        }
    }
}

fn create_trackbar_callback(tx: mpsc::SyncSender<Message>) -> gui::TrackbarCallback {
    Some(Box::new(move |val| match val {
        0 => tx.send(Message::SelectObject).unwrap(),
        2 => tx.send(Message::SelectMagnet).unwrap(),
        _ => (),
    }))
}

fn create_mouse_callback(tx: mpsc::SyncSender<Message>) -> gui::MouseCallback {
    Some(Box::new(move |event, x, y, _flags| match event {
        gui::EVENT_LBUTTONDOWN => tx.send(Message::Position(x, y)).unwrap(),
        _ => (),
    }))
}
fn create_button_callback(tx: mpsc::SyncSender<Message>, target: Message) -> gui::ButtonCallback {
    Some(Box::new(move |_| {
        tx.send(dbg!(target)).unwrap();
    }))
}

fn convert_mat(mat: &mut Mat) {
    if mat.typ() == cv::core::CV_8UC3 {
        return;
    }

    let mut out = Mat::default();
    mat.convert_to(&mut out, cv::core::CV_8UC3, 1., 0.).unwrap();
    *mat = out;
}
