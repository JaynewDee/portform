use super::api::{Header, HistoryEntry, Project, Skill, Summary};
use super::errors::Error;
use super::file_io::{ConfigFileHandler, FileHandler};
use super::generate::ResumeWriter;
use clap::{Arg, ArgMatches, Command};

pub struct Cli;

impl Cli {
    pub fn matches() -> ArgMatches {
        Command::new("PortForm")
            .version("1.0")
            .author("Joshua Newell Diehl <jdiehl2236@gmail.com>")
            .about("PDF Resume Builder")
            .subcommand(Subcommands::list())
            .subcommand(Subcommands::set())
            .subcommand(Subcommands::write())
            .arg_required_else_help(true)
            .get_matches()
    }
}

struct Subcommands;

trait Operator {
    fn list() -> Command;
    fn set() -> Command;
    fn write() -> Command;
}

impl Operator for Subcommands {
    fn list() -> Command {
        Command::new("list").subcommand(Command::new("config"))
    }
    fn set() -> Command {
        let set_filename = move |args| Command::new("filename").args(args);
        let set_title = move |args| Command::new("title").args(args);
        let set_header = move |args| Command::new("header").args(args);
        let set_summary = move |args| Command::new("summary").args(args);
        let set_employment_history = move |args| Command::new("employment").args(args);
        let set_projects = move |args| Command::new("projects").args(args);
        let set_skillset = move |args| Command::new("skills").args(args);
        let set_contact_details = move |args| Command::new("contact").args(args);

        Command::new("set")
            .subcommand(set_filename(Arguments::filename()))
            .subcommand(set_title(Arguments::title()))
            .subcommand(set_header(Arguments::header()))
            .subcommand(set_summary(Arguments::summary()))
            .subcommand(set_employment_history(Arguments::employment_history()))
            .subcommand(set_projects(Arguments::projects()))
            .subcommand(set_skillset(Arguments::skillset()))
            .subcommand(set_contact_details(Arguments::contact_details()))
    }

    fn write() -> Command {
        Command::new("write")
    }
}

struct Arguments;

impl Arguments {
    pub fn filename() -> [Arg; 1] {
        [Arg::new("filename").required(true)]
    }

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

    pub fn summary() -> [Arg; 1] {
        [Arg::new("summary").required(true)]
    }

    // New History entry will be appended to vector, overwriting oldest if overflow
    pub fn employment_history() -> [Arg; 5] {
        [
            Arg::new("position").long("position").required(true),
            Arg::new("location").long("location").required(true),
            Arg::new("start").long("start").required(false),
            Arg::new("end").long("end").required(false),
            Arg::new("description").long("description").required(true),
        ]
    }

    pub fn projects() -> [Arg; 3] {
        [
            Arg::new("name").long("name").required(true),
            Arg::new("description").long("description").required(true),
            Arg::new("deployment").long("deployment").required(false),
        ]
    }

    pub fn skillset() -> [Arg; 1] {
        [Arg::new("")]
    }

    pub fn contact_details() -> [Arg; 4] {
        [
            Arg::new("email").long("email").required(true),
            Arg::new("website").long("website").required(true),
            Arg::new("phone").long("phone").required(false),
            Arg::new("address").long("address").required(false),
        ]
    }
}

pub struct CLParser;

impl CLParser {
    /// Handle top-level command
    pub fn handle_input() -> Result<(), Error> {
        let matches = Cli::matches();

        match matches.subcommand() {
            Some(("list", matches)) => Self::handle_list_command(matches)?,
            Some(("set", matches)) => Self::handle_set_command(matches)?,
            Some(("write", matches)) => Self::handle_write_command(matches)?,
            Some((unknown, _)) => eprintln!("Subcommand {:#?} not recognized", unknown),
            None => eprintln!("No matches found for subcommand..."),
        };

        Ok(())
    }

    fn handle_list_command(matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("config", _)) => {
                let config = ConfigFileHandler::read()?;
                println!("{:#?}", &config);
            }
            _ => eprintln!("Unknown subcommand ... "),
        };

        Ok(())
    }

    fn handle_set_command(matches: &ArgMatches) -> Result<(), Error> {
        let mut document_config = ConfigFileHandler::read()?;

        match matches.subcommand() {
            Some(("filename", args)) => {
                document_config.filename = args.get_one::<String>("filename").cloned();
            }
            Some(("title", args)) => {
                document_config.title = args.get_one::<String>("title").cloned()
            }
            Some(("header", args)) => {
                let name = args.get_one::<String>("name").cloned().unwrap();
                let profession = args.get_one::<String>("profession").cloned().unwrap();
                let header = Some(Header { name, profession });
                document_config.header = header;
            }
            Some(("summary", args)) => {
                let summary = Some(Summary {
                    body: args.get_one::<String>("summary").cloned().unwrap(),
                });
                document_config.summary = summary;
            }
            Some(("employment", args)) => {
                let position = args.get_one::<String>("position").cloned().unwrap();
                let location = args.get_one::<String>("location").cloned().unwrap();
                let start = args
                    .get_one::<String>("start")
                    .cloned()
                    .unwrap_or(String::with_capacity(7));
                let end = args
                    .get_one::<String>("end")
                    .cloned()
                    .unwrap_or(String::with_capacity(7));
                let description = args.get_one::<String>("description").cloned().unwrap();

                let history_entry = HistoryEntry {
                    position,
                    location,
                    dates_employed: (start, end),
                    description,
                };

                if let Some(vector) = document_config.employment_history.as_mut() {
                    vector.push(history_entry);
                } else {
                    document_config.employment_history = Some(vec![history_entry]);
                }
            }
            Some(("projects", args)) => {
                let name = args.get_one::<String>("name").cloned().unwrap();
                let description = args.get_one::<String>("description").cloned().unwrap();
                let deployment = args.get_one::<String>("deployment").cloned().unwrap();

                let project_entry = Project {
                    name,
                    description,
                    deployment,
                };

                if let Some(vector) = document_config.projects.as_mut() {
                    vector.push(project_entry);
                } else {
                    document_config.projects = Some(vec![project_entry]);
                }
            }
            Some(("skills", args)) => {
                let name = args.get_one::<String>("name").unwrap().to_string();

                let skill = Skill { name };

                if let Some(vector) = document_config.skillset.as_mut() {
                    vector.push(skill);
                } else {
                    document_config.skillset = Some(vec![skill])
                }
            }
            Some((unknown, _)) => eprintln!("Subcommand {:#?} not recognized.", unknown),
            None => eprintln!("No matches found for subcommand..."),
        };

        ConfigFileHandler::write(document_config)?;

        Ok(())
    }

    fn handle_write_command(matches: &ArgMatches) -> Result<(), Error> {
        println!("{:#?}", matches.subcommand());
        let generator = ResumeWriter::new();
        let (doc, _pg_idx, _layer_idx) = generator.doc;

        Ok(())
    }
}
