fn main(){
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    inferred_type = 4294967295i64;

    let mut mutable = 12;
    mutable = 21;

    // err mutable = ture;
    //
    let mutable = true; 
}
