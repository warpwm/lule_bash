use colored::*;
use crate::scheme::*;
use pastel::ansi;
use crate::display::canvas;
use std::io::{self, Write};
use atty::Stream;

pub fn show_colors(output: &WRITE) {
    for i in 0..output.colors().len() {
        let val = if !true { format!("  {:#03}  ", i) } else { format!(" {} ", output.colors()[i].to_rgb_hex_string(true)) };
        if (i % 12 == 4 && i > 16) || (i == 16 || i == 8) { println!() };
        if i == 16 || i == 232 { println!() };
        print!("{}",
            val.on_truecolor(
                output.colors()[i].to_rgba().r.into(),
                output.colors()[i].to_rgba().g.into(), 
                output.colors()[i].to_rgba().b.into()
            ).color( if output.colors()[i].to_lab().l < 50.0 { "white" } else { "black" } )
            );
    }
    println!();
}

fn write_stderr(c: Color, title: &str, message: &str) {
    writeln!(
        io::stderr(),
        "{}: {}",
        "error:",
        message
    ).ok();
}

pub fn show_pastel_colors(output: &WRITE) {
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();
    let mut out = canvas::Output::new(&mut stdout_lock);
    let config = canvas::Config {
        padding: 2,
        interactive_mode: atty::is(Stream::Stdout),
        brush: ansi::Brush::from_mode(Some(ansi::Mode::TrueColor)),
    };

    for color in output.colors().iter() {
        out.show_color(&config, color);
    }
}
