 fn main(){
    let shadowed_binding = 1;
    {
        println!("before using shadowed: {}", shadowed_binding);
        // 이 바인딩은 바깥 바인딩을 shadow 한다. 
        let shadowed_binding = "abc";
        println!("shadowed in inner block : {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // 이 바인딩은 이전의 바인딩을 shadow 한다. 
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
 }
