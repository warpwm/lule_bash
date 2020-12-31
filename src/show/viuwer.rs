use std::io::stdout;
use crossterm::{execute, terminal::{Clear, ClearType, ScrollUp, SetSize, size}};
use crate::scheme::*;

pub fn display_image(output: &WRITE, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, rows) = size()?;
    execute!(
        stdout(),
        SetSize(10, 10),
        ScrollUp(5)
    )?;
    execute!(stdout(), SetSize(cols, rows))?;
    execute!(stdout(), Clear(ClearType::FromCursorDown))?;

    let filename = output.wallpaper().clone();
    let conf = viuer::Config {
        x: 0,
        y: 0,
        restore_cursor: true,
        // use_kitty: true,
        width: Some(width),
        height: Some(height),
        ..Default::default()
    };

    viuer::print_from_file(&filename, &conf)?;

    Ok(())
}
