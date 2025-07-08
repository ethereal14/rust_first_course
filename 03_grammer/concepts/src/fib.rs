// 程序的基本控制流程分为以下几种，我们应该都很熟悉了，
// 重点看如何在 Rust 中运行。

// 顺序执行就是一行行代码往下执行。
// 在执行的过程中，遇到函数，会发生函数调用。
// 函数调用是代码在执行过程中，调用另一个函数，
// 跳入其上下文执行，直到返回。

// Rust 的循环和大部分语言都一致，
// 支持死循环 loop、条件循环 while，以及对迭代器的循环 for。
// 循环可以通过 break 提前终止，或者 continue 来跳到下一轮循环。

// 满足某个条件时会跳转， Rust 支持分支跳转、模式匹配、错误跳转和异步跳转。
// 分支跳转就是我们熟悉的 if/else；
// Rust 的模式匹配可以通过匹配表达式或者值的某部分的内容，来进行分支跳转；
//
// 在错误跳转中，当调用的函数返回错误时，Rust 会提前终止当前函数的执行，向上一层返回错误。

// 在 Rust 的异步跳转中 ，当 async 函数执行 await 时，
// 程序当前上下文可能被阻塞，执行流程会跳转到另一个异步任务执行，直至 await 不再阻塞。

// 通过斐波那契数列，使用 if 和 loop / while / for 这几种循环，来实现程序的基本控制流程

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);
    }
}

// Rust 的 for 循环可以用于任何实现了 IntoIterator trait 的数据结构。
// 在执行过程中，IntoIterator 会生成一个迭代器，for 循环不断从迭代器中取值，直到迭代器返回 None 为止。
// 因而，for 循环实际上只是一个语法糖，编译器会将其展开使用 loop 循环对迭代器进行循环访问，直至返回 None。
fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
