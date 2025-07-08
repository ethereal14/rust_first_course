// 数据结构是程序的核心组成部分，在对复杂的问题进行建模时，我们就要自定义数据结构。
// Rust 非常强大，可以用 struct 定义结构体，用 enum 定义标签联合体（tagged union），
// 还可以像 Python 一样随手定义元组（tuple）类型

// 比如我们可以这样定义一个聊天服务的数据结构

use std::os::unix::raw::uid_t;

// 一个枚举类型，在 Rust 下，使用 enum 可以定义类似 C 的枚举类型
#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

// struct 的特殊形式，称为元组结构体。它的域都是匿名的，可以用索引访问，适用于简单的结构体。
#[derive(Debug, Clone, Copy)]
struct UserId(u64);

// Clone 让数据结构可以被复制，而 Copy 则让数据结构可以在参数传递的时候自动按字节拷贝
#[derive(Debug, Clone, Copy)]
struct TopicId(u64);

// 标准的结构体，可以把任何类型组合在结构体里使用
#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

// 标准的标签联合体，它定义了三种事件：Join、Leave、Message。每种事件都有自己的数据结构。
#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn main() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };
    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));

    println!(
        "event1: {:?}, event2:{:?}, event3:{:?}",
        event1, event2, event3
    );

    process_event(&event1);
    process_event(&event2);
    process_event(&event3);
}

// Rust 的模式匹配吸取了函数式编程语言的优点，强大优雅且效率很高。
// 它可以用于 struct / enum 中匹配部分或者全部内容，
// 比如上文中我们设计的数据结构 Event，可以这样匹配

fn process_event(ev: &Event) {
    match ev {
        Event::Join((uid, _tid)) => println!("user: {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user: {:?}, left: {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
    // 除了使用 match 关键字做模式匹配外，
    // 我们还可以用 if let / while let 做简单的匹配，
    // 如果上面的代码我们只关心 Event::Message，可以这么写

    if let Event::Message((_, _, msg)) = ev {
        println!("broadcast: {}", msg);
    }
}
