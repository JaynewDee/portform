use clap::{arg, Arg, ArgMatches, Command};

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

    pub fn handle_arguments(args: ArgMatches) {
        unimplemented!()
    }
}

struct Subcommands;

impl Subcommands {
    pub fn set() -> Command {
        Command::new("set")
            .arg(Arguments::title())
            .arg(Arguments::header())
            .subcommand(
            Command::new("set")
                .arg(arg!(-n --name <HUMAN_NAME> "Name of human professional"))
                .arg(arg!(-prof --profession <PRACTICING_PROFESSION> "Human's professional title")),
        )
    }
}

struct Arguments;

impl Arguments {
    pub fn title() -> Arg {
        arg!(-t --title <DOCUMENT_TITLE> "Title of PDF document").required(false)
    }

    pub fn header() -> Arg {
        arg!(-head --header <NAME_AND_PROFESSION> "Name of wielder with profession subtitle")
            .required(false)
    }
}
