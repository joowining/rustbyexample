fn main(){
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // an_integer를 copied_integer에 복사하기 
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value {:?}", unit);

    // 컴파일러는 사용ㅈ하지 않은 변수 바인딩은 경고한다. 
    // 이러한 경고는 면수 명 앞에 밑줄을 더해줌으로써 피해갈 수 있다. 
    let _unused_varaible = 3u32;
    let _noisy_unused_variable = 2u32;
}
