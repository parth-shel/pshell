extern crate libc;
extern crate std;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::{Stdio,Command,Child,Output,exit};
use std::os::unix::io::{FromRawFd, AsRawFd};
use std::os::unix::process::CommandExt;
use nix::sys::signal;
use nix::unistd::pipe;
use std::env;
use std::path::Path;
// use libc;

extern "C" fn handle_sigchld(_: i32) {
    unsafe {
        let mut stat: i32 = 0;
        let ptr: *mut i32 = &mut stat;
        let pid = libc::waitpid(-1, ptr, libc::WNOHANG);
    }
    
}

pub struct ComplexCommand {
	pub simple_commands: Vec<SimpleCommand>,
	pub in_file: String,
	pub out_file: String,
	pub err_file: String,
	pub append: bool,
	pub background: bool,
	pub piped: bool,
	pub logical: bool,
}

impl ComplexCommand {
	pub fn new() -> ComplexCommand {
		ComplexCommand {
			simple_commands: Vec::new(),
			in_file: String::new(),
			out_file: String::new(),
			err_file: String::new(),
			append: false,
			background: false,
			piped: false,
			logical: false,
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

pub fn exec(cmd_table: ComplexCommand) {
	/* don't do anything if there are no simple commands */
	if cmd_table.simple_commands.len() == 0 {
		return;
	}

	/* if the command has logical short circuiting */
	if cmd_table.logical {
		let mut ecode = 0;
		let mut sep = String::new();

		for x in cmd_table.simple_commands {
			if x.args[0] == ";" || x.args[0] == "&&" || x.args[0] == "||" {
				sep = x.args[0].clone();
				continue;
			}
			if sep == "&&" && ecode != 0 {
				break;
			}
			if sep == "||" && ecode == 0 {
				break;
			}
			// ecode = run_proc(x.args, cmd_table);
		}
	}
	/* if the command has piping */ 
	else if cmd_table.piped {
		run_piped(cmd_table);
	}
	/* it's a simple command */ 
	else {
		run(cmd_table);
	}
	
	return;
}

fn run(cmd_table: ComplexCommand) {
	let sig_action = signal::SigAction::new(
        	signal::SigHandler::Handler(handle_sigchld),
        	signal::SaFlags::empty(),
        	signal::SigSet::empty(),
    	);
	unsafe {
        match signal::sigaction(signal::SIGCHLD, &sig_action) {
            Ok(_) => {}
            Err(e) => println!("sigaction error: {:?}", e),
        }
    }
	for i in 0..cmd_table.simple_commands.len() {
		/* check for built-ins */
		if built_in(cmd_table.simple_commands[i].args.clone()) {
			continue;
		}

		let mut _in = 0;
		let mut _out = 1;
		let mut _err = 2;

		/* redirect I/O as specified */
		if !cmd_table.in_file.is_empty() {
			_in = OpenOptions::new().read(true).open(cmd_table.in_file.clone()).unwrap().as_raw_fd();
		} else {
			_in = std::io::stdin().as_raw_fd();
		}
		if !cmd_table.out_file.is_empty() {
			if cmd_table.append {
				_out = OpenOptions::new().write(true).append(true).create(true).open(cmd_table.out_file.clone()).unwrap().as_raw_fd();
			} else {
				_out = OpenOptions::new().write(true).truncate(true).create(true).open(cmd_table.out_file.clone()).unwrap().as_raw_fd();
			}
		} else {
			_out = std::io::stdout().as_raw_fd();
		}
		if !cmd_table.err_file.is_empty() {
			if cmd_table.append {
				_err = OpenOptions::new().write(true).append(true).create(true).open(cmd_table.err_file.clone()).unwrap().as_raw_fd();
			} else {
				_err = OpenOptions::new().write(true).truncate(true).create(true).open(cmd_table.err_file.clone()).unwrap().as_raw_fd();
			}
		} else {
			_err = std::io::stderr().as_raw_fd();
		}

		/* spawn new process for each command */
		let mut child = Command::new(&cmd_table.simple_commands[i].args[0])
									.args(&cmd_table.simple_commands[i].args[1.. ])
									.stdin(unsafe { Stdio::from_raw_fd(_in) })
									.stdout(unsafe { Stdio::from_raw_fd(_out) })
									.stderr(unsafe { Stdio::from_raw_fd(_err) })
									.spawn().expect("pshell failed to execute command");

		/* wait for child running in background */
		if !cmd_table.background {
			let ecode = child.wait().expect("pshell failed to wait on child");
			/* TODO: set environment variable for return code of process */
		} else {
			let pid = child.id();
			/* TODO: set environment variable for PID of backgrounded process */
		}
	}
	return;
}

fn run_proc(args: Vec<String>, cmd_table: ComplexCommand) -> i32 {
	return 0;
}

fn run_piped(cmd_table: ComplexCommand) {
	return;
}

/* TODO: execute built-in features */
fn built_in(args: Vec<String>) -> bool {
	match args[0].trim() {
		"exit" => {
			/* print shell goodbye message */
			println!("I'll miss you :'(");

			exit(0);

			// return true;
		}, "quit" => {
			/* print shell goodbye message */
			println!("I'll miss you :'(");

			exit(0);

			// return true;
		}, "cd" => {
			if args.len() == 1 {
				match env::home_dir() {
					Some(path) => {
						match env::set_current_dir(path) {
							Ok(_) => return true,
							Err(err) => println!("pshell cd error: {:?}", err),
						}
					},
					None => println!("pshell failed to get home dir"),
				}
			} else {
				let dir = Path::new(&args[1]);
				match env::set_current_dir(dir) {
					Ok(_) => return true,
					Err(err) => println!("pshell cd error: {:?}", err),
				}
			}
			return true;
		}, "setenv" => {
			if args.len() >= 3 {
				let key = &args[1];
				let value = &args[2];
				env::set_var(key, value);
			}
			return true;
		}, "unsetenv" => {
			if args.len() >= 2 {
				let key = &args[1];
				env::remove_var(key);
			}
			return true;
		}, "printenv" => {
			if args.len() >= 2 {
				let key = &args[1];
				match env::var(key) {
				    Ok(value) => println!("{}: {:?}", key, value),
				    Err(e) => println!("couldn't interpret {}: {}", key, e),
				}
			} else {
				for (key, value) in env::vars() {
    				println!("{}: {:?}", key, value);
				}
			}
			return true;
		}, "source" => {
			/* TODO */
			return true;
		}, _ => {
			return false;
		}
	}
}
