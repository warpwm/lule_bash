use crate::scheme::*;
use anyhow::Result;

pub fn run(_app: &clap::ArgMatches, _output: &mut WRITE, _scheme: &mut SCHEME) -> Result<()> {
    // let mut pipe_name = std::env::temp_dir();
    // pipe_name.push("lule_pipe");

    println!("Sent data");
    Ok(())
}
