use super::errors::Error;
use super::generate::ResumeGenerator;

use clap::{Arg, ArgMatches, Command};
use std::fs::File;
use std::io::BufWriter;
pub struct Cli;

impl Cli {
    pub fn matches() -> ArgMatches {
        Command::new("PortForm")
            .version("1.0")
            .author("Joshua Newell Diehl <jdiehl2236@gmail.com>")
            .about("PDF Resume Builder")
            .subcommand(Subcommands::set())
            .subcommand(Subcommands::write())
            .arg_required_else_help(true)
            .get_matches()
    }
}

struct Subcommands;

trait Operator {
    fn set() -> Command;
    fn write() -> Command;
}

impl Operator for Subcommands {
    fn set() -> Command {
        let set_header = move |args| Command::new("header").args(args);
        let set_title = move |args| Command::new("title").args(args);

        Command::new("set")
            .subcommand(set_header(Arguments::header()))
            .subcommand(set_title(Arguments::title()))
    }

    fn write() -> Command {
        Command::new("write")
    }
}

struct Arguments;

impl Arguments {
    pub fn title() -> [Arg; 1] {
        // Leave out length variants to disclude flag
        [Arg::new("title").required(true)]
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
    /// Handle top-level command
    pub fn handle_input() -> Result<(), Error> {
        let matches = Cli::matches();

        match matches.subcommand() {
            Some(("set", matches)) => Self::handle_set_command(matches)?,
            Some(("write", matches)) => Self::handle_write_command(matches)?,
            Some((unknown, _)) => eprintln!("Subcommand {:#?} not recognized", unknown),
            None => eprintln!("No matches found for subcommand..."),
        };

        Ok(())
    }

    fn handle_set_command(matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("header", args)) => {
                let name = args.get_one::<String>("name").unwrap();
                println!("{:#?}", &name);
                let profession = args.get_one::<String>("profession").unwrap();
                println!("{:#?}", &profession);
            }
            Some(("title", args)) => println!("{:#?}", args),
            Some((unknown, _)) => eprintln!("Subcommand {:#?} not recognized.", unknown),
            None => eprintln!("No matches found for subcommand..."),
        };

        Ok(())
    }

    fn handle_write_command(matches: &ArgMatches) -> Result<(), Error> {
        println!("{:#?}", matches.subcommand());
        let generator = ResumeGenerator::new("My_New_Resume".to_string());
        let (doc, _pg_idx, _layer_idx) = generator.doc;

        let file = File::create(generator.filename)?;
        let buffer_writer = &mut BufWriter::new(file);

        doc.save(buffer_writer)?;

        Ok(())
    }
}
