// enum 我们之前讲过，在 Rust 下它是一个标签联合体（tagged union），它的大小是标签的大小，加上最大类型的长度。
// 一般而言，64 位 CPU 下，enum 的最大长度是：最大类型的长度 + 8，
// 因为 64 位 CPU 的最大对齐是 64bit，也就是 8 个字节。

// Rust 编译器会对 enum 做一些额外的优化，让某些常用结构的内存布局更紧凑

use std::collections::HashMap;
use std::mem::size_of;

enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}

// 这是一个声明宏，它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
macro_rules! show_size {
    (header) => {
        println!(
            "{:<24} {:>4}      {}      {}",
            "Type", "T", "Option<T>", "Result<T, io:Error>"
        );
        println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:4}      {:8}      {:12}",
            stringify!($t),
            size_of::<$t>(),
            size_of::<Option<$t>>(),
            size_of::<Result<$t, std::io::Error>>(),
        );
    };
}

// 这段代码包含了一个声明宏（declarative macro）show_size，我们先不必管它。
// 运行这段代码时，你会发现，Option 配合带有引用类型的数据结构，
// 比如 &u8、Box、Vec、HashMap ，没有额外占用空间，这就很有意思了。

// 对于 Option 结构而言，它的 tag 只有两种情况：0 或 1， 
// tag 为 0 时，表示 None，tag 为 1 时，表示 Some。

// 正常来说，当我们把它和一个引用放在一起时，虽然 tag 只占 1 个 bit，
// 但 64 位 CPU 下，引用结构的对齐是 8，所以它自己加上额外的 padding，
// 会占据 8 个字节，一共 16 字节，这非常浪费内存。怎么办呢？
// Rust 是这么处理的，我们知道，引用类型的第一个域是个指针，
// 而指针是不可能等于 0 的，但是我们可以复用这个指针：
// 当其为 0 时，表示 None，否则是 Some，减少了内存占用，
// 这是个非常巧妙的优化，我们可以学习。
fn main() {
    show_size!(header);
    show_size!(u8);
    show_size!(f64);
    show_size!(&u8);
    show_size!(Box<u8>);
    show_size!(&[u8]);

    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);
}
