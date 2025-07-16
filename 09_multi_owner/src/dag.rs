// 现在我们用 Rc 来实现之前无法实现的 DAG。
// 假设 Node 就只包含 id 和指向下游（downstream）的指针，
// 因为 DAG 中的一个节点可能被多个其它节点指向，所以我们使用 Rc 来表述它；
// 一个节点可能没有下游节点，所以我们用 Option> 来表述它。
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn updae_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&mut self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node5 = Node::new(5);
    let node4 = Node::new(4);
    node3.updae_downstream(Rc::new(node4));
    

    node1.updae_downstream(Rc::new(node3));
    node2.updae_downstream(node1.get_downstream().unwrap());


    println!("node:{:?}, node2:{:?}", node1, node2);

    // let node5 = Node::new(5);
    // let node3 = node1.get_downstream().unwrap();
    // 它无法编译通过，编译器会告诉你“node3 cannot borrow as mutable”。
    // 因为 Rc 是一个只读的引用计数器
    // node3.updae_downstream(Rc::new(node5));

    // println!("node1: {:?}, node2:{:?}", node1, node2);
}
