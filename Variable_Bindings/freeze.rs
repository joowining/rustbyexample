fn main(){
    let mut _mutable_integer = 7i32;

    {
        // _mutable_integer를 immutable shadow 
        let _mutable_integer = _mutable_integer;

        // 이제 _mutable_integer는 이 scope 내에서 freeze 된다. 
        // error! 다음 코드는 실행 시 에러가 난다. 
        // _mutable_integer = 50;
    }
    // _mutable_integer는 더 이상 frozen하지 않아서 변경 가능하다. 
    _mutable_integer = 3;
}
