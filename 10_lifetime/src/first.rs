// 虽然我们没有做任何生命周期的标注，
// 但编译器会通过一些简单的规则为函数自动添加标注：
// 所有引用类型的参数都有独立的生命周期 'a 、'b 等。
// 如果只有一个引用型输入，它的生命周期会赋给所有输出。
// 如果有多个引用类型的参数，其中一个是 self，那么它的生命周期会赋给所有输出。


// max() 函数为什么编译器无法处理呢？
// 按照规则 1， 我们可以对 max() 函数的参数 s1 和 s2 分别标注'a 和'b ，
// 但是返回值如何标注？是 'a 还是'b 呢？这里的冲突，编译器无能为力。
fn main() {
    let s1 = "Hello world";

    println!("first word of s1: {}", first(&s1));
}

fn first(s: &str) -> &str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}
