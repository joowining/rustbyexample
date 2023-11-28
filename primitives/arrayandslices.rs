use std::mem;

// 이 함수는 slice를 빌린다. 
fn analyze_slice(slice: &[i32]){
    println!("First element of the slice: {}", slice[0]);
    println!("slice has {} elements", slice.len());
}

fn main(){
    // 고정된 크기의 array ( 불필요한 타입 시그니처를 포함했다. 
    let xs : [i32; 5 ] = [1,2,3,4,5];

    // 모든 원소들은 같은 값으로도 초기화가 가능하다. 
    let ys: [i32; 500 ] = [0; 500];

    // 인덱스는 0부터 시작한다. 
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // .len 메소드는 해당 배열의 원소의 수를 리턴한다. 
    println!("Number of elements in array: {}", xs.len());

    // array는 스택에 저장된다. 
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // array는 자동적으로 slices의 형태로 빌려진다. 
    println!("Borrow the whole array as a slice");
    analyze_slice(&xs);

    // slices는 특정 부분의 array를 가리킬 수 있다. 
    // 그것들은 다음 형태로 정의된다 [시작 인덱스 .. 마지막 인덱스].
    // 시작 인덱스는 해당 slice의 첫번째 위치를 차지하게 되며
    // 마지막 인덱스는 해당 slice의 마지막 위치를 차지하게 된다. 
    println!("Borrow a section of the array as slice");
    analyze_slice(&ys[1..4]);

    // 빈 슬라이스 &[]의 예시는 다음과 같다. 
    let empty_array:[u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    // array는 .get메소드를 통해 안전하게 접근가능하다. 
    // 이 메소드는 Option을 리턴하는데 이 Option은 다음과 같이 매치될 수 있다. 
    // 혹은 .expect() 메소드를 사용해서 array에 접근이 실패할 경우 메세지와 함께
    // 프로그램을 종료시킬 수도 있다. 
    for i in 0..xs.len() + 1{
        match xs.get(i){
            Some(xval) => println!(" {} : {}", i, xval),
         None => println!("Slow donw! {} is too far", i),
        }
    }

    // array에 대한 인덱스 아웃바운딩은 에러를 야기한다. 
    // println!("{}, xs[5]);
    // slice에 대한 인덱스 아웃바인딩은 런타임에 에ㅇ러를 야기한다. 
    // println!("{}", xs[..][5]);
}
