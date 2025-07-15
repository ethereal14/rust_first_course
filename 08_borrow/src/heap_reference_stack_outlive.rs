// 把一个本地变量的引用存入一个可变数组中
fn main() {
    let mut data: Vec<&u32> = Vec::new();
    push_local_ref(&mut data);

    println!("data: {:?}", data);
}

fn push_local_ref(data: &mut Vec<&u32>) {
    let v = 42;
    // v 生命周期不够长，如果不注释掉会编译不过
    // data.push(&v);
}
