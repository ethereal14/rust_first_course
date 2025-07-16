// 有时候我们希望能够绕开这个编译时的检查，
// 对并未声明成 mut 的值或者引用，也想进行修改。
// 也就是说，在编译器的眼里，值是只读的，
// 但是在运行时，这个值可以得到可变借用，
// 从而修改内部的数据，这就是 RefCell 的用武之地。

use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1);

    {
        let mut v = data.borrow_mut();
        *v += 1;
    }
    println!("data:{:?}", data.borrow());
}
