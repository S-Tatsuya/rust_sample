use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, Level, InputPin, OutputPin};
use rppal::system::DeviceInfo;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const GPIO_SWITCH: u8 = 24;
const GPIO_LED: u8    = 25;

fn main() -> Result<(), Box<dyn Error>> {
	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();

	ctrlc::set_handler(move || {
		r.store(false, Ordering::SeqCst);
	}).expect("Error setting Ctrl-C handler");

	println!("Blinking an Switch Control LED {}", DeviceInfo::new()?.model());

	let switch = Gpio::new()?.get(GPIO_SWITCH)?.into_input();
	let mut led= Gpio::new()?.get(GPIO_LED)?.into_output();

	while running.load(Ordering::SeqCst) {
		thread::sleep(Duration::from_millis(10));
		pin_output_switch(&switch, &mut led);
	}

	led.set_low();
	Ok(())
}

fn pin_output_switch(input_pin: &InputPin, output_pin: &mut OutputPin) {
	if input_pin.read() == Level::High {
		output_pin.set_high();
	} else {
		output_pin.set_low();
	}
}
