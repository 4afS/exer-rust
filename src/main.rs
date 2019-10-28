extern crate clap;

use clap::{App, SubCommand};

mod run;

enum Command {
    Run,
}

fn to_command(command_str: &str) -> Option<Command> {
    match command_str {
        "run" => Some(Command::Run),
        _ => None,
    }
}

fn main() {
    let app = App::new("exer")
        .version("0.1.0")
        .author("4afS")
        .about("A run tool that identifies the used build tool.")
        .subcommand(SubCommand::with_name("run").about("Run any project"));

    match app
        .get_matches()
        .subcommand_name()
        .and_then(|command_str| to_command(command_str))
    {
        Some(Command::Run) => run::run(),
        None => {
            println!("exer: try 'exer --help' for more information");
        }
    }
}
