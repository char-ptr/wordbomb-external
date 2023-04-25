use std::{collections::HashMap, ffi::{CString}, time::Duration, rc::Rc, cell::{RefCell, RefMut}, process::Command};

use wordbomb_external::faker_input::{FakerInput, keyboard_report::{KBDReport, KeyboardKey}, mouse_report::{MouseReport, MouseButtons}};
use mouse_position::mouse_position::Mouse;
use rand::Rng;
use windows::{Win32::{UI::{Input::KeyboardAndMouse::{*}, WindowsAndMessaging::{FindWindowA, SetForegroundWindow}}, Foundation::HWND}, core::{PCSTR}};

fn do_loop<'a>(input_mngr : &mut FakerInput, words : &'a Vec<&'a str>, mut seen: RefMut<Vec<&'a str>>,key_map: &HashMap<char, KeyboardKey>, roblox_handle: HWND) -> Option<()> {

    let mut thd = rand::thread_rng();

    // let locals = PyDict::new(*py);
    // let _result = py.run(include_str!("../ocr.py"),None, Some(locals));
    // let out = locals.get_item("res")?.to_string();
    // println!("ocr output: {:?}", out);

    // return None;
    let out = Command::new("python").arg("./ocr2.py").output().ok()?.stdout;
    let out = String::from_utf8(out).ok()?;
    let j = out.trim();
    println!("ocr output: {}", j);
    let mut split = j.split(' ');
    let op : i32 = split.next()?.parse().ok()?;
    match op {
        2 => {
            let x : i32 = split.next()?.parse().ok()?;
            let y : i32 = split.next()?.parse().ok()?;
            let w : i32 = split.next()?.parse().ok()?;
            let h : i32 = split.next()?.parse().ok()?;
            
            let cen = (x + (w/2), (y + h/2) + 50);
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
        1 =>{
            let word = split.next()?.to_lowercase();
            if word.is_empty() {
                return None;
            }

            // random

            let wo:Vec<&&str> = words.iter()
            .filter(|x| x.contains(word.trim()) && !seen.contains(x) /* && x.len() < max */)
            // .reduce(|a, b| if a.len() > b.len() { a } else { b });
            .collect();
            if wo.len() == 0 {
                println!("No words found");
                return None
            }
        
            let word  =wo.get(thd.gen_range(1..wo.len()))?;    

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
            
            for (idx,c) in w.chars().enumerate() {
                // println!("Sending key: {}", c);
                let inp = key_map.get(&c).expect("bruh");
        
                let report = KBDReport::new()
                .key_down(*inp);
        
                input_mngr.update_keyboard(report);
        
                let report = KBDReport::new()
                .key_up(*inp);
        
                input_mngr.update_keyboard(report);
        
        
                let time = (w.len() as f64 * 4f64) - (idx as f64 * 1.3);
        
                let max_speed = thd.gen_range(40.0..47.0);
        
                // std::thread::sleep(Duration::from_millis(1));
                std::thread::sleep(Duration::from_millis((max_speed + (time % 80.0)) as u64));
            }
            
            let report = KBDReport::new()
            .key_down(KeyboardKey::Enter);
            
            
            
            input_mngr.update_keyboard(report);
        
            let report = KBDReport::new()
            .key_up(KeyboardKey::Enter);
        
            input_mngr.update_keyboard(report);
    
        }
        _=>{}
    }

    Some(())

}

fn main() {
    let words: Vec<&str> = include_str!("../words.txt").split('\n').collect();
    let seen_words : Rc<RefCell<Vec<&str>>> = Rc::new(RefCell::new(vec![]));

    let mut key_map : HashMap<char, KeyboardKey> = HashMap::new();

    let mut finp = FakerInput::new();
    finp.connect();
    // let mut report = MouseReport::new();
    // report.x = 1070;
    // report.y = 300;
    // finp.update_absolute_mouse(&report);
    // std::thread::sleep(Duration::from_millis(500));
    // if let Mouse::Position { x, y } = Mouse::get_mouse_position() {
    //     println!("Mouse position: {}, {}", x, y);
    // }

    // return;

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


    let roblox = unsafe { 
        let rbx_c = CString::new("Roblox").unwrap();
        // let rbx_c = CString::new("Untitled - Notepad").unwrap();
        FindWindowA(PCSTR::null(), PCSTR::from_raw(rbx_c.as_ptr() as *const u8))
    };
    println!("{:?}", roblox);
    // Python::with_gil(|py| {
    loop {
        do_loop(&mut finp, &words, seen_words.clone().borrow_mut(), &key_map, roblox);
        std::thread::sleep(Duration::from_millis(300));
    }
    // }) 

}
