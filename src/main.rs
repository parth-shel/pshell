/**
 * pshell - a simple *nix shell
 * @author: parth_shel
 * @version: v:0.1 - August 15, 2018
 **/

extern crate libc;
extern crate hostname;

use std::io;
use std::io::Write;
use std::env;

mod pshell;
mod lex;

fn main() {
	/* print shell startup message */
	println!("Hello, World!");

	/* continually prompt the user, read input and execute command(s) */
	loop {
		/* display prompt */

		let istty = unsafe { libc::isatty(libc::STDOUT_FILENO as i32) } != 0;
		if istty {
			print_prompt();
		}

		/* read input */

		let mut input = String::new();
		io::stdin().read_line(&mut input)
			.expect("failed to read line");

		/* exit gracefully on end-of-file */
		if input.len() == 0 {
			break;
		}

		/* if line contains only NEWLINE, go to next input line */
		if input.len() <= 1 {
			continue;
		}

		/* parse input line and divide into tokens */

		let tokens: Vec<String> = lex::parse(input);

		/* parse input and build command table */

		let cmd_table: pshell::Command = pshell::parse(tokens);

		/* execute command(s) */

		pshell::exec(cmd_table);
	}
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
	
	print!("{}[{};{}m{}{}[{}m", esc, bright, green, user_name, esc, reset);
	print!("@");
	print!("{}[{};{}m{}{}[{}m", esc, bright, blue, host_name, esc, reset);
	print!("{}[{};{}m|{}[{}m", esc, bright, red, esc, reset);
	print!("{}[{};{}m{}{}[{}m ", esc, bright, yellow, curr_dir.display(), esc, reset);
	print!("{}[{};{}m $pshell> {}[{}m", esc, bright, cyan, esc, reset);

	io::stdout().flush().unwrap();
}
