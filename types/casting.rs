#![allow(overflowing_literals)]

fn main(){
    let decimal = 65.4321_f32;

    // 다음과 같은 암시적 반환은 이루어지지 않는다. 
    // let integer: u8 = decimal;

    // 명시적 반환의 예시는 다음과 같다. 
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting :{} -> {} -> {}", decimal, integer, character);

    // 어떤 값을 부호 없는 유형으로 캐스팅할 때, 값이 새 유형에 맞을 때까지
    // T 혹은 T::MAX + 1을 더하거나 뺀다. 

    // 1000은 이미 u16 type에 적합하다. 
    println!("1000 as an u16 is {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 내부적으로 첫번째 8개의 명시적 비트는 유지된다. (less significant bits)
    // 나머지가 계속 msot significant bit (MSB)가 빠진다. 
    println!("1000 as a u8 is {}", 1000 as u8);
    // -1 + 256 = 255
    println!(" -1 as an u8 is {}", (-1i8) as u8);

    // 양수이지만 타입의 범위를 넘어가는 값에 대해서는 
    // 다음의 연산과 타입캐스팅의 방식은 동일하다. 
    println!("1000 mod 256 is {}", 1000 % 256 );

    // 부호 있는 타입으로 변환시에
    // 첫번째 캐스팅과 유사하게 일어나지만,
    // 해당 값의 범위를 넘어가는 경우 양수 -> 음수가 된다. 
    
    // 다음은 이미 타입에 적합하다. 
    println!("128 as a i16 is {}", 128 as i16);

    // 범위가 127 ~ -128인 i8 경우에 다음은 -128과 동일한 값으로 변환된다. 
    println!("128 is an i8 is {}", 128 as i8);

    // 위의 예시를 다시 반복해보면
    // 1000 as u8 -> 232
    println!("1000 as an u8 is : {}", 1000 as u8 );
    // 232라는 값에 대한 8비트로 나타낸 표현식은 -24가 된다. 
    println!("232 as an i8 is : {}", 232 as i8);

    // as 키워드는 포화 캐스팅의 역할을 한다. 
    // float에서 int로 캐스팅을 할 때 만약 float가 최대, 최소값을 넘길 경우 
    // 그 부호의 끝값에 일치시킨다. 
    // 300.0 as u8 = 255
    println!("300.0 as u8 is :{}", 300.0_f32 as u8);
    // -100.0 as u8 = 0 
    println!("-100.0 as u8 is : {}", (-100.0_f32) as u8);
    // nan as u8 is 0
    println!(" nan as u8 is : {}", f32::NAN as u8);

    // 다음의 동작들은 런타임 비용을 발생시키며 피해야할 위험한 메소드들이다. 
    // 그러나 결과는 overflow되거나 unsound values를 리턴할 수도 잇으니 
    // 잘 생각해서 사용해야 한다. 
    unsafe {
        // 300.0 as u8 is 44
        println!("300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!(" nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
