mod config;
mod draw_thread;
mod input;
use chiprust_emu::Chip8;
use config::Config;
#[cfg(feature = "sound")]
use rodio::Sink;
use std::{sync::Arc, thread};
use parking_lot::Mutex;
use spin_sleep::LoopHelper;

static FOREGROUND_COLOR: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
static mut BACKGROUND_COLOR: [u8; 4] = [0x10, 0x10, 0x10, 0xFF];

#[inline(always)]
pub fn cpu_thread(chip: Arc<Mutex<Chip8>>, cpu_freq: u32) {
    let loop_helper = LoopHelper::builder()
        .report_interval_s(0.5);
    let mut loop_helper = if cpu_freq != 0 {
        loop_helper.build_with_target_rate(cpu_freq)
    } else {
        loop_helper.build_without_target_rate()
    };

    loop {
        loop_helper.loop_start();
        {
            let mut chip = chip.lock();
            chip.cpu_tick().unwrap();
        };
        loop_helper.loop_sleep()
    }
}

#[inline(always)]
pub fn timers_thread(chip: Arc<Mutex<Chip8>>, timers_freq: u32, #[cfg(feature = "sound")] sink: Option<Sink>) {
    let loop_helper = LoopHelper::builder()
        .report_interval_s(0.5);
    let mut loop_helper = if timers_freq != 0 {
        loop_helper.build_with_target_rate(timers_freq)
    } else {
        loop_helper.build_without_target_rate()
    };

    #[cfg(feature = "sound")]
    let sink = sink.unwrap();

    loop {
        loop_helper.loop_start();
        {
            let mut chip = chip.lock();
            chip.timers_tick();
            #[cfg(feature = "sound")]
            if chip.is_sound_playing() {
                sink.play()
            } else {
                sink.pause()
            }
        }
        loop_helper.loop_sleep()
    }
}

pub fn run() {
    // load args configuration
    let config = match Config::load_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // create an emulator instance and load rom from the config
    let mut chip = Chip8::new
        ::<&'static (dyn Fn() -> u8 + Send + Sync + 'static),
        &'static (dyn Fn(u8) -> bool + Send + Sync + 'static)>
        (&input::key_wait_handler, &input::key_state_handler);

    chip.load(0x200, &config.program, None);

    // wrap the instance into an arc mutex
    let chip = Arc::new(Mutex::new(chip));

    // clone the intance and needed constant values and start the cpu thread
    let chip_clone = chip.clone();
    let cpu_freq = config.cpu_freq;
    thread::spawn(move || cpu_thread(chip_clone, cpu_freq));
    // clone the intance and needed constant values and start the timers thread
    let chip_clone = chip.clone();
    let timers_freq = config.timers_freq;
    #[cfg(feature = "sound")]
    let sink = config.sink;
    thread::spawn(move || timers_thread(chip_clone, timers_freq, #[cfg(feature = "sound")] sink));

    // run draw therad in the main thread for compatability with winit's event loops
    draw_thread::draw_thread(chip, config.draw_freq)
}
