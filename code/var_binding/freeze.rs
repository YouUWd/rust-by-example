/**
 * rust let 变量默认是不可变的，当一个外部可变变量被内部代码块scope声明为不可变后，在当前scope下就失去了mut属性，变为不可变，直到scope结束
 **/
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        _mutable_integer = 50;
        // FIXME ^ Comment out this line

        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}