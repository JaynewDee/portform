mod api;
mod cli;
mod errors;
mod file_io;
mod generate;
mod os;

use cli::Handler;
use errors::Error;

fn main() -> Result<(), Error> {
    //

    cli::CLParser::handle_input()?;

    //

    Ok(())
}
