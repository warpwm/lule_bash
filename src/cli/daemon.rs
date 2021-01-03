use crate::var;
use crate::scheme::*;
// use daemonize::Daemonize;
use std::{thread, time};
use anyhow::{Result, Context};

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use crate::helper;
use crate::fun::fifo;

pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {

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

    deamoned(app, output, scheme)?;
    Ok(())
}

fn deamoned(_app: &clap::ArgMatches<'_>, _output: &mut WRITE, scheme: &mut SCHEME) -> Result<()> {
    // let sub = app.subcommand_matches("daemon").unwrap();
    // run_create(output, scheme);
    // tokio::task::spawn( async move { reader.read_string().await } );

    let mut pipe_name = std::env::temp_dir();
    pipe_name.push("lule_pipe");
    std::fs::remove_file(pipe_name.to_str().unwrap()).ok();
    std::fs::File::create(pipe_name.to_str().unwrap()).ok();
    let _pipe = fifo::Pipe::new(pipe_name.to_str().unwrap()).ensure_exists().unwrap();


    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch(pipe_name.to_str().unwrap(), RecursiveMode::NonRecursive).unwrap();

    for i in 0..scheme.looop().unwrap() {
        if rx.try_recv().is_ok() {
            if let Ok(content) = helper::file_to_string(pipe_name.clone()) {
                if let Ok(profile) = make_scheme(content.clone()) {
                    let jsonified = serde_json::to_value(&profile).unwrap();
                    println!("{}", jsonified);
                } else {
                    println!("something bad happened");
                }
            }
            std::fs::remove_file(pipe_name.to_str().unwrap()).ok();
            std::fs::File::create(pipe_name.to_str().unwrap()).ok();
            continue
        }
        thread::sleep(time::Duration::from_secs(1));
        println!("{:?}", i);
    }


    println!("{}", "outside");
    Ok(())
}

fn make_scheme(data: String) -> Result<SCHEME> {
    let scheme: SCHEME = serde_json::from_str(&data).context("something got fucked-up reaading json")?;
    Ok(scheme)
}
