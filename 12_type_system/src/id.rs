// 泛型函数
// 在声明一个函数的时候，我们还可以不指定具体的参数或返回值的类型，而是由泛型参数来代替。
// 对于泛型函数，Rust 会进行单态化（Monomorphization）处理，
// 也就是在编译时，把所有用到的泛型函数的泛型参数展开，生成若干个函数。
// 所以，刚才的 id() 编译后会得到一个处理后的多个版本

// 单态化的好处是，泛型函数的调用是静态分派（static dispatch），
// 在编译时就一一对应，既保有多态的灵活性，又没有任何效率的损失，和普通函数调用一样高效。

// 但是对比刚才编译会展开的代码也能很清楚看出来，
// 单态化有很明显的坏处，就是编译速度很慢，
// 一个泛型函数，编译器需要找到所有用到的不同类型，
// 一个个编译，所以 Rust 编译代码的速度总被人吐槽，
// 这和单态化脱不开干系（另一个重要因素是宏）。


// 因为单态化，代码以二进制分发会损失泛型的信息。
// 如果我写了一个库，提供了如上的 id() 函数，
// 使用这个库的开发者如果拿到的是二进制，
// 那么这个二进制中必须带有原始的泛型函数，
// 才能正确调用。但单态化之后，原本的泛型信息就被丢弃了。

fn id<T>(x: T) -> T {
    return x;
}

fn main() {
    let int = id(10);
    let string = id("Tyr");
    println!("{}, {}", int, string);
}