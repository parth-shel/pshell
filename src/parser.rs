extern crate commands;

use pshell::Command;
use pshell::SimpleCommand;
use parser::commands::tokenizer::{tokenize, TokenType};

/* tokenize - tokenize the input string based on whitespace chars and handle " & \ */
pub fn tokenize_input(input: String) -> Vec<String> {
	let mut tokens:Vec<String> = Vec::new();;
	if let Ok(vec) = tokenize(&input) {
		for x in &vec {
			if x.token_type == TokenType::Word {
				let mut to_push = String::from(x.text);
				if to_push.is_empty() {
					continue;
				}
				if to_push.starts_with("\"") && to_push.ends_with("\"") {
					to_push.remove(0);
					to_push.pop();
				}
				tokens.push(to_push);
			}
		}
	} else {
		println!("pshell tokenize error");
	}
	return tokens;
}

/* parse - build command table from the tokens */
pub fn parse_input(tokens: Vec<String>) -> Command {
	let mut cmd_table:Command = Command::new();
	return cmd_table;
}
