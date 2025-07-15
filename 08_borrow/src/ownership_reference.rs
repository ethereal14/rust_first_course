fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;

    // 值的地址是什么？引用的地址又是什么？
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1:{:p}",
        &data, data1, &&data, &data[3]
    );

    println!("sum of data1:{}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of iterms: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}

fn sum(data: &Vec<u32>) -> u32 {
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);

    data.iter().fold(0, |acc, x| acc + x)
}
