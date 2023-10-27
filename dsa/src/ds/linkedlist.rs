

struct Node<T>{
    val: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>
}


pub fn print(){
    let n1 = Node{ val: ("Hello"), next: None, prev: None };
    println!("Printing Linked List : {}", n1.val);
}