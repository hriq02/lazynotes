
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    BrightBlack,
    BrightWhite,
    BrightRed,
    BrightGreen,
    BrightBlue,
    BrightYellow,
    BrightMagenta,
    BrightCyan,
}



pub fn colorize(text : &str, color : Color)-> String{
    let color_code = match color{
        Color::Black => "30",
        Color::White => "37",
        Color::Red => "31",
        Color::Green => "32",
        Color::Blue => "34",
        Color::Yellow => "33",
        Color::Magenta => "35",
        Color::Cyan => "36",
        Color::BrightBlack => "90",
        Color::BrightWhite => "97",
        Color::BrightRed => "91",
        Color::BrightGreen => "92",
        Color::BrightBlue => "94",
        Color::BrightYellow => "93",
        Color::BrightMagenta => "95",
        Color::BrightCyan => "96",
    };

    format!("\x1b[{color_code}m{text}\x1b[0m")
}

//println!("\x1b[{color}m{}\x1b[0m", "teste");