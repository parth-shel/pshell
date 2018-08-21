extern crate libc;

use std::mem;
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

	/* for every simple command, fork a new process
		setup I/O redirection
		and call exec()
	*/

	/* store initial states of stdin, stdout & stderr */
	let initial_in = unsafe { libc::dup(0) };
	let initial_out = unsafe { libc::dup(1) };
	let initial_err = unsafe { libc::dup(2) };

	/* set stdin, stdout & stderr to defaults */
	let mut _in = 0;
	let mut _out = 1;
	let mut _err = 2;

	/* setup input redirection */
	if !cmd_table.in_file.is_empty() {
		_in = unsafe { libc::open(cmd_table.in_file.as_ptr() as *const _, libc::O_RDONLY) };
	} else {
		_in = unsafe { libc::dup(initial_in) };
	}

	let mut ret = 0;

	/* for and execute all commands */
	for i in 0..cmd_table.simple_commands.len() {
		/* redirect input for each command */
		unsafe { libc::dup2(_in, 0) };
		unsafe { libc::close(_in) };

		/* redirect output & err for the last command */
		if i == cmd_table.simple_commands.len() - 1 {
			/* open files and redirect I/O if specified */
			if !cmd_table.out_file.is_empty() {
				if cmd_table.append {
					_out = unsafe { libc::open(cmd_table.out_file.as_ptr() as *const _,
						libc::O_WRONLY|libc::O_CREAT|libc::O_APPEND, 0600) };
				} else {
					_out = unsafe { libc::open(cmd_table.out_file.as_ptr() as *const _,
						libc::O_WRONLY|libc::O_CREAT|libc::O_TRUNC, 0600) };
				}
			} else {
				_out = unsafe { libc::dup(initial_out) };
			}
			if !cmd_table.err_file.is_empty() {
				if cmd_table.append {
					_err = unsafe { libc::open(cmd_table.err_file.as_ptr() as *const _,
						libc::O_WRONLY|libc::O_CREAT|libc::O_APPEND, 0600) };
				} else {
					_err = unsafe { libc::open(cmd_table.err_file.as_ptr() as *const _,
						libc::O_WRONLY|libc::O_CREAT|libc::O_TRUNC, 0600) };
				}
			} else {
				_err = unsafe { libc::dup(initial_err) };
			}
		}
		/* piping for the rest of the commands */
		else {
			let mut pipefd = [0; 2];
			let _fd: i64 = unsafe { mem::transmute(pipefd) };
			if unsafe { libc::pipe(_fd as *mut i32) } < 0 {
				panic!("pshell pipe error");
			} else {
				_in = pipefd[0];
				_out = pipefd[1];
			}
		}

		/* redirect output & err for each command */
		unsafe { libc::dup2(_out, 1) };
		unsafe { libc::close(_out) };
		unsafe { libc::dup2(_err, 2) };
		unsafe { libc::close(_err) };

		/* check for built-ins */
		if built_in(cmd_table.simple_commands[i].args.clone()) {
			continue;
		}

		/* spawn new process for each command */
		ret = unsafe { libc::fork() };
		if ret == 0 {
			/* child process */
			/* exec */
			unsafe { libc::execvp(cmd_table.simple_commands[i].args[0].as_ptr() as *const _,
				cmd_table.simple_commands[i].args.as_ptr() as *const *const _) };
			/* exec() error */
			panic!("pshell exec error");
		} else if ret < 0 {
			/* fork() error */
			panic!("pshell fork error");
		}
		/* parent process */
	}

	/* wait for child running in background */
	if !cmd_table.background {
		let mut stat_val: i32 = 0;
		unsafe {libc::waitpid(ret, &mut stat_val as *mut i32, 0) };
		/* TODO: set environment variable for return code of process */
		/*if unsafe { libc::WIFEXITED(stat_val) } > 0 {

		}*/
	} else {
		/* TODO: set environment variable for PID of backgrounded process */
	}

	/* restore stdin, stdout, stderr */
	unsafe { libc::dup2(initial_in, 0) };
	unsafe { libc::dup2(initial_out, 0) };
	unsafe { libc::dup2(initial_err, 0) };

	/* close opened file descriptors to avoid descriptor leaks */
	unsafe { libc::close(initial_in) };
	unsafe { libc::close(initial_out) };
	unsafe { libc::close(initial_err) };

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
