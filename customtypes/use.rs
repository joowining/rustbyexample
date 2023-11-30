#![allow(dead_code)]

enum Status{
    Rich,
    Poor,
}

enum Work{
    Civilian,
    Soldier,
}

fn main(){
    // 명시적으로 use를 각 enum에 사용함으로써 일반적인 범위 설정을 하지 않아도 된다. 
    use crate::Status::{Poor, Rich};
    // 자동으로 Work내의 이름을 사용 가능하게 만든다.
    use crate::Work::*;

    // 다음의 코드는 Status::Poor와 동일하다. 
    let status = Poor;
    // 다음의 코드는 Work::Civilian과 동일하다. 
    let work = Civilian;

    match status {
        Rich => println!("The rich have lost of money"),
        Poor => println!("The Poor have not enough money"),
    }
    match work{
        Civilian => println!("Civilian work!"),
        Soldier => println!("Soldiers fight"),
    }
}
