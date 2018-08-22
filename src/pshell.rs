extern crate libc;

use std::mem;
use std::process;
use std::ptr;

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

	/* for every simple command, fork a new process
		setup I/O redirection
		and call exec()
	*/

	/* store initial states of stdin, stdout & stderr */

	/* set stdin, stdout & stderr to defaults */

	/* setup input redirection */

	/* for and execute all commands */
		
		/* redirect input for each command */

		/* redirect output & err for the last command */

			/* open files and redirect I/O if specified */

		/* piping for the rest of the commands */

		/* redirect output & err for each command */

		/* check for built-ins */

		/* spawn new process for each command */

			/* child process */
			/* exec */
			/* exec() error */

			/* fork() error */

		/* parent process */

	/* wait for child running in background */

		/* TODO: set environment variable for return code of process */

		/* TODO: set environment variable for PID of backgrounded process */
	
	/* restore stdin, stdout, stderr */

	/* close opened file descriptors to avoid descriptor leaks */

	return;
}

/* TODO: execute built-in features */
fn built_in(args: Vec<String>) -> bool {
	match args[0].trim() {
		"exit" => {
			/* print shell goodbye message */
			println!("I'll miss you :'(");

			process::exit(0);

			return true;
		}, "quit" => {
			/* print shell goodbye message */
			println!("I'll miss you :'(");

			process::exit(0);

			return true;
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
