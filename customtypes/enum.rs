// 웹 이벤트를 분류할 목적으로 enum을 생성한다. 
// 이름과 타입정보가 그 variant를 구분하는 것을 확인하게 될 것이다. 
// 'PageLoad != PageUnload'와 KeyPress(char) != Paste(String'.
// 이들을 각각 서로 다르며 독립적이다. 
enum WebEvent{
    // enum variant 는 'unit-like' 형태가 될 수 있다. 
    PageLoad,
    PageUnload,
    // tuple 구조체를 닮을 수도 있다. 
    KeyPress(char),
    Paste(String),
    // 혹은 C와 같은 구조체 형태일 수 있다. 
    Click { x: i64, y: i64},
}

// WebEvent enum을 매개변수로 받ㅇ고 아무것도 반환하지 않고 구분만 하는 함수이다. 
fn inspect(event: WebEvent){
    match event{
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // enum variant내의 c라는 내용을 destructing 
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s), 
        //Click k variant의 내용중에서 x, y를 destructing 
        WebEvent::Click { x, y } => {
            println!("clicked at x = {}, y = {}.", x, y );
        },
    }
}

fn main(){
    let pressed = WebEvent::KeyPress('x');
    // to_owned 메소드는 string slice로부터 String과 소유권을 같이 반환해준다. 
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click{ x:20, y: 80} ;
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
