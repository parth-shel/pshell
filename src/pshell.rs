extern crate libc;

use std::process;

pub struct Command {
	pub simple_commands: Vec<SimpleCommand>,
	pub in_file: String,
	pub out_file: String,
	pub err_file: String,
	pub append: bool,
	pub background: bool,
}

impl Command {
	pub fn new() -> Command {
		Command {
			simple_commands: Vec::new(),
			in_file: String::new(),
			out_file: String::new(),
			err_file: String::new(),
			append: false,
			background: false,
		}
	}
}

pub struct SimpleCommand {
	pub args: Vec<String>,
}

impl SimpleCommand {
	pub fn new() -> SimpleCommand {
		SimpleCommand {
			args: Vec::new(),
		}
	}
}

pub fn exec(cmd_table: Command) {
	/* don't do anything if there are no simple commands */
	if cmd_table.simple_commands.len() == 0 {
		return;
	}



	return;
}

/* TODO: execute built-in features */
fn built_in(args: Vec<String>) -> bool {
	match args[0].trim() {
		"exit" => {
			/* print shell goodbye message */
			println!("I'll miss you :'(");

			process::exit(0);

			// return true;
		}, "quit" => {
			/* print shell goodbye message */
			println!("I'll miss you :'(");

			process::exit(0);

			// return true;
		}, "cd" => {
			/* TODO */
			return true;
		}, "setenv" => {
			/* TODO */
			return true;
		}, "unsetenv" => {
			/* TODO */
			return true;
		}, "printenv" => {
			/* TODO */
			return true;
		}, "source" => {
			/* TODO */
			return true;
		}, _ => {
			return false;
		}
	}
}
