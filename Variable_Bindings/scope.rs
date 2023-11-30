fn main(){
    // 이 바인딩은 main 함수 내에서만 살아있게 된다. 
    let long_lived_binding = 1;

    // 이것은 메인 함수같은 블록을 제공하게 된다. 
    {
        // 여기의 바인딩은 오직 이 블록 내에서만 유효하다. 
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }
    // 블록이 종료된 후 더이상 short_lived_binding에 접근할 수 없다. 
    println!("outer long: {}", long_lived_binding);
}
