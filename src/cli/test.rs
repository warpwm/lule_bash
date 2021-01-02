use crate::scheme::*;
use queue_file::QueueFile;

pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) {
    let mut pipe_name = std::env::temp_dir();
    pipe_name.push("lule_pipe");
    let mut qf = QueueFile::open(pipe_name.to_str().unwrap()).expect("cannot open queue file");

    qf.add("something_cool".as_bytes()).expect("add failed");

    println!("Sent data");
}
