mod cli;
mod errors;
mod file_io;
mod generate;
mod models;
mod os;

use errors::Error;

fn main() -> Result<(), Error> {
    //

    cli::CLParser::handle_input()?;

    //

    Ok(())
}
