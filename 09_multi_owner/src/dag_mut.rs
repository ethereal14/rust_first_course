use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn updae_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&mut self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.updae_downstream(Rc::new(RefCell::new(node4)));

    node1.updae_downstream(Rc::new(RefCell::new(node3)));
    node2.updae_downstream(node1.get_downstream().unwrap());

    println!("node:{:?}, node2:{:?}", node1, node2);

    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    // 获得可变引用，来修改 downstream
    node3.borrow_mut().downstream = Some(Rc::new(RefCell::new(node5)));
    println!("node1: {:?}, node2:{:?}", node1, node2);
}

// Arc 和 Mutex/RwLock
// 多个线程访问同一块内存的问题，是否也可以使用 Rc 来处理呢？
// 不行。因为 Rc 为了性能，使用的不是线程安全的引用计数器。
// 因此，我们需要另一个引用计数的智能指针：Arc，它实现了线程安全的引用计数器。
// Arc 内部的引用计数使用了 Atomic Usize ，而非普通的 usize。
// Atomic Usize 是 usize 的原子类型，它使用了 CPU 的特殊指令，来保证多线程下的安全