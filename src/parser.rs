extern crate commands;

use pshell::ComplexCommand;
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
		let mut empty_tokens:Vec<String> = Vec::new();
		return empty_tokens;
	}
	return tokens;
}

/* parse - build command table from the tokens */
pub fn parse_input(mut tokens: Vec<String>) -> ComplexCommand {
	let mut cmd_table:ComplexCommand = ComplexCommand::new();

	/* if last token is '&', set background */
	if tokens.len() > 0 && tokens[tokens.len() - 1] == "&" {
		cmd_table.background = true;
		tokens.pop();
	}

	/* check for I/O redirection (default is none) */
	let mut i_redirect_found: bool = false;
	let mut o_redirect_found: bool = false;
	let mut e_redirect_found: bool = false;

	for _i in 0..3 {
		/* check last-but-1 token */
		if tokens.len() >= 3 && (match tokens[tokens.len() - 2].trim() {
			"<" => true,
			"2>" => true,
			"1>" => true,
			">" => true,
			">&" => true,
			">>" => true,
			">>&" => true,
			_ => false,
			}) {
				match tokens[tokens.len() - 2].trim() {
					"<" => { // input redirection
						if i_redirect_found {
							println!("pshell parse error");
							let mut empty_cmd_table:ComplexCommand = ComplexCommand::new();
							return empty_cmd_table;
						}
						i_redirect_found = true;
						cmd_table.in_file = tokens[tokens.len() - 1].clone();
					}, "2>" => { // error redirection
						if e_redirect_found {
							println!("pshell parse error");
							let mut empty_cmd_table:ComplexCommand = ComplexCommand::new();
							return empty_cmd_table;
						}
						e_redirect_found = true;
						cmd_table.err_file = tokens[tokens.len() - 1].clone();
					}, "1>" => { // output redirection
						if o_redirect_found {
							println!("pshell parse error");
							let mut empty_cmd_table:ComplexCommand = ComplexCommand::new();
							return empty_cmd_table;
						}
						o_redirect_found = true;
						cmd_table.out_file = tokens[tokens.len() - 1].clone();
					}, ">" => { // output redirection
						if o_redirect_found {
							println!("pshell parse error");
							let mut empty_cmd_table:ComplexCommand = ComplexCommand::new();
							return empty_cmd_table;
						}
						o_redirect_found = true;
						cmd_table.out_file = tokens[tokens.len() - 1].clone();
					}, ">&" => { // output & error redirection
						if o_redirect_found || e_redirect_found {
							println!("pshell parse error");
							let mut empty_cmd_table:ComplexCommand = ComplexCommand::new();
							return empty_cmd_table;
						}
						o_redirect_found = true;
						e_redirect_found = true;
						cmd_table.out_file = tokens[tokens.len() - 1].clone();
						cmd_table.err_file = tokens[tokens.len() - 1].clone();
					}, ">>" => { // output redirection (append mode)
						if o_redirect_found {
							println!("pshell parse error");
							let mut empty_cmd_table:ComplexCommand = ComplexCommand::new();
							return empty_cmd_table;
						}
						o_redirect_found = true;
						cmd_table.out_file = tokens[tokens.len() - 1].clone();
						cmd_table.append = true;
					}, ">>&" => { // output & error redirection (append mode)
						if o_redirect_found || e_redirect_found {
							println!("pshell parse error");
							let mut empty_cmd_table:ComplexCommand = ComplexCommand::new();
							return empty_cmd_table;
						}
						o_redirect_found = true;
						e_redirect_found = true;
						cmd_table.out_file = tokens[tokens.len() - 1].clone();
						cmd_table.err_file = tokens[tokens.len() - 1].clone();
						cmd_table.append = true;
					}, _ => {

					},
				}
			tokens.pop();
			tokens.pop();
		}
	}
	
	/* iterate over simple commands and build command table */
	let mut simple_command:SimpleCommand = SimpleCommand::new();
	for x in &tokens {
		if x.trim() == "|" {
			if cmd_table.logical {
				println!("pshell parse error");
				let mut empty_cmd_table:ComplexCommand = ComplexCommand::new();
				return empty_cmd_table;
			}
			cmd_table.piped = true;
			cmd_table.simple_commands.push(simple_command);
			simple_command = SimpleCommand::new();
			continue;
		} else if x.trim() == ";" || x.trim() == "&&" || x.trim() == "||" {
			if cmd_table.piped {
				println!("pshell parse error");
				let mut empty_cmd_table:ComplexCommand = ComplexCommand::new();
				return empty_cmd_table;
			}
			cmd_table.logical = true;
			simple_command.args.push(x.to_string());
			cmd_table.simple_commands.push(simple_command);
			simple_command = SimpleCommand::new();
		}
		simple_command.args.push(x.to_string());
	}
	if simple_command.args.len() > 0{
		cmd_table.simple_commands.push(simple_command);
	}
	
	/* TODO: expand environment variables and '~' */

	/* TODO: implement wildcards '*' */

	/* TODO: handle subshell '`...`' */

	return cmd_table;
}
