use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const GPIO_LED: u8 = 25;

fn main() -> Result<(), Box<dyn Error>> {
	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();

	ctrlc::set_handler(move || {
		r.store(false, Ordering::SeqCst);
	}).expect("Error setting Ctrl-C handler");

	println!("Blinking an LED Chika Chika a {}." , DeviceInfo::new()?.model());

	let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

	while running.load(Ordering::SeqCst) {
		pin.set_high();
		thread::sleep(Duration::from_millis(500));
		pin.set_low();
		thread::sleep(Duration::from_millis(500));
	}

	pin.set_low();
	Ok(())
}
