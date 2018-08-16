extern crate commands;

use pshell::Command;
use pshell::SimpleCommand;
use parser::commands::tokenizer::{tokenize, TokenType};

/* tokenize - tokenize the input string based on whitespace chars and handle " & \ */
pub fn tokenize_input(input: String) -> Vec<String> {
	let mut tokens:Vec<String> = Vec::new();
	/* tokenize */
	if let Ok(vec) = tokenize(&input) {
		for x in &vec {
			/* ignore all whitespace chars */
			if x.token_type == TokenType::Word {
				let mut to_push = String::from(x.text.trim());
				/* ignore empty tokens */
				if to_push.is_empty() {
					continue;
				}
				/* remove '"' from words */
				if to_push.starts_with("\"") && to_push.ends_with("\"") {
					to_push.remove(0);
					to_push.pop();
				}
				/* TODO: remove '\' after the escaped '"' */
				tokens.push(to_push);
			}
		}
	} else {
		println!("pshell tokenize error");
	}
	return tokens;
}

/* parse - build command table from the tokens */
pub fn parse_input(mut tokens: Vec<String>) -> Command {
	let mut cmd_table:Command = Command::new();
	
	/* if last token is '&', set background */
	if tokens.len() > 0 && tokens[tokens.len() - 1] == "&" {
		cmd_table.background = true;
		tokens.pop();
	}

	/* check for I/O redirection (default is none) */
	let mut oe_redirect_found: bool = false;
	let mut i_redirect_found: bool = false;
	
	/* check last-but-3 token */
	if !(i_redirect_found) && (tokens.len() > 5 && tokens[tokens.len() - 4] == "<") {
		i_redirect_found = true;
		cmd_table.in_file = tokens[tokens.len() - 3].clone();
		let len = tokens.len();
		tokens.remove(len - 4);
		tokens.remove(len - 3);
	}
	if !(oe_redirect_found) && (tokens.len() > 5 && match tokens[tokens.len() - 4].trim() {
		"2>" => true,
		"1>" => true,
		">" => true,
		">&" => true,
		">>" => true,
		">>&" => true,
		_ => false,
		}) {
			oe_redirect_found = true;
			match tokens[tokens.len() - 4].trim() {
				"2>" => { // error redirection
					cmd_table.err_file = tokens[tokens.len() - 3].clone();
				}, "1>" => { // output redirection
					cmd_table.out_file = tokens[tokens.len() - 3].clone();
				}, ">" => { // output redirection
					cmd_table.out_file = tokens[tokens.len() - 3].clone();
				}, ">&" => { // output & error redirection
					cmd_table.out_file = tokens[tokens.len() - 3].clone();
					cmd_table.err_file = tokens[tokens.len() - 3].clone();
				}, ">>" => { // output redirection (append mode)
					cmd_table.out_file = tokens[tokens.len() - 1].clone();
					cmd_table.append = true;
				}, ">>&" => { // output & error redirection (append mode)
					cmd_table.out_file = tokens[tokens.len() - 3].clone();
					cmd_table.err_file = tokens[tokens.len() - 3].clone();
					cmd_table.append = true;
				}, _ => {

				},
			}
			let len = tokens.len();
			tokens.remove(len - 4);
			tokens.remove(len - 3);
	}

	/* check last-but-1 token */
	if !(i_redirect_found) && (tokens.len() > 3 && tokens[tokens.len() - 2] == "<") {
		i_redirect_found = true;
		cmd_table.in_file = tokens[tokens.len() - 1].clone();
		tokens.pop();
		tokens.pop();
	}
	if !(oe_redirect_found) && (tokens.len() > 3 && match tokens[tokens.len() - 2].trim() {
		"2>" => true,
		"1>" => true,
		">" => true,
		">&" => true,
		">>" => true,
		">>&" => true,
		_ => false,
		}) {
			oe_redirect_found = true;
			match tokens[tokens.len() - 2].trim() {
				"2>" => { // error redirection
					cmd_table.err_file = tokens[tokens.len() - 1].clone();
				}, "1>" => { // output redirection
					cmd_table.out_file = tokens[tokens.len() - 1].clone();
				}, ">" => { // output redirection
					cmd_table.out_file = tokens[tokens.len() - 1].clone();
				}, ">&" => { // output & error redirection
					cmd_table.out_file = tokens[tokens.len() - 1].clone();
					cmd_table.err_file = tokens[tokens.len() - 1].clone();
				}, ">>" => { // output redirection (append mode)
					cmd_table.out_file = tokens[tokens.len() - 1].clone();
					cmd_table.append = true;
				}, ">>&" => { // output & error redirection (append mode)
					cmd_table.out_file = tokens[tokens.len() - 1].clone();
					cmd_table.err_file = tokens[tokens.len() - 1].clone();
					cmd_table.append = true;
				}, _ => {

				},
			}
		tokens.pop();
		tokens.pop();
	}

	/* iterate over simple commands and build command table */
	while let Some(curr) = tokens.pop() {
		if curr.trim() == "|" || tokens.len() == 0 {
			
		}
	}

	return cmd_table;
}
