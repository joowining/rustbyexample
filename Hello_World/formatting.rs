use std::fmt::{self, Formatter, Display};

struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color{
    fn fmt(&self, f: &mut Formatter)-> fmt::Result{
        let red : u64 =  *&self.red as u64 ;
        let green : u64 = *&self.green as u64;
        let blue : u64 = *&self.blue as u64;
        let rgb_space: u64 = red*65536 + green*256+blue;

        write!(f,"RGB ({},{},{}) 0x{:0>2}",&self.red, &self.green, &self.blue, rgb_space) 
    }
}

fn main(){
    for color in [
        Color { red:128, green: 255, blue:90},
        Color { red:0, green: 3, blue: 254},
        Color { red:0, green: 0, blue: 0}
    ]{
        println!("{}", color);
    }
}
