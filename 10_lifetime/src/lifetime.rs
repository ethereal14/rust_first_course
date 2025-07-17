// 如果一个值的生命周期贯穿整个进程的生命周期，那么我们就称这种生命周期为静态生命周期。

// 当值拥有静态生命周期，其引用也具有静态生命周期。
// 我们在表述这种引用的时候，可以用 'static 来表示。
// 比如： &'static str 代表这是一个具有静态生命周期的字符串引用。

// 如果一个值是在某个作用域中定义的，也就是说它被创建在栈上或者堆上，那么其生命周期是动态的。
//  'a 、'b 或者 'hello 这样的小写字符或者字符串来表述。
// ' 后面具体是什么名字不重要，它代表某一段动态的生命周期，
// 其中， &'a str 和 &'b str 表示这两个字符串引用的生命周期可能不一致。
fn main() {
    println!("Hello, world!");

    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result = max(&s1, &s2);

    println!("bigger one: {}", result);
}

fn max(s1: &str, s2: &str2) -> &str {
    if s1 > s2 { s1 } else { s2 }
}
