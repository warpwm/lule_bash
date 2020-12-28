use colored::*;

pub fn show_colors(colors: Vec<pastel::Color>) {
    for i in 0..colors.len() {
        let val = if true { format!("  {:#03}  ", i) } else { colors[i].to_rgb_hex_string(true) };
        if (i % 12 == 4 && i > 16) || (i == 16) { println!() };
        print!("{}",
            val.on_truecolor(
                colors[i].to_rgba().r.into(),
                colors[i].to_rgba().g.into(), 
                colors[i].to_rgba().b.into()
            ).color( if colors[i].to_lab().l < 50.0 { "white" } else { "black" } )
            );
    }
    println!();
}
