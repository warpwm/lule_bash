use crate::scheme::*;

pub fn run(_app: &clap::ArgMatches, _output: &mut WRITE, scheme: &mut SCHEME) {

    let scheme_json = serde_json::to_value(&scheme).unwrap();
    println!("{}", scheme_json);

}
