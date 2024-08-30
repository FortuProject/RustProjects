// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

pub fn main() {
    let pick = Color::Black;
    match pick {
        Color::Black => println!("Color equals black."),
        Color::White => println!("Color equals White."),
        Color::Green => println!("Color equals Green."),
        Color::Purple => println!("Color equals Purple.")
    }
}

enum Color{
    White,
    Black,
    Green,
    Purple,
}