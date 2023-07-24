mod api;
mod cli;
mod consts;
mod errors;
mod file_io;
mod generate;
mod os;

use cli::Handler;
use errors::Error;

fn main() -> Result<(), Error> {
    //

    optick::event!();
    optick::start_capture();
    cli::CLParser::handle_input()?;
    optick::stop_capture("main");

    //

    Ok(())
}
