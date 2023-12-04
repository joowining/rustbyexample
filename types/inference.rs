fn main(){
    // 어노테이션 덕분에 컴파일러는 elem이 u8인지 알 수 있다. 
    let elem = 5u8;

    // 빈 가변 벡틀를 생성한다.
    let mut vec = Vec::new();
    // 아직 컴파일러는 vec의 정확한 타입을 모른다. 
    // 단지 그것이 어떤 것의 벡터임만을 알고 있다. Vec<_>
    
    // elem을 그 벡터에 넣으면 
    vec.push(elem);
    // 컴파일러가 이제 그 벡터는 Vec<u8>임을 알게 된다. 
    println!("{:?}", vec);
}
