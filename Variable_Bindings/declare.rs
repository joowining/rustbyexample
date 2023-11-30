fn main(){
    // 변수 바인딩을 선언만 하기 
    let a_binding;

    {
        let x = 2;
        // 바인딩을 초기화하기
        a_binding = x*x;
    }
    prinltn!("a binding: {}",a_binding);
    let another_binding;

    // 선언만 하고 초기화하지 않은 바인딩인 another_binding은 사용할 수 없다. 
    // println!("another binding: {}", another_binding);
    another_binding = 1;
    println!("another binding :{}", another_binding);
}
