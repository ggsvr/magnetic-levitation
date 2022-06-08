pub mod color;
pub mod gui;
pub mod serial;
use std::f32::consts::PI;

use color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
}

use cv::{
    core::Ptr,
    features2d::{SimpleBlobDetector, SimpleBlobDetector_Params},
    prelude::*,
};
use opencv as cv;

fn lower_upper(color: Color, tolerance: u8) -> (Color, Color) {
    (color - tolerance, color + tolerance)
}

pub fn isolate_obj(img: &Mat, color: Color, tolerance: u8, dst: &mut Mat) {
    let (lower, upper) = lower_upper(color, tolerance);

    let lower = cv::core::Scalar::from((lower.b().into(), lower.g().into(), lower.r().into()));
    let upper = cv::core::Scalar::from((upper.b().into(), upper.g().into(), upper.r().into()));

    let mut out = Mat::default();
    cv::core::in_range(img, &lower, &upper, &mut out).unwrap();
    cv::imgproc::cvt_color(&out, dst, cv::imgproc::COLOR_GRAY2BGR, 3).unwrap();
}

fn create_blob_detector() -> Ptr<SimpleBlobDetector> {
    let blob_params = SimpleBlobDetector_Params {
        filter_by_color: true,
        blob_color: 255,
        filter_by_area: false,
        filter_by_circularity: false,
        filter_by_convexity: false,
        filter_by_inertia: false,
        ..SimpleBlobDetector_Params::default().unwrap()
    };

    SimpleBlobDetector::create(blob_params).unwrap()
}

pub fn process_image(src: &Mat, color: Color, tolerance: u8, dst: &mut Mat) -> Option<Ball> {
    let mut blob_detector = create_blob_detector();

    isolate_obj(src, color, tolerance, dst);

    let mut keypoints = cv::core::Vector::new();
    blob_detector
        .detect(dst, &mut keypoints, &cv::core::no_array())
        .unwrap();

    //assert!(
    //    keypoints.len() <= 1,
    //    "More than 1 blob detected, maybe check color calibration"
    //);

    match keypoints.get(0) {
        Ok(kp) => Some(Ball {
            x: kp.pt.x,
            y: kp.pt.y,
        }),
        Err(_) => None,
    }
}

pub fn save_img(img: &Mat, ball: Ball) {
    let size = img.size().unwrap();
    let mut out = image::RgbImage::new(size.width as u32, size.height as u32);

    for (x, y, pixel) in out.enumerate_pixels_mut() {
        let color: &cv::core::Vec3b = img.at_2d(y as i32, x as i32).unwrap();
        let color = Color::new(color.0[2], color.0[1], color.0[0]);

        let x = x as f32;
        let y = y as f32;

        let distance = ((x - ball.x).powi(2) + (y - ball.y).powi(2)).sqrt().abs();
        if distance < 10. {
            *pixel = image::Rgb([255, 0, 0]);
        } else {
            *pixel = image::Rgb(color.channels);
        }
    }

    out.save("out.png").unwrap();
}

//use serialport::SerialPort;
//use std::sync::mpsc;
//use std::thread;
//
//pub fn serial(port: Option<Box<dyn SerialPort>>) -> mpsc::Sender<[u8; 4]> {
//    let (tx, rx) = mpsc::channel::<[u8; 4]>();
//
//    match port {
//        Some(mut p) => thread::spawn(move || {
//            while let Ok(bytes) = rx.recv() {
//                p.write_all(&bytes).unwrap();
//            }
//        }),
//        None => thread::spawn(move || {
//            while let Ok(bytes) = rx.recv() {
//                eprintln!("{bytes:?}");
//            }
//        }),
//    };
//
//    tx
//}
//
