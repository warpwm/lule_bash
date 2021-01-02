use crate::var;
use crate::helper;
use crate::scheme::*;
use crate::fun::fifo;
use daemonize::Daemonize;
use std::path::{Path, PathBuf};


pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) {

    // deamoned(app, output, scheme);

    let stdout = std::fs::File::create("/tmp/daemon.out").unwrap();
    let stderr = std::fs::File::create("/tmp/daemon.err").unwrap();

    let lule = Daemonize::new()
        .pid_file("/tmp/test.pid") // Every method except `new` and `start`
        // .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .group(1000)      // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match lule.start() {
        Ok(_) => deamoned(app, output, scheme),
        Err(e) => eprintln!("Error, {}", e),
    }

}

fn deamoned(app: &clap::ArgMatches, _output: &mut WRITE, scheme: &mut SCHEME) {
    // let sub = app.subcommand_matches("daemon").unwrap();
    // var::defs::concatinate(scheme);
    // var::envi::concatinate(scheme);
    // var::args::concatinate(app, scheme);
    // var::pipe::concatinate(scheme);

    // let scheme_json = serde_json::to_value(&scheme).unwrap();
    // println!("{}", scheme_json);

    // println!("{}", "daemonized");
    //
    let mut pipe_name = dirs::runtime_dir().unwrap();
    pipe_name.push("lule_pipe");
    let lule_fifo = fifo::Pipe::new(pipe_name);

    helper::write_to_file(PathBuf::from("/tmp/daemon.out"), "before_read".as_bytes());
    let reader = lule_fifo.open_read();
    // let t2 = tokio::task::spawn(async move { reader.read_string().await });
    helper::write_to_file(PathBuf::from("/tmp/daemon.out"), "after_read".as_bytes());

        
    
}
