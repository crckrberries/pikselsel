// this module contains the color structures and functions

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    // pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    //     return Color {
    //         r,
    //         g,
    //         b,
    //         a
    //     }
    // }

    // pub fn hexify(c: Color) -> String {
    //     return String::from(format!("{:02x}{:02x}{:02x}", c.r, c.g, c.b))
    // }

    pub fn hexify_rgb(r: u8, g: u8, b: u8, a: u8) -> String {
        // turns rgb values into a hexadecimal color code
        return String::from(format!("{:02x}{:02x}{:02x}{:02x}", r, g, b, a));
    }
}
