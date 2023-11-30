fn main(){
    let _imutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation : {}",mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    //다음 코드는 에러를 야기한다. 왜냐하면 불변 변수 바인딩으로 변수를 정의했기 때문이다. 
    //_imutable_biding
}
