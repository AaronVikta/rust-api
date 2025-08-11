#[allow(unused_imports)]
use clap::{Arg,Command};
use cli_app::commands;


pub fn main()->anyhow::Result<()>{

    let mut command = Command::new("Sample CLI application");
    command = commands::configure(command);

    let matches  = command.get_matches();
    commands::handle(&matches)?;

    Ok(())
}