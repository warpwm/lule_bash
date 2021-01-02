use crate::scheme::*;

pub fn run(app: &clap::ArgMatches, _output: &mut WRITE, scheme: &mut SCHEME) {
    let sub = app.subcommand_matches("config").unwrap();


    if atty::isnt(atty::Stream::Stdout) {
        let scheme_json = serde_json::to_value(&scheme).unwrap();
        println!("{}", scheme_json);
    } else {
        if let Some(arg) = sub.value_of("action") {
            println!("{}", arg);
        }
    }
}
