use catppuccin::ColorName;
use raylib::prelude::*;

pub fn get(name: ColorName) -> Color {
    let hex = &catppuccin::PALETTE.mocha.get_color(name).hex.to_string();

    Color::from_hex(&hex[1..]).unwrap()
}
