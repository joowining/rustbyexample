// NanoSecond, Inch, U64는 u64의 새로운 이름들이다. 
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main(){
    // NanoSecond = Inch = U64 = u64 
    let nanoseconds : NanoSecond = 5 as U64;
    let inches : Inches = 2 as U64;

    // 타입 별칭이 어떠한 타입 안정성을 제공해주진 않는다. 
    // 왜냐하면 타입 별칭은 별칭일 뿐 타입이 아니기 때문이다. 
}
