// 把一个本地变量的引用存入一个可变数组中
fn main() {
    let mut data: Vec<&u32> = Vec::new();
    let v = 42;
    data.push(&v);

    println!("data: {:?}", data);
}
