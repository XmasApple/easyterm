use std::fmt::Display;
use std::io::Write;

use num::FromPrimitive;
use num::Num;

pub struct EasyTerm {}

impl EasyTerm {
    pub fn clear_screen() {
        print!("\x1B[2J");
    }
    pub fn set_cursor_invisible() {
        print!("\x1B[?25l");
    }
    pub fn set_cursor_top_left() {
        print!("\x1B[1;1H");
    }
    pub fn set_cursor_pos<N: Num + Display>(x: N, y: N) {
        print!("\x1B[{};{}H", y, x);
    }
    pub fn move_cursor_up() {
        print!("\x1B[1A");
    }
    pub fn move_cursor_down() {
        print!("\x1B[1B");
    }
    pub fn move_cursor_left() {
        print!("\x1B[1D");
    }
    pub fn move_cursor_right() {
        print!("\x1B[1C");
    }
    pub fn move_cursor_up_by<N: Num + Display>(n: N) {
        print!("\x1B[{}A", n);
    }
    pub fn move_cursor_down_by<N: Num + Display>(n: N) {
        print!("\x1B[{}B", n);
    }
    pub fn move_cursor_left_by<N: Num + Display>(n: N) {
        print!("\x1B[{}D", n);
    }
    pub fn move_cursor_right_by<N: Num + Display>(n: N) {
        print!("\x1B[{}C", n);
    }
    pub fn move_cursor_by<N>(x: N, y: N)
    where
        N: Num + Display + PartialOrd + FromPrimitive + std::ops::Neg<Output = N>,
    {
        if x >= FromPrimitive::from_i32(0).unwrap() {
            if y >= FromPrimitive::from_i32(0).unwrap() {
                print!("\x1B[{}C\x1B[{}A", x, y)
            } else {
                print!("\x1B[{}C\x1B[{}B", x, -y)
            }
        } else {
            if y >= FromPrimitive::from_i32(0).unwrap() {
                print!("\x1B[{}D\x1B[{}A", -x, y)
            } else {
                print!("\x1B[{}D\x1B[{}B", -x, -y)
            }
        }
    }
    pub fn print_at<N: Num + Display, T: Display>(x: N, y: N, s: T) {
        EasyTerm::set_cursor_pos(x, y);
        print!("{}", s);
        std::io::stdout().flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
