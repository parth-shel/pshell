/**
 * pshell - a simple *nix shell
 * @author: parth_shel
 * @version: v:0.1 - August 15, 2018
 **/

extern crate libc;
extern crate rustyline;
extern crate hostname;

use std::io;
use std::io::Write;
use std::env;
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod pshell;
mod parser;

fn main() {

	/* check target system */
	if !(cfg!(target_os = "linux") || cfg!(target_os = "unix")) {
		panic!("pshell system error: your OS isn't supported");
	}

	/* print shell startup message */
	println!("Hello, World!");

	/* RustyLine line editor */
	let mut rl = Editor::<()>::new();
	rl.load_history(".pshell_history");

	/* continually prompt the user, read input and execute command(s) */
	loop {
		/* display prompt */

		let istty = unsafe { libc::isatty(libc::STDOUT_FILENO as i32) } != 0;
		if istty {
			print_prompt();
		}

		/* read input */

		let readline = rl.readline("");
		match readline {
			Ok(line) => {
				/* if line is empty, move to the next input line */
				if line.len() == 0 {
					continue;
				}

			 	/* add input to history */

				rl.add_history_entry(line.as_ref());

				/* parse input line and divide into tokens */

				let tokens: Vec<String> = parser::tokenize_input(line);
				if tokens.is_empty() {
					continue;
				}
				// DEBUG
				println!("TOKENS:");
				for x in &tokens {
					println!("{}", x);
				}

				/* parse input and build command table */

				let cmd_table: pshell::Command = parser::parse_input(tokens);
				if cmd_table.simple_commands.is_empty() {
					continue;
				}
				// DEBUG
				println!("COMMAND TABLE:");
				println!("Simple Commands:");
				for _sc in &cmd_table.simple_commands {
					println!("Arguments:");
					for _arg in &_sc.args {
						println!("{}", _arg);
					}	
				}
				println!("Input File: {}", cmd_table.in_file);
				println!("Output File: {}", cmd_table.out_file);
				println!("Error File: {}", cmd_table.err_file);
				println!("Append: {:?}", cmd_table.append);
				println!("Background: {:?}", cmd_table.background);

				/* execute command(s) */

				pshell::exec(cmd_table);
			}
			/* ctrl-C */
			Err(ReadlineError::Interrupted) => {
				// continue;
				break; // DEBUG: this should be continue
			},
			/* exit gracefully on end-of-file */
			Err(ReadlineError::Eof) => {
				break;
			},
			Err(err) => {
				println!("pshell read line error: {:?}", err);
				break;
			}
		}
	}
	rl.save_history(".pshell_history").unwrap();
}

fn print_prompt() {
	let esc_char = vec![27];
	let esc = String::from_utf8(esc_char).unwrap();
	let reset: u8 = 0;
	let bright: u8 = 1;
	let black: u8 = 30;
	let red: u8 = 31;
	let green: u8 = 32;
	let yellow: u8 = 33;
	let blue: u8 = 34;
	let magenta: u8 = 35;
	let cyan: u8 = 36;

	let mut user_name = String::new();
	match env::var("USER") {
		Ok(val) => user_name = String::from(val),
			Err(err) => panic!("couldn't get env var! {}", err),
	}

	let mut host_name = hostname::get_hostname().unwrap();

	let curr_dir = env::current_dir().unwrap();

	println!("{}[{};{}m{}{}[{}m@{}[{};{}m{}{}[{}m{}[{};{}m | {}[{}m{}[{};{}m{}{}[{}m{}[{};{}m $pshell ↴ {}[{}m",
			esc, bright, green, user_name, esc, reset,
			esc, bright, blue, host_name,esc, reset,
			esc, bright, red, esc, reset,
			esc, bright, yellow, curr_dir.display(), esc, reset,
			esc, bright, cyan, esc, reset);
}
