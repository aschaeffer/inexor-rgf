use std::string::ToString;
use std::sync::Arc;

use clap::Parser;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Cmd;
use rustyline::Editor;
use rustyline::Event;
use rustyline::KeyCode;
use rustyline::KeyEvent;
use rustyline::Modifiers;
use shellwords::split;

use crate::cli::handler::handle_command;
use crate::cli::repl::args::ReplArgs;
use crate::cli::repl::chars::*;
use crate::cli::repl::repl_helper::ReplHelper;
use crate::cli::repl::return_state::ReturnState;
use crate::client::InexorRgfClient;

pub(crate) mod args;
pub(crate) mod chars;
pub(crate) mod hint;
pub(crate) mod repl_helper;
pub(crate) mod return_state;

pub(crate) async fn repl(client: &Arc<InexorRgfClient>) -> Result<(), i32> {
    let mut rl = Editor::<ReplHelper, DefaultHistory>::new().map_err(|_| 255)?;
    rl.set_helper(Some(ReplHelper::new()));
    rl.bind_sequence(Event::KeySeq(vec![KeyEvent(KeyCode::Tab, Modifiers::NONE)]), Cmd::CompleteHint);

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut return_state = ReturnState::NEUTRAL;
    let mut break_state = false;
    loop {
        let prompt = format!("{} {} {} ", CHAR_PROMPT, client.remote().url().cyan().bold(), return_state.to_string());
        let readline = rl.readline(prompt.as_str());
        match readline {
            Ok(line) => {
                match line.as_str() {
                    "exit" | "quit" => break,
                    _ => match split(line.as_str())
                        .map(|mut args| {
                            args.insert(0, String::from(" "));
                            args
                        })
                        .map(ReplArgs::try_parse_from)
                    {
                        Ok(Ok(cli_args)) => {
                            let command = cli_args.commands;
                            match handle_command(&client, command).await {
                                Ok(response) => {
                                    println!("{}", response);
                                    // input was executed (successful or not)
                                    return_state = ReturnState::SUCCESS;
                                    break_state = false;
                                }
                                Err(e) => {
                                    eprintln!("{}: {}", "Command failed with error".red(), e);
                                    // input errored
                                    return_state = ReturnState::ERROR;
                                }
                            }
                        }
                        Ok(Err(e)) => {
                            eprintln!("{}", e);
                            return_state = ReturnState::ERROR;
                        }
                        Err(r) => {
                            eprintln!("{}: {}", "Mismatched Quotes".red(), r);
                            return_state = ReturnState::ERROR;
                        }
                    },
                }
                let _ = rl.add_history_entry(line.as_str());
            }
            // CTRL-C
            Err(ReadlineError::Interrupted) => {
                if break_state {
                    break;
                }
                break_state = true;
                println!("Press CTRL-C again to exit");
            }
            // CTRL-D
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }
    let _ = rl.save_history("history.txt");
    Ok(())
}

pub fn longest_common_prefix(s: &Vec<&String>) -> String {
    let mut result = "".to_string();
    let mut count = 0;
    let mut found = false;
    if s.len() == 0 || s[0].len() == 0 {
        return result;
    }
    loop {
        result.push_str(&s[0][count..count + 1]);
        for i in 0..s.len() {
            if s[i].len() < count + 1 || s[i][0..count + 1] != result {
                found = true;
                break;
            }
        }
        match found {
            true => break result[0..count].to_string(),
            false => {
                if count + 1 == s[0].len() {
                    break result;
                }
            }
        }
        count += 1;
    }
}
