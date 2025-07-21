// 用泛型实现参数多态
// 泛型数据结构

// 在定义刚才这个泛型数据结构的时候，你有没有这样的感觉，有点像在定义函数：
// 函数，是把重复代码中的参数抽取出来，使其更加通用，调用函数的时候，根据参数的不同，我们得到不同的结果；
// 而泛型，是把重复数据结构中的参数抽取出来，在使用泛型类型时，根据不同的参数，我们会得到不同的具体类型。

use std::fs::File;
use std::io::{BufReader, Read, Result};

// 定义一个带有泛型参数R的reader，此处不限制R
struct MyReader<R> {
    reader: R,
    buf: String,
}

// 实现new函数是，不需要限制R
impl<R> MyReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::with_capacity(1024),
        }
    }
}

// 定义process时，需要用到R的方法，此时限制R必须实现Read trait
impl<R> MyReader<R>
where
    R: Read,
{
    pub fn process(&mut self) -> Result<usize> {
        self.reader.read_to_string(&mut self.buf)
    }
}

fn main() {
    let f = File::open("/etc/hosts").unwrap();
    let mut reader = MyReader::new(BufReader::new(f));

    let size = reader.process().unwrap();
    println!("total size read: {}", size);
}
