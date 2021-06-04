use chiprust_emu::Chip8;
use std::hint::unreachable_unchecked;
use std::{sync::Arc, time::{Instant, Duration}};
use parking_lot::Mutex;
use winit::{event::{ElementState, Event, KeyboardInput, WindowEvent}, event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder}};
use pixels::{Pixels, SurfaceTexture};
use crate::{BACKGROUND_COLOR, FOREGROUND_COLOR, input};

#[inline(always)]
pub fn draw_thread(chip: Arc<Mutex<Chip8>>, draw_freq: u32) {
    let event_loop = EventLoop::new();
    let win = WindowBuilder::new()
        .with_title("ChipRust Emulator GUI")
        .with_maximized(true)
        .with_transparent(true)
        .build(&event_loop).unwrap();
    
    let size = win.inner_size();

    let width = size.width;
    let height = size.height;

    let surface_texture = SurfaceTexture::new(width, height, &win);
    let mut pixels = Pixels::new(128, 64, surface_texture).unwrap();

    let frame_time =  Duration::from_secs_f64(if draw_freq != 0 {
        1./draw_freq as f64
    } else {
        0.
    });

    event_loop.run(move |e, _, cf| {
        let start = Instant::now();
        *cf = ControlFlow::Wait;
        match e {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {*cf = ControlFlow::Exit; return },
            Event::WindowEvent { event: WindowEvent::Resized(size), ..} => {
                let width = size.width;
                let height = size.height;
                pixels.resize_surface(width, height);
            },
            Event::WindowEvent { event: WindowEvent::KeyboardInput {input: KeyboardInput {state, virtual_keycode, ..}, ..} , .. } => {
                let virtual_keycode = if let Some(v) = virtual_keycode {
                    v
                } else {
                    return
                };
                if input::KEY_MAP.contains_left(&virtual_keycode) {
                    let key = *input::KEY_MAP.get_by_left(&virtual_keycode).unwrap();
                    input::PRESSED_KEYS.lock()[key as usize] = 
                    if let ElementState::Pressed = state {
                        true
                    } else if let ElementState::Released = state {
                        false
                    } else { unsafe {unreachable_unchecked()}};
                    *input::KEY_PRESS.0.lock() = key;
                    input::KEY_PRESS.1.notify_one();
                }
            },
            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
            },
            _ => (),
        }
        let (_chip_state, display) = {
            let mut chip = chip.lock();
            (chip.to_state(), if chip.display.dirty() {Some(*chip.display.read())} else {None})
        };
        if let Some(d) = display {
            let mut frame: Vec<Vec<&mut [u8]>> = pixels.get_frame()
                .chunks_exact_mut(128 * 4)
                .map(|x| x.chunks_exact_mut(4).collect())
                .collect();
            for (y, row) in frame.iter_mut().enumerate() {
                for (x, px) in row.iter_mut().enumerate() {
                    if chiprust_emu::display::get_px(&d, x, y) { 
                        px.copy_from_slice(&FOREGROUND_COLOR)
                    } else {
                        px.copy_from_slice(unsafe {&BACKGROUND_COLOR})
                    }
                }
            }
            pixels.render().unwrap();
        }
        *cf = ControlFlow::WaitUntil(start + frame_time);
    });
}
