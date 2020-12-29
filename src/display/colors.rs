use colored::*;

use crate::scheme::*;

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
