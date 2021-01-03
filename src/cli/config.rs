use crate::scheme::*;
use crate::helper;
use crate::var;

pub fn run(app: &clap::ArgMatches, _output: &mut WRITE, scheme: &mut SCHEME) {
    // let sub = app.subcommand_matches("config").unwrap();
    var::defs::concatinate(scheme);
    var::envi::concatinate(scheme);
    var::args::concatinate(app, scheme);
    var::pipe::concatinate(scheme);


    let scheme_json = serde_json::to_value(&scheme).unwrap();
    let format_scheme = format!("{}", scheme_json);
    if atty::isnt(atty::Stream::Stdout) {
        println!("{}", scheme_json);
    } else {
        println!("{}", scheme_json);
        let mut pipe_name = std::env::temp_dir();
        pipe_name.push("lule_pipe");
        helper::write_to_file(pipe_name, format_scheme.as_bytes());
    }
}

