use crate::errors::Error;
use clap::{Arg, ArgMatches, Command};
pub struct Cli;

impl Cli {
    pub fn matches() -> ArgMatches {
        Command::new("PortForm")
            .version("1.0")
            .author("Joshua Newell Diehl <jdiehl2236@gmail.com>")
            .about("PDF Resume Builder")
            .subcommand(Subcommands::set())
            .arg_required_else_help(true)
            .get_matches()
    }
}

struct Subcommands;

impl Subcommands {
    pub fn set() -> Command {
        let set_header = move |args| Command::new("header").args(args);
        let set_title = move |args| Command::new("title").args(args);

        Command::new("set")
            .subcommand(set_header(Arguments::header()))
            .subcommand(set_title(Arguments::title()))
    }
}

struct Arguments;

impl Arguments {
    pub fn title() -> [Arg; 1] {
        [Arg::new("title").short('t').required(false)]
    }

    pub fn header() -> [Arg; 2] {
        [
            Arg::new("name").short('n').long("name").required(true),
            Arg::new("profession")
                .short('p')
                .long("profession")
                .required(true),
        ]
    }
}

pub struct CLParser;

impl CLParser {
    pub fn handle_input_arguments() -> Result<(), Error> {
        let matches = Cli::matches();

        match matches.subcommand() {
            Some(("set", matches)) => Self::handle_set_command(matches),
            Some((unknown, _)) => eprintln!("Subcommand {:#?} not recognized.", unknown),
            None => eprintln!("No matches found for subcommand..."),
        };

        Ok(())
    }

    fn handle_set_command(matches: &ArgMatches) {
        match matches.subcommand() {
            Some(("header", args)) => println!("{:#?}", args.get_one::<String>("name")),
            Some(("title", args)) => println!("{:#?}", args),
            Some((unknown, _)) => eprintln!("Subcommand {:#?} not recognized.", unknown),
            None => eprintln!("No matches found for subcommand..."),
        };
    }
}
