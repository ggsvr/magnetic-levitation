use crate::Hsv;
use opencv::highgui as cv_gui;
use std::sync::{mpsc, Arc, Mutex};

pub const WINDOW_NAME: &str = "Magnetic Levitation";

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectObject,
    Position(i32, i32),
    SelectMagnet,
    ToggleRaw(bool),
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

pub fn create_tolerance_trackbars(hsv: Arc<Mutex<Hsv>>) {
    let create_trackbar = |name, max_val, closure| {
        cv_gui::create_trackbar(name, WINDOW_NAME, None, max_val, Some(closure)).unwrap();
    };

    let hsv1 = hsv.clone();
    create_trackbar(
        "H Tolerance",
        180,
        Box::new(move |val| hsv1.lock().unwrap().h = val as u8),
    );
    let hsv1 = hsv.clone();
    create_trackbar(
        "S Tolerance",
        255,
        Box::new(move |val| hsv1.lock().unwrap().s = val as u8),
    );
    let hsv1 = hsv.clone();
    create_trackbar(
        "V Tolerance",
        255,
        Box::new(move |val| hsv1.lock().unwrap().v = val as u8),
    );
}
