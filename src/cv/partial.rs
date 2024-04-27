use std::{collections::HashMap, ffi::c_void};

use opencv::{
    core::{
        find_non_zero, in_range, Mat, MatTraitConst, Mat_AUTO_STEP, Point_, Scalar, Size_,
        ToInputArray, Vector, BORDER_CONSTANT, CV_8UC4,
    },
    imgcodecs::imencode,
    imgproc::{
        bounding_rect, cvt_color, get_structuring_element, morphology_default_border_value,
        morphology_ex, COLOR_RGBA2BGR, MORPH_OPEN, MORPH_RECT,
    },
};
use rusty_tesseract::{image_to_string, Args, Image};

/// extract the partial part of word

pub fn process_partial(img: &impl ToInputArray) -> eyre::Result<String> {
    let mut ranged = Mat::default();
    let lower_range = Scalar::from((36, 12, 10));
    let upper_range = Scalar::from((40, 16, 14));
    in_range(img, &lower_range, &upper_range, &mut ranged)?;
    let els = get_structuring_element(
        MORPH_RECT,
        Size_::new(3, 3),
        opencv::core::Point_ { x: -1, y: -1 },
    )?;
    let mut out = Mat::default();
    morphology_ex(
        &ranged,
        &mut out,
        MORPH_OPEN,
        &els,
        opencv::core::Point_ { x: -1, y: -1 },
        1,
        BORDER_CONSTANT,
        morphology_default_border_value()?,
    )?;
    let mut locations = Mat::default();
    find_non_zero(&out, &mut locations)?;
    let mut rect = bounding_rect(&locations)?;
    rect += Point_::new(-10, -10);
    rect += Size_::new(20, 20);
    // println!("{:?}", rect);
    // rectangle(&mut bgr, rect, Scalar::from((255, 0, 0)), 2, LINE_8, 0)?;
    let cropped = ranged.roi(rect)?;
    let mut crop_mat = Mat::default();
    cropped.copy_to(&mut crop_mat)?;
    let mut final_out = Vector::default();
    imencode(".jpg", &crop_mat, &mut final_out, &Vector::new())?;
    // named_window("test", WINDOW_AUTOSIZE)?;
    // imshow("test", &crop_mat).unwrap();
    // wait_key(0)?;
    let img = image::load_from_memory(&final_out.to_vec())?;
    let img_tess = Image::from_dynamic_image(&img)?;

    let mut args = Args::default();
    args.lang = "eng".to_string();
    args.psm = Some(6);
    args.oem = Some(3);
    args.dpi = Some(300);
    args.config_variables = HashMap::from([(
        "tessedit_char_whitelist".into(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
    )]);
    let out = image_to_string(&img_tess, &args)?;
    Ok(out)
}
