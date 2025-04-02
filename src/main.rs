use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput};

use std::process::Command;

fn main() {
//    let mut cmd = Command::new("loginctl");
//    let fullcmd = cmd.arg("unlock-session");
//    let val = fullcmd.spawn().expect("loginctl failed to start");
    match run() {
         Ok(_) => (),
         Err(err) => println!("Error: {}", err),
    }
//    println!("{:?}", val);
}

fn run() -> Result<(), Box<dyn Error>> {

	let mut input = String::new();

	let mut midi_in = MidiInput::new("midir reading input")?;
	midi_in.ignore(Ignore::None);

	//Get an input port (read from console if multiple are available)
	let in_ports = midi_in.ports();
	let in_port = match in_ports.len() {
		0 => return Err("No input port found".into()),
		1 => {
			println!(
				"Choosing the only available input port: {}",
				midi_in.port_name(&in_ports[0]).unwrap()
			);
			&in_ports[0]
		}
		_ => {
			println!("\nAvailable input ports:");
			for (i, p) in in_ports.iter().enumerate() {
				println!("{}: {}", i, midi_in.port_name(p).unwrap());
			}
			print!("Please select input port: ");
			stdout().flush()?;
			let mut input = String::new();
			stdin().read_line(&mut input)?;
			in_ports
				.get(input.trim().parse::<usize>()?)
				.ok_or("invalid input port selected")?
		}
	};

	println!("\nOpening connection");
	let in_port_name = midi_in.port_name(in_port)?;

	let chord_lock: Vec<u8> = vec![60, 64, 67];
	let mut c: bool = false;
	let mut e: bool = false;
	let mut g: bool = false;
	let mut v: Vec<u8> = Vec::new();
	// _conn_in needs to be a named parameter because it needs to be kept alive until the end of the scope
	let _conn_in = midi_in.connect(
		in_port,
		"midir-read-input",
		move |stamp, message, _| {
    			let mut cmd = Command::new("loginctl");
			let fullcmd = cmd.arg("unlock-session");

			println!("{}: {:?} (len = {})", stamp, message, message.len());
			v.push(message[1]);
			for i in &chord_lock {
				if message[1] == *i && *i == 60 {
					c = true;
				}
				if message[1] == *i && *i == 64 {
					e = true;
				}
				if message[1] == *i && *i == 67 {
					g = true;
				}
			}
			if v.len() >= 3 {
				//println!("Third note pressed!");
				let mut v: Vec<u8> = Vec::new();
			}
			if c == true && e == true && g == true {
				println!("Chord lock unlocked!");
				let val = fullcmd.spawn().expect("loginctl failed to start");
				c = false;
				e = false;
				g = false;
				println!("Relocking...");
			}
		},
		(),
	)?;
	println!(
		"Connection open, reading input from '{}' (press enter to exit) ...",
		in_port_name
	);

	input.clear();
	stdin().read_line(&mut input)?; //wait for next enter key press

	println!("Closing connection");
	Ok(())
}
