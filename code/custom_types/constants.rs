static LANGUAGE: &str = "Rust";
const THREADHOLD: i32 = 10;

fn is_big(arg: i32) -> bool {
    arg > THREADHOLD
}

fn main() {
    let n = 6;
    println!("This is {}", LANGUAGE);
    println!("The threadhold is {}", THREADHOLD);
    println!(
        "{} is {} then THREADHOLD",
        n,
        if is_big(n) { "bigger" } else { "small" }
    );
    // THREADHOLD = 5;//Error! Cannot modify a `const`.
}
