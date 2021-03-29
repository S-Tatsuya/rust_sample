use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::system::DeviceInfo;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const GPIO_SWITCH: u8 = 24;

fn main() -> Result<(), Box<dyn Error>> {
	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();

	ctrlc::set_handler(move || {
		r.store(false, Ordering::SeqCst);
	}).expect("Error setting Ctrl-C handler");

	println!("Blinking an Switch Control LED {}", DeviceInfo::new()?.model());

	let switch = Gpio::new()?.get(GPIO_SWITCH)?.into_input();

	while running.load(Ordering::SeqCst) {
		thread::sleep(Duration::from_millis(500));
		if switch.read() == Level::High {
			println!("Switch is ON.");
		} else {
			println!("Switch is OFF.");
		}
	}

	Ok(())
}