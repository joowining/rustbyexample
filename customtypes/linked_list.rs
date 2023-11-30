use crate::List::*;

enum List{
    // Cons : 자신의 원소와 다음의 노드를 가리키는 포인터를 가진 튜플 구조체 
    Cons(u32, Box<List>),
    // Nil : 해당 연결리스트의 마지막 노드
    Nil,
}

// Methods를 enum에 연결시킬 수 있다. 
impl List{
    // 빈 리스트를 생성하기
    fn new() -> List{
        // Nil의 반환 타입은 List이다. 
        Nil
    }
    // list를 소비하되 새로운 원소를 그 앞에 추가하여 해당 리스트를 리턴하는 메소드
    fn prepend(self, elem: u32) -> List{
        // Cons의 타입도 List이다. 
        Cons(elem, Box::new(self))
    }
    // list의 길이를 리턴한다. 
    fn len(&self) -> u32 {
        // self라는 것은 match를 통해 구분된다. 
        // 왜냐하면 이 메소드의 수행은 self의 variant에 달려있기 때문이다. 
        // &self는 &List 타입을 가지고 *self는 List타입을 가지게 된다. 
        // 그러한 상황에서 러스트는 구체적 유형 T에 대한 match일치가 
        // 참조 &T에 대한 일치보다 선호된다. 
        // Rust 2018이후로 self나 tail( ref 키워드없이)를 여기에 사용할 수 있다. 
        // Rust가 &s와 ref tail을 추론할 것이기 때문이다. 
        match *self{
            // self는 빌린 것이기 때문에 tail에 대한 소유권을 가질 수 없다. 
            // 대신 tail에 대한 참조를 가져온다. 
            Cons(_, ref tail ) => 1+ tail.len(),
            // 빈 리스트는 길이가 0이된다. 
            Nil => 0
        }
    }

    // heap에 해당리스트를 표현한 문자열을 할당하는 함수
    fn stringify(&self) -> String{
        match *self{
            Cons(head, ref tail) => {
                // 'format!'은 'print!'와 유사하지만 
                // 콘솔에 글자를 띄우는 대신 heap에 문자열을 할당한다. 
                format!("{}, {}", head, tail.stringify())
            },
            Nil =>{
                format!("Nil")
            },
        }
    }
}

fn main(){
    // 빈 리스트를 생성한다. 
    let mut list = List::new();

    // 몇 원소들을 prepend 시켜준다. 
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // list의 마지막 상태를 보여준다. 
    println!("linked list has length {}", list.len());
    println!("{}", list.stringify());
    }
