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
	return;
}
