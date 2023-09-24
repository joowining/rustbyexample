fn main(){
    // 일반적으로 '{}'은 자연스럽게 그 다음의 인자로 여겨진다. 
    println!("{} days", 31);
    // {} 내에는 인덱싱을 통해 접근할 수도 있다. 
    println!("{0}, this is {1}. {1}, this is {0})", "Alice", "Bob");
    // 특정한 변수명을 할당해서 활용이 가능하다.
    println!("{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brwon fox",
            verb = "jumps over");
    // : 다음에 넣는 문자열로 원하는 형태로 형변환을 할 수 있다. 
    println!("Base 10:      {}", 69420);
    println!("Base 2 (binary):      {:b}", 69420);
    println!("Base 8 (octal):       {:o}", 69420);
    println!("Base 16 (hexadecimal):        {:x}",69420);
    println!("Base 16 (hexadecimal):        {:X}",69420);
    // format에 대해 좌 우로 원하는 형태로 만들어낼 수 있다. 
    // 예를 들어 다음예시들은 5라는 숫자만큼 좌 우로 : 다음의 내용을
    // 인자의 좌 우 사이로 채워준다. 
    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0<5}", number=1);
    println!("{number:0>width$}", number=1, width=5);

    println!("My name is {0}, {1} {0}", "Bond", "James");
    
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct `{:?}` won't print . . . ", Structure(3));
    
    let number: f64=1.0;
    let width: usize = 5;
    println!("{number:>width$}");

}
