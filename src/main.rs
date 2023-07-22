mod cli;
mod errors;
mod file_io;
mod generate;
mod models;
mod os;

use errors::Error;
use file_io::{ConfigFileHandler, FileHandler};
fn main() -> Result<(), Error> {
    //

    cli::CLParser::handle_input()?;

    ConfigFileHandler::write(models::DocumentShape::default())?;
    //

    Ok(())
}
