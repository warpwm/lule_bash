use crate::var;
use crate::scheme::*;
// use daemonize::Daemonize;
use queue_file::QueueFile;
use std::{thread, time};

pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) {

    // deamoned(app, output, scheme);
    var::defs::concatinate(scheme);
    var::envi::concatinate(scheme);
    var::args::concatinate(app, scheme);
    var::pipe::concatinate(scheme);

    // let stdout = std::fs::File::create("/tmp/daemon.out").unwrap();
    // let stderr = std::fs::File::create("/tmp/daemon.err").unwrap();

    // let mut lule_pid = std::env::temp_dir();
    // lule_pid.push("lule_pid");

    // let lule = Daemonize::new()
    //     .pid_file(lule_pid.to_str().unwrap()) // Every method except `new` and `start`
    //     .chown_pid_file(true)      // is optional, see `Daemonize` documentation
    //     .working_directory("/tmp") // for default behaviour.
    //     .user(1000)
    //     .group(1000)
    //     .umask(0o777)
    //     .stdout(stdout)
    //     .stderr(stderr);

    // let rtm = tokio::runtime::Runtime::new().unwrap();
    // match lule.start() {
    //     // Ok(_) => rtm.block_on(deamoned(app, output, scheme)),
    //     Ok(_) => deamoned(app, output, scheme),
    //     Err(e) => eprintln!("Error, {}", e),
    // }

    deamoned(app, output, scheme)

}

fn deamoned(app: &clap::ArgMatches<'_>, output: &mut WRITE, scheme: &mut SCHEME){
    // let sub = app.subcommand_matches("daemon").unwrap();
    // run_create(output, scheme);
    // tokio::task::spawn( async move { reader.read_string().await } );

    let mut pipe_name = std::env::temp_dir();
    pipe_name.push("lule_pipe");
    std::fs::remove_file(pipe_name.to_str().unwrap()).ok();
    let mut qf = QueueFile::open(pipe_name.to_str().unwrap()).expect("cannot open queue file");

    let sleep = time::Duration::from_secs(1);
    let mut looop = 0;


    loop {
        qf = QueueFile::open(pipe_name.to_str().unwrap()).expect("cannot open queue file");
        thread::sleep(sleep);
        if !  qf.is_empty()  || looop > scheme.looop().unwrap() {
            break
        }

        looop += 1;
    }
    println!("{}", "outside");
}


// async fn run_create(_output: &mut WRITE, scheme: &mut SCHEME){
//     let scheme_json = serde_json::to_value(&scheme).unwrap();
//     println!("{}", scheme_json);
// }
