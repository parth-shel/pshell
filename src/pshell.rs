extern crate libc;
extern crate std;

use std::process;
use std::process::Stdio;
use std::fs::OpenOptions;
use std::io;
use std::fs::File;
use std::io::prelude::*;

pub struct Command {
	pub simple_commands: Vec<SimpleCommand>,
	pub in_file: String,
	pub out_file: String,
	pub err_file: String,
	pub append: bool,
	pub background: bool,
	pub piped: bool,
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
			piped: false,
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

	let mut _in = as_raw_fd(std::io::stdin());
	let mut _out = as_raw_fd(std::io::stdout());
	let mut _err = as_raw_fd(std::io::stderr());
	child;

	/* setup input redirection */
	if !cmd_table.in_file.is_empty() {
		_in = OpenOptions::new().read(true).open(cmd_table.in_file);
	} else {
		_in = as_raw_fd(std::io::stdin);
	}

	/* for every simple command, fork a new process
		setup I/O redirection
		and call exec()
	*/
	for i in 0..cmd_table.simple_commands.len() {
		/* redirect input for each command */
		if i != 0 {
			_in = _out;
		}
		/* redirect output & err for the last command */
		if i == cmd_table.simple_commands.len() - 1 {
			/* open files and redirect I/O if specified */
			if !cmd_table.out_file.is_empty() {
				if cmd_table.append {
					_out = as_raw_fd(OpenOptions::new().write(true).append(true).create(true).open(cmd_table.out_file));
				} else {
					_out = as_raw_fd(OpenOptions::new().write(true).truncate(true).create(true).open(cmd_table.out_file));
				}
			} else {
				_out = as_raw_fd(std::io::stdout);
			}
			if !cmd_table.err_file.is_empty() {
				if cmd_table.append {
					_err = as_raw_fd(OpenOptions::new().write(true).append(true).create(true).open(cmd_table.err_file));
				} else {
					_err = as_raw_fd(OpenOptions::new().write(true).truncate(true).create(true).open(cmd_table.err_file));
				}
			} else {
				_err = as_raw_fd(std::io::stderr);
			}
		}
		/* check for built-ins */
		if built_in(cmd_table.simple_commands[i].args) {
			continue;
		}
		/* spawn new process for each command */ 
		else {
			let cmd = cmd_table.simple_commands[i].args[0];
			cmd_table.simple_commands[i].args.remove(0);
			child = process::Command::new(cmd)
                        .args(cmd_table.simple_commands[i].args)
                        .stdin(from_raw_fd(_in))
                        .stdout(from_raw_fd(_out))
                        .stderr(from_raw_fd(_err))
                        .spawn()
                        .expect("pshell failed to execute command");
		}
	}

	/* wait for child running in background */
	if !cmd_table.background {
		let ecode = child.wait().expect("pshell failed to wait on child");
		/* TODO: set environment variable for return code of process */
	} else {
		let pid = child.id();
		/* TODO: set environment variable for PID of backgrounded process */
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
