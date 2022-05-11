pub mod gui;

use cv::core::Scalar;
use cv::prelude::*;
use opencv as cv;

#[derive(Debug, Clone)]
pub struct Hsv {
    pub h: u8,
    pub s: u8,
    pub v: u8,
}

impl Hsv {
    pub fn new() -> Self {
        Self { h: 0, s: 0, v: 0 }
    }
}

pub fn convert_mat(mat: &mut Mat) {
    if mat.typ() == cv::core::CV_8UC3 {
        return;
    }

    let mut out = Mat::default();
    mat.convert_to(&mut out, cv::core::CV_8UC3, 1., 0.).unwrap();
    *mat = out;
}

pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> Hsv {
    let r = r as f32 / 255.;
    let g = g as f32 / 255.;
    let b = b as f32 / 255.;
    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let delta = c_max - c_min;

    // 0 <= v <= 1
    let v = c_max;

    // 0 <= s <= 1
    let s = if c_max == 0. { 0. } else { delta / c_max };

    // 0 <= h <= 360
    let h = if c_max == r {
        60. * (g - b) / delta
    } else if c_max == g {
        120. + 60. * (b - r) / delta
    } else if c_max == b {
        240. + 60. * (r - g) / delta
    } else {
        0.
    };

    let h = (h / 2.) as u8;
    let s = (s * 255.) as u8;
    let v = (v * 255.) as u8;

    Hsv { h, s, v }
}

pub fn isolate_object(
    img: &Mat,
    color: Hsv,
    h_tolerance: u8,
    s_tolerance: u8,
    v_tolerance: u8,
    dst: &mut Mat,
) {
    assert!(
        h_tolerance <= 180,
        "HUE value must be between 0 and 180, but got {h_tolerance}"
    );

    let mut hsv_img = Mat::default();
    cv::imgproc::cvt_color(&img, &mut hsv_img, cv::imgproc::COLOR_BGR2HSV, 0).unwrap();

    let mut low_hsv = [color.h, color.s, color.v];
    let mut high_hsv = [color.h, color.s, color.v];

    for (i, &channel) in [h_tolerance, s_tolerance, v_tolerance].iter().enumerate() {
        low_hsv[i] = low_hsv[i].saturating_sub(channel);
        high_hsv[i] = high_hsv[i].saturating_add(channel);
    }

    high_hsv[0] = high_hsv[0].min(180);

    cv::core::in_range(
        &hsv_img,
        &cv::core::VecN(low_hsv),
        &cv::core::VecN(high_hsv),
        dst,
    )
    .unwrap();
}
