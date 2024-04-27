use image::{DynamicImage, RgbImage, RgbaImage};
use opencv::{
    core::{
        find_non_zero, in_range, Mat, Mat_AUTO_STEP, Point_, Rect_, Scalar, Size_, Vector,
        BORDER_CONSTANT, CV_8UC4,
    },
    highgui::{imshow, named_window, wait_key, WINDOW_AUTOSIZE},
    imgcodecs::{imdecode, imencode, imwrite},
    imgproc::{
        bounding_rect, cvt_color, get_structuring_element, morphology_default_border_value,
        morphology_ex, rectangle, COLOR_RGBA2BGR, COLOR_RGBA2BGRA, LINE_8, MORPH_OPEN, MORPH_RECT,
    },
    prelude::*,
};
use rusty_tesseract::{image_to_string, Args, Image};
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    ffi::CString,
    fs::{self, File},
    io::Write,
    ops::Add,
    os::windows::thread,
    process::Command,
    rc::Rc,
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use mouse_position::mouse_position::Mouse;
use rand::Rng;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        UI::{
            Input::KeyboardAndMouse::*,
            WindowsAndMessaging::{FindWindowA, SetForegroundWindow},
        },
    },
};
use windows_capture::{
    capture::GraphicsCaptureApiHandler,
    encoder::{ImageEncoder, VideoEncoder, VideoEncoderQuality, VideoEncoderType},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    settings::Settings,
    window::Window,
};
use wordbomb_external::faker_input::{
    keyboard_report::{KBDReport, KeyboardKey},
    mouse_report::{MouseButtons, MouseReport},
    FakerInput,
};

fn do_loop<'a>(
    input_mngr: &mut FakerInput,
    words: &'a Vec<&'a str>,
    mut seen: RefMut<Vec<&'a str>>,
    key_map: &HashMap<char, KeyboardKey>,
    roblox_handle: HWND,
) -> Option<()> {
    let mut thd = rand::thread_rng();

    // let locals = PyDict::new(*py);
    // let _result = py.run(include_str!("../ocr.py"),None, Some(locals));
    // let out = locals.get_item("res")?.to_string();
    // println!("ocr output: {:?}", out);

    // return None;
    let out = Command::new("python")
        .arg("./ocr2.py")
        .output()
        .ok()?
        .stdout;
    let out = String::from_utf8(out).ok()?;
    let j = out.trim();
    println!("ocr output: {}", j);
    let mut split = j.split(' ');
    let op: i32 = split.next()?.parse().ok()?;
    match op {
        2 => {
            let x: i32 = split.next()?.parse().ok()?;
            let y: i32 = split.next()?.parse().ok()?;
            let w: i32 = split.next()?.parse().ok()?;
            let h: i32 = split.next()?.parse().ok()?;

            let cen = (x + (w / 2), (y + h / 2) + 50);
            println!("Clicking at: {:?}", cen);
            let mut report = MouseReport::new();
            report.x = cen.0 as i16;
            report.y = cen.1 as i16;
            report.button_down(MouseButtons::Left);
            input_mngr.update_absolute_mouse(&report);

            report.reset_position();
            report.button_up(MouseButtons::Left);
            input_mngr.update_relative_mouse(&report);
        }
        1 => {
            let word = split.next()?.to_lowercase();
            if word.is_empty() {
                return None;
            }

            // random

            let wo: Vec<&&str> = words
                .iter()
                .filter(
                    |x| x.contains(word.trim()) && !seen.contains(x), /* && x.len() < max */
                )
                // .reduce(|a, b| if a.len() > b.len() { a } else { b });
                .collect();
            if wo.len() == 0 {
                println!("No words found");
                return None;
            }

            let word = wo.get(thd.gen_range(1..wo.len()))?;

            // asshole

            // let word = words.iter()
            // .filter(|x| x.contains(word.trim()) && !seen.contains(x) /* && x.len() < max */)
            // .reduce(|a, b| if a.len() > b.len() { a } else { b })?;

            println!("Word: {}", word);

            seen.push(word);

            let w = word.trim();

            unsafe {
                SetForegroundWindow(roblox_handle);
                SetFocus(roblox_handle);
                SetActiveWindow(roblox_handle);
                std::thread::sleep(Duration::from_millis(300));
            }

            for (idx, c) in w.chars().enumerate() {
                // println!("Sending key: {}", c);
                let inp = key_map.get(&c).expect("bruh");

                let report = KBDReport::new().key_down(*inp);

                input_mngr.update_keyboard(report);

                let report = KBDReport::new().key_up(*inp);

                input_mngr.update_keyboard(report);

                let time = (w.len() as f64 * 4f64) - (idx as f64 * 1.3);

                let max_speed = thd.gen_range(40.0..47.0);

                // std::thread::sleep(Duration::from_millis(1));
                std::thread::sleep(Duration::from_millis((max_speed + (time % 80.0)) as u64));
            }

            let report = KBDReport::new().key_down(KeyboardKey::Enter);

            input_mngr.update_keyboard(report);

            let report = KBDReport::new().key_up(KeyboardKey::Enter);

            input_mngr.update_keyboard(report);
        }
        _ => {}
    }

    Some(())
}

enum Communication {
    Ignore,
    Word(String),
    GameStart((i32, i32)),
}
struct Capture {
    // The video encoder that will be used to encode the frames.
    tx: Sender<Communication>, // To measure the time the capture has been running
}

impl GraphicsCaptureApiHandler for Capture {
    // The type of flags used to get the values from the settings.
    type Flags = Sender<Communication>;

    // The type of error that can occur during capture, the error will be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create the struct. The flags can be passed from settings.
    fn new(message: Self::Flags) -> Result<Self, Self::Error> {
        Ok(Self { tx: message })
    }

    // Called every time a new frame is available.
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        // Send the frame to the video encoder
        // frame.save_as_image("./test.png", windows_capture::frame::ImageFormat::Png);
        // let cv2_vec = opencv::core::Vector;
        let Ok(mut buf) = frame.buffer() else {
            return Ok(());
        };
        let img = unsafe {
            Mat::new_size_with_data(
                Size_::new(buf.width() as i32, buf.height() as i32),
                CV_8UC4,
                buf.as_raw_nopadding_buffer()? as *mut _ as *mut _,
                Mat_AUTO_STEP,
            )
        }?;
        let mut bgr = Mat::default();
        cvt_color(&img, &mut bgr, COLOR_RGBA2BGR, 0)?;
        let mut ranged = Mat::default();
        let lower_range = Scalar::from((36, 12, 10));
        let upper_range = Scalar::from((40, 16, 14));
        in_range(&bgr, &lower_range, &upper_range, &mut ranged)?;
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
        if let Ok(cropped) = ranged.roi(rect) {
            let mut crop_mat = Mat::default();
            cropped.copy_to(&mut crop_mat)?;
            let mut final_out = Vector::default();
            imencode(".jpg", &crop_mat, &mut final_out, &Vector::new())?;
            fs::write("./test.jpg", &final_out)?;
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
            self.tx.send(Communication::Word(out));
            // named_window("test", WINDOW_AUTOSIZE)?;
            // imshow("test", &crop_mat).unwrap();
            // wait_key(0)?;
        }

        Ok(())
    }

    // Optional handler called when the capture item (usually a window) closes.
    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture Session Closed");

        Ok(())
    }
}

fn main() {
    let words: Vec<&str> = include_str!("../words.txt").split('\n').collect();
    let seen_words: Rc<RefCell<Vec<&str>>> = Rc::new(RefCell::new(vec![]));

    let mut key_map: HashMap<char, KeyboardKey> = HashMap::new();

    let mut finp = FakerInput::new().expect("unable to load dll");
    finp.connect();

    key_map.insert('a', KeyboardKey::A);
    key_map.insert('b', KeyboardKey::B);
    key_map.insert('c', KeyboardKey::C);
    key_map.insert('d', KeyboardKey::D);
    key_map.insert('e', KeyboardKey::E);
    key_map.insert('f', KeyboardKey::F);
    key_map.insert('g', KeyboardKey::G);
    key_map.insert('h', KeyboardKey::H);
    key_map.insert('i', KeyboardKey::I);
    key_map.insert('j', KeyboardKey::J);
    key_map.insert('k', KeyboardKey::K);
    key_map.insert('l', KeyboardKey::L);
    key_map.insert('m', KeyboardKey::M);
    key_map.insert('n', KeyboardKey::N);
    key_map.insert('o', KeyboardKey::O);
    key_map.insert('p', KeyboardKey::P);
    key_map.insert('q', KeyboardKey::Q);
    key_map.insert('r', KeyboardKey::R);
    key_map.insert('s', KeyboardKey::S);
    key_map.insert('t', KeyboardKey::T);
    key_map.insert('u', KeyboardKey::U);
    key_map.insert('v', KeyboardKey::V);
    key_map.insert('w', KeyboardKey::W);
    key_map.insert('x', KeyboardKey::X);
    key_map.insert('y', KeyboardKey::Y);
    key_map.insert('z', KeyboardKey::Z);
    key_map.insert(' ', KeyboardKey::Space);
    key_map.insert('-', KeyboardKey::Minus);
    key_map.insert('=', KeyboardKey::Equals);
    key_map.insert('⭐', KeyboardKey::Enter);

    let (tx, rx) = std::sync::mpsc::channel();
    let rbxw = Window::from_name("Roblox").expect("unable to find roblox in processes");
    let sets = Settings::new(
        rbxw,
        windows_capture::settings::CursorCaptureSettings::WithoutCursor,
        windows_capture::settings::DrawBorderSettings::WithoutBorder,
        windows_capture::settings::ColorFormat::Rgba8,
        tx,
    )
    .expect("wha");
    std::thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                Communication::Word(x) => {
                    println!("video stream sent sequence {}", x)
                }
                _ => {}
            }
        }
    });
    let cap = Capture::start(sets).expect("unable to start recording");
}
