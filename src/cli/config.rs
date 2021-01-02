use crate::scheme::*;
use crate::var;
use queue_file::QueueFile;

pub fn run(app: &clap::ArgMatches, _output: &mut WRITE, scheme: &mut SCHEME) {
    let sub = app.subcommand_matches("config").unwrap();
    var::defs::concatinate(scheme);
    var::envi::concatinate(scheme);
    var::args::concatinate(app, scheme);
    var::pipe::concatinate(scheme);


    if atty::isnt(atty::Stream::Stdout) {
        let scheme_json = serde_json::to_value(&scheme).unwrap();
        println!("{}", scheme_json);
        
        let mut pipe_name = std::env::temp_dir();
        pipe_name.push("lule_pipe");
        let mut qf = QueueFile::open(pipe_name.to_str().unwrap()).expect("cannot open queue file");

        qf.add("something_cool".as_bytes()).expect("add failed");


    } else {
        if let Some(arg) = sub.value_of("action") {
            println!("{}", arg);
        }
    }
}
