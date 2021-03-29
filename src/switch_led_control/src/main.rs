use std::error::Error;

use rppal::gpio::{Gpio, Trigger};
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

	let mut switch = Gpio::new()?.get(GPIO_SWITCH)?.into_input();
	let mut led= Gpio::new()?.get(GPIO_LED)?.into_output();

	let r = switch.set_interrupt(Trigger::FallingEdge);
	match r {
		Ok(n) => n,
		Err(e) => println!("Error: {:?}", e),
	}

	while running.load(Ordering::SeqCst) {
		let level = switch.poll_interrupt(true, None);
		match level {
			Ok(_) => led.toggle(),
			Err(e) => {
				println!("Error: {:?}", e);
				break;
			}
		}
	}

	led.set_low();
	let r = switch.clear_interrupt();
	match r {
		Ok(n) => n,
		Err(e) => println!("Error: {:?}", e),
	}
	Ok(())
}
