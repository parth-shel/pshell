pub fn parse(tokens: Vec<String>) -> Command {
	let mut cmd_table:Command = Command::new();
	return cmd_table;
}

pub fn exec(cmd_table: Command) {
	return;
}

pub struct Command {
	pub simple_commands: Vec<SimpleCommand>,
	pub out_file: String,
	pub in_file: String,
	pub err_file: String,
	background: bool,
	append: bool,
}

impl Command {
	pub fn new() -> Command {
		Command {
			simple_commands: Vec::new(),
			out_file: String::new(),
			in_file: String::new(),
			err_file: String::new(),
			background: false,
			append: false,
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
