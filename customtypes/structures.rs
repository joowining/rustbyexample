#![allow(dead_code)]

#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

// Unit 구조체
struct Unit;

// tuple 구조체
struct Pair(i32, f32);

// 두개의 feild를 가진 구조체
struct Point{
    x: f32, 
    y: f32,
}

// 하나의 구조체는 다른 구조체의 필드로써 사용할 수 있다. 
struct Rectangle{
    // 사각형의 특징으로 상하 좌우의 각을 표현
    top_left: Point,
    bottom_right: Point,
}

fn main(){
    //간단하게 필드값이 될 것들을 생성하여 구조체 생성
    let name = String::from("peter");
    let age = 27;
    let peter = Person { name, age};

    println!("{:?}", peter);

    // Point 구조체의 인스턴스 생성
    let point : Point = Point { x: 10.3, y: 0.4};

    // point 구조체의 필드값에 접근하기
    println!("point coordinates : ( {} {} )", point.x, point.y);

    // 새로운 point 구조체 인스턴스를 만들되struct update 문법을 사용하여 만든다. 
    // 새로운 구조체를 생성할 때 이미 입력한 필드값을 제외하고 요구되는 다른 필드값을 
    // 동일한 타입의 구조체 인스턴스가 가진 나머지 필드값으로 대체한다. 
    let bottom_right = Point { x: 5.2, ..point };

    // bottom_right.y와 point.y는 동일하다. 왜냐하면 point의 필드를 그대로 가져오기 때문이다. 
    println!("second point:({} {}", bottom_right.y, point.y);

    // let binding을 통해 point를 destructure 할 수 도 있다. 
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct 인스턴스 생성은 구문으로 여겨진다. 
        top_left : Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // unit 구조체 생성
    let _unit = Unit;

    // tuple 구조체 생성
    let pair = Pair(1, 0.1);

    // tuple구조체의 필드에 접근하기
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // tuple 구조체도 destructre 가 가능하다. 
    let Pair ( integer, decimal ) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
