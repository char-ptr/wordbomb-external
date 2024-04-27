use std::ffi::c_void;

use opencv::{
    core::{Mat, MatTraitConst, Mat_AUTO_STEP, Rect, Size_, CV_8UC4},
    imgproc::{cvt_color, COLOR_RGBA2BGR},
};

use crate::Communication;

pub mod partial;
pub mod turn;

pub fn process_image(
    width: i32,
    height: i32,
    image_buf: *const c_void,
) -> eyre::Result<Communication> {
    let img = unsafe {
        Mat::new_size_with_data(
            Size_::new(width, height),
            CV_8UC4,
            image_buf as *mut _,
            Mat_AUTO_STEP,
        )
    }?;
    let mut bgr = Mat::default();
    cvt_color(&img, &mut bgr, COLOR_RGBA2BGR, 0)?;
    let crop_ref = bgr.roi(Rect::new(0, height / 4, width - 300, height / 2))?;
    let white_text = turn::process_turn(&crop_ref)?;
    // println!("white_text: {}", white_text);
    if white_text.to_lowercase().contains("quick") {
        // its your turn, let's find out the partial
        let part = partial::process_partial(&crop_ref)?;
        return Ok(Communication::WordPart(part));
        // println!("part: {}", part);
    }
    Ok(Communication::Ignore)
}
