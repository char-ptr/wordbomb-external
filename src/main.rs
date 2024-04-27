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
    sync::{
        atomic::AtomicBool,
        mpsc::{Receiver, Sender},
        Arc,
    },
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
use wordbomb_external::{
    cv::{partial::process_partial, process_image},
    faker_input::{
        keyboard_report::{KBDReport, KeyboardKey},
        mouse_report::{MouseButtons, MouseReport},
        FakerInput,
    },
    Communication,
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

struct Capture {
    // The video encoder that will be used to encode the frames.
    tx: Sender<Communication>, // To measure the time the capture has been running
    send_data: Arc<AtomicBool>,
}

impl GraphicsCaptureApiHandler for Capture {
    // The type of flags used to get the values from the settings.
    type Flags = (Sender<Communication>, Arc<AtomicBool>);

    // The type of error that can occur during capture, the error will be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create the struct. The flags can be passed from settings.
    fn new(message: Self::Flags) -> Result<Self, Self::Error> {
        Ok(Self {
            tx: message.0,
            send_data: message.1,
        })
    }

    // Called every time a new frame is available.
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        _: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        // Send the frame to the video encoder
        // frame.save_as_image("./test.png", windows_capture::frame::ImageFormat::Png);
        // let cv2_vec = opencv::core::Vector;
        let data = self.send_data.load(std::sync::atomic::Ordering::Relaxed);
        // println!("Data: {}", data);
        if !data {
            return Ok(());
        }
        let Ok(mut buf) = frame.buffer() else {
            return Ok(());
        };
        let out = process_image(
            buf.width() as i32,
            buf.height() as i32,
            buf.as_raw_nopadding_buffer()?.as_ptr() as *const _,
        );
        match out {
            Ok(part) => {
                self.tx.send(part);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        };
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
    let mut seen_words: Vec<&str> = vec![];

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
    let send_data = Arc::new(AtomicBool::new(true));
    let sets = Settings::new(
        rbxw,
        windows_capture::settings::CursorCaptureSettings::WithoutCursor,
        windows_capture::settings::DrawBorderSettings::WithoutBorder,
        windows_capture::settings::ColorFormat::Rgba8,
        (tx, send_data.clone()),
    )
    .expect("wha");
    let mut thd = rand::thread_rng();
    std::thread::spawn(move || {
        let cap = Capture::start(sets).expect("unable to start recording");
    });
    while let Ok(msg) = rx.recv() {
        match msg {
            Communication::WordPart(word) => {
                send_data
                    .clone()
                    .store(false, std::sync::atomic::Ordering::Relaxed);
                let flip = Flipper(send_data.clone());
                if word.is_empty() {
                    continue;
                }
                let lower = word.to_lowercase();
                println!("video stream sent sequence {} ", word);
                let wo: Vec<&&str> = words
                        .iter()
                        .filter(
                            |x| x.contains(lower.trim()) && !seen_words.contains(x), /* && x.len() < max */
                        )
                        // .reduce(|a, b| if a.len() > b.len() { a } else { b });
                        .collect();
                if wo.is_empty() {
                    println!("No words found");
                    continue;
                }

                let Some(word) = wo.get(thd.gen_range(1..wo.len())) else {
                    continue;
                };

                // asshole

                // let word = words.iter()
                // .filter(|x| x.contains(word.trim()) && !seen.contains(x) /* && x.len() < max */)
                // .reduce(|a, b| if a.len() > b.len() { a } else { b })?;

                println!("Word: {}", word);
                // unsafe {
                //     SetForegroundWindow(roblox_handle);
                //     SetFocus(roblox_handle);
                //     SetActiveWindow(roblox_handle);
                //     std::thread::sleep(Duration::from_millis(300));
                // }

                let w = word.trim();
                for (idx, c) in w.chars().enumerate() {
                    // println!("Sending key: {}", c);
                    let inp = key_map.get(&c).expect("bruh");

                    let report = KBDReport::new().key_down(*inp);

                    finp.update_keyboard(report);

                    let report = KBDReport::new().key_up(*inp);

                    finp.update_keyboard(report);

                    let time = (w.len() as f64 * 4f64) - (idx as f64 * 1.3);

                    let max_speed = thd.gen_range(40.0..47.0);

                    // std::thread::sleep(Duration::from_millis(1));
                    std::thread::sleep(Duration::from_millis((max_speed + (time % 80.0)) as u64));
                }

                let report = KBDReport::new().key_down(KeyboardKey::Enter);

                finp.update_keyboard(report);

                let report = KBDReport::new().key_up(KeyboardKey::Enter);

                finp.update_keyboard(report);

                seen_words.push(word);
            }
            Communication::Ignore => {}
            _ => {}
        }
    }
}
struct Flipper(Arc<AtomicBool>);
impl Drop for Flipper {
    fn drop(&mut self) {
        self.0.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
