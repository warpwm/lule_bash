use std::process::Command;
// use crate::scheme::scheme::*;

pub fn external_command(){
    Command::new("bash")
                .arg("-c")
                .arg("/home/bresilla/code/proj/warp/lule/scripts/lule_colors")
                .output()
                .expect("failed to execute process").stdout;
}

