// 在 Rust 下，函数是一等公民，可以作为参数或者返回值。
// 这里 fn(i32) -> i32 是 apply 函数第二个参数的类型，
// 它表明接受一个函数作为参数，这个传入的函数必须是：
// 参数只有一个，且类型为 i32，返回值类型也是 i32。
fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn main() {
    println!("apply square:{}", apply(2, square));
    println!("apply cube:{}", apply(2, cube));
}
