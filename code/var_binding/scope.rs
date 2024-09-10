/**
 * 作用域仅限于{}代码块中，不同于部分其他语言，rust内部代码块可以复用外部代码块的变量名字
 **/
fn main() {

    // This binding lives in the main function
    let long_lived_binding = 1;
    let shadowed_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        let shadowed_binding = "abc";
        println!("shadowed_binding inner: {}",shadowed_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);    
    println!("shadowed_binding outer: {}",shadowed_binding);
}