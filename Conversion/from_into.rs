let my_str = "hell0"; // str type 
let my_string = String::from(my_str); // str -> String

use std::convert::from;

#[derive(Debug)]
struct Number{
    value : i32,
}

impl From<i32> for Number{
    fn from(item: i32) -> Self {
        Number{value: item}
    }
}

impl Into<Number> for i32{
    fn into(self) -> Number{
        Number { value : self }
    }
}

fn main(){
    let num = Number::from(30);
    println!("My number is {:?", num);


    let int =5 ;

    let num: Number = int.into();

    println!("My number is {:?}", num);
}

