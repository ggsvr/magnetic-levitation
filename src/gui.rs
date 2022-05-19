use opencv::highgui as cv_gui;
use std::sync::{mpsc, Arc, Mutex};

pub const WINDOW_NAME: &str = "Magnetic Levitation";

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectObject,
    Position(i32, i32),
    SelectMagnet,
    ToggleRaw(bool),
    SaveImg,
}

pub type Sender = Arc<Mutex<mpsc::Sender<Message>>>;

pub fn mouse_pos_callback(tx: Sender) -> cv_gui::MouseCallback {
    Some(Box::new(move |event, x, y, _flags| match event {
        cv_gui::EVENT_LBUTTONDOWN => tx.lock().unwrap().send(Message::Position(x, y)).unwrap(),
        _ => (),
    }))
}

pub fn create_button_callback(tx: Sender, target: Message) -> cv_gui::ButtonCallback {
    Some(Box::new(move |_| {
        tx.lock().unwrap().send(target).unwrap();
    }))
}

pub fn raw_image_callback(tx: Sender) -> cv_gui::ButtonCallback {
    Some(Box::new(move |val| {
        let val = if val > 0 { true } else { false };
        tx.lock().unwrap().send(Message::ToggleRaw(val)).unwrap();
    }))
}

//pub fn create_tolerance_trackbars(hsv: Arc<Mutex<Hsv>>) {
//    let create_trackbar = |name, max_val, closure| {
//        cv_gui::create_trackbar(name, WINDOW_NAME, None, max_val, Some(closure)).unwrap();
//    };
//
//    let hsv1 = hsv.clone();
//    create_trackbar(
//        "H Tolerance",
//        255,
//        Box::new(move |val| hsv1.lock().unwrap().h = val as u8),
//    );
//    let hsv1 = hsv.clone();
//    create_trackbar(
//        "S Tolerance",
//        255,
//        Box::new(move |val| hsv1.lock().unwrap().s = val as u8),
//    );
//    let hsv1 = hsv.clone();
//    create_trackbar(
//        "V Tolerance",
//        255,
//        Box::new(move |val| hsv1.lock().unwrap().v = val as u8),
//    );
//}
use std::sync::atomic::{AtomicU8, Ordering::SeqCst};
pub fn create_tolerance_trackbar(tol: Arc<AtomicU8>) {
    cv_gui::create_trackbar(
        "Tolerance",
        WINDOW_NAME,
        None,
        255,
        Some(Box::new(move |val| tol.store(val as u8, SeqCst))),
    )
    .unwrap();
}

pub fn create_buttons(tx: Sender) {
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

    cv_gui::create_button(
        "Save Image",
        create_button_callback(tx.clone(), Message::SaveImg),
        cv_gui::QT_PUSH_BUTTON,
        false,
    )
    .unwrap();
}
//pub fn create_tolerance_trackbars_rgb(rgb: Arc<Mutex<crate::color::Color>>) {
//    let create_trackbar = |name, max_val, closure| {
//        cv_gui::create_trackbar(name, WINDOW_NAME, None, max_val, Some(closure)).unwrap();
//    };
//
//    let rgb1 = rgb.clone();
//    create_trackbar(
//        "R Tolerance",
//        255,
//        Box::new(move |val| *rgb1.lock().unwrap().r_mut() = val as u8),
//    );
//    let rgb1 = rgb.clone();
//    create_trackbar(
//        "G Tolerance",
//        255,
//        Box::new(move |val| *rgb1.lock().unwrap().g_mut() = val as u8),
//    );
//    let rgb1 = rgb.clone();
//    create_trackbar(
//        "B Tolerance",
//        255,
//        Box::new(move |val| *rgb1.lock().unwrap().b_mut() = val as u8),
//    );
//}
