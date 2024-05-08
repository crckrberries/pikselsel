// this module contains the frame structures and functions

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn hexify_rgb(r: u8, g: u8, b: u8, a: u8) -> String {
        // turns rgb values into a hexadecimal color code
        format!("{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub commands: Vec<String>,
    pub delay: u32,
}

#[cfg(test)]
mod tests {
    use crate::frame::*;

    #[test]
    fn test_hexify() {
        assert_eq!(
            Color::hexify_rgb(255, 178, 77, 255),
            String::from("ffb24dff")
        );
    }
}
