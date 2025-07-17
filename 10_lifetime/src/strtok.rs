fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1:{}, s:{}", hello, s1, s);
}

// pub fn strtok<'b, 'a>(s: &'b mut &'a str, delimiter: char) -> &'a str {
//     if let Some(i) = s.find(delimiter) {
//         let prefix = &s[..i];
//         let suffix = &s[(i + delimiter.len_utf8())..];
//         *s = suffix;
//         prefix
//     } else {
//         let prefix = *s;
//         *s = "";
//         prefix
//     }
// }

// 因为返回值的生命周期跟字符串引用有关，
// 我们只为这部分的约束添加标注就可以了，
// 剩下的标注交给编译器自动添加，
// 所以代码也可以简化成如下这样，
// 让编译器将其扩展成上面的形式：
pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}
