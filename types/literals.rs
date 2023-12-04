fn main(){
    // 접미사 리터럴 방식을 통해 초기화 시에 타입을 결정
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 접미사가 붙지 않은 리터럴
    let i = 1;
    let f = 1.0;

    // size_of_val은 해당 변수의 타입의 사이즈를 리턴한다. 
    println!("size of `x` in bytes : {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes : {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes : {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes : {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes : {}", std::mem::size_of_val(&f)));
}
