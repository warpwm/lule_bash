use std::io::Write;

use pastel::ansi::{Brush, ToAnsiStyle};
use pastel::Color;
use pastel::Format;

pub struct Canvas {
    height: usize,
    width: usize,
    pixels: Vec<Option<Color>>,
    chars: Vec<Option<char>>,
    brush: Brush,
}

impl Canvas {
    pub fn new(height: usize, width: usize, brush: Brush) -> Self {
        assert!(height % 2 == 0);

        let mut pixels = vec![];
        pixels.resize(height * width, None);
        let mut chars = vec![];
        chars.resize(height / 2 * width, None);

        Canvas {
            height,
            width,
            pixels,
            chars,
            brush,
        }
    }

    pub fn draw_rect(
        &mut self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
        color: &Color,
    ) {
        for i in 0..height {
            for j in 0..width {
                *self.pixel_mut(row + i, col + j) = Some(color.clone());
            }
        }
    }

    pub fn draw_checkerboard(
        &mut self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
        dark: &Color,
        light: &Color,
    ) {
        for i in 0..height {
            for j in 0..width {
                let color = if (i + j) % 2 == 0 { dark } else { light };
                *self.pixel_mut(row + i, col + j) = Some(color.clone());
            }
        }
    }

    pub fn draw_text(&mut self, row: usize, col: usize, text: &str) {
        assert!(row % 2 == 0);

        for (j, c) in text.chars().enumerate() {
            *self.char_mut(row / 2, col + j) = Some(c);
        }
    }

    pub fn print(&self, out: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>> {
        for i_div_2 in 0..self.height / 2 {
            for j in 0..self.width {
                if let Some(c) = self.char(i_div_2, j) {
                    write!(out, "{}", c)?;
                } else {
                    let p_top = self.pixel(2 * i_div_2, j);
                    let p_bottom = self.pixel(2 * i_div_2 + 1, j);

                    match (p_top, p_bottom) {
                        (Some(top), Some(bottom)) => write!(
                            out,
                            "{}",
                            self.brush.paint("▀", top.ansi_style().on(bottom))
                        )?,
                        (Some(top), None) => write!(out, "{}", self.brush.paint("▀", top))?,
                        (None, Some(bottom)) => write!(out, "{}", self.brush.paint("▄", bottom))?,
                        (None, None) => write!(out, " ")?,
                    };
                }
            }
            writeln!(out)?;
        }
        Ok(())

    }

    fn pixel(&self, i: usize, j: usize) -> &Option<Color> {
        assert!(i < self.height);
        assert!(j < self.width);
        &self.pixels[i * self.width + j]
    }

    fn pixel_mut(&mut self, i: usize, j: usize) -> &mut Option<Color> {
        assert!(i < self.height);
        assert!(j < self.width);
        &mut self.pixels[i * self.width + j]
    }

    fn char(&self, i: usize, j: usize) -> &Option<char> {
        assert!(i < self.height / 2);
        assert!(j < self.width);
        &self.chars[i * self.width + j]
    }

    fn char_mut(&mut self, i: usize, j: usize) -> &mut Option<char> {
        assert!(i < self.height / 2);
        assert!(j < self.width);
        &mut self.chars[i * self.width + j]
    }
}


pub struct Config<> {
    pub padding: usize,
    pub interactive_mode: bool,
    pub brush: Brush,
}

pub struct Output<'a> {
    pub handle: &'a mut dyn Write,
    colors_shown: usize,
}

impl Output<'_> {
    pub fn new(handle: &mut dyn Write) -> Output {
        Output {
            handle,
            colors_shown: 0,
        }
    }

    pub fn show_color_tty(&mut self, config: &Config, color: &Color) -> Result<(), Box<dyn std::error::Error>> {
        let checkerboard_size: usize = 16;
        let color_panel_size: usize = 12;

        let checkerboard_position_y: usize = 0;
        let checkerboard_position_x: usize = config.padding;
        let color_panel_position_y: usize =
            checkerboard_position_y + (checkerboard_size - color_panel_size) / 2;
        let color_panel_position_x: usize =
            config.padding + (checkerboard_size - color_panel_size) / 2;
        let text_position_x: usize = checkerboard_size + 2 * config.padding;
        let text_position_y: usize = 0;

        let mut canvas = Canvas::new(checkerboard_size, 51, config.brush);
        canvas.draw_checkerboard(
            checkerboard_position_y,
            checkerboard_position_x,
            checkerboard_size,
            checkerboard_size,
            &Color::graytone(0.94),
            &Color::graytone(0.71),
        );
        canvas.draw_rect(
            color_panel_position_y,
            color_panel_position_x,
            color_panel_size,
            color_panel_size,
            color,
        );

        let text_y_offset = 0;
        // let similar = similar_colors(&color);

        // for (i, nc) in similar.iter().enumerate().take(3) {
        //     if nc.color == *color {
        //         canvas.draw_text(
        //             text_position_y,
        //             text_position_x,
        //             &format!("Name: {}", nc.name),
        //         );
        //         text_y_offset = 2;
        //         continue;
        //     }

        //     canvas.draw_text(text_position_y + 10 + 2 * i, text_position_x + 7, nc.name);
        //     canvas.draw_rect(
        //         text_position_y + 10 + 2 * i,
        //         text_position_x + 1,
        //         2,
        //         5,
        //         &nc.color,
        //     );
        // }

        #[allow(clippy::identity_op)]
        canvas.draw_text(
            text_position_y + 0 + text_y_offset,
            text_position_x,
            &format!("Hex: {}", color.to_rgb_hex_string(true)),
        );
        canvas.draw_text(
            text_position_y + 2 + text_y_offset,
            text_position_x,
            &format!("RGB: {}", color.to_rgb_string(Format::Spaces)),
        );
        canvas.draw_text(
            text_position_y + 4 + text_y_offset,
            text_position_x,
            &format!("HSL: {}", color.to_hsl_string(Format::Spaces)),
        );

        canvas.draw_text(
            text_position_y + 8 + text_y_offset,
            text_position_x,
            "Most similar:",
        );

        canvas.print(self.handle)
    }

    pub fn show_color(&mut self, config: &Config, color: &Color) -> Result<(), Box<dyn std::error::Error>> {
        if config.interactive_mode {
            if self.colors_shown < 1 {
                writeln!(self.handle)?
            };
            self.show_color_tty(config, color)?;
            writeln!(self.handle)?;
        } else {
            writeln!(self.handle, "{}", color.to_rgb_hex_string(true))?;
        }
        self.colors_shown += 1;

        Ok(())
    }
}
