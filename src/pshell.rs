extern crate libc;

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
		_in = unsafe { libc::open(cmd_table.in_file.as_ptr()  as *const _, libc::O_RDONLY) };
	} else {
		_in = unsafe { libc::dup(initial_in) };
	}

	let mut ret;

	/* for and execute all commands */
	for _sc in &cmd_table.simple_commands {
		/* redirect input for each command */
		unsafe { libc::dup2(_in, 0) };
		unsafe { libc::close(_in) };

		/* redirect output & err for the last command */

		/* piping for the rest of the commands */

		/* redirect output & err for each command */

		/* check for built-ins */

		/* spawn new process for each command */
		ret = unsafe { libc::fork() };
		if ret == 0 {
			/* child process */
			/* exec */
			unsafe { libc::execvp() };
			panic!("pshell exec error");
		} else if ret < 0 {
			/* fork() error */
			panic!("pshell fork error");
		}
		/* parent process */
	}

	/* wait for child running in background */

	/* restore stdin, stdout, stderr */

	/* close opened file descriptors to avoid descriptor leaks */

	return;
}
