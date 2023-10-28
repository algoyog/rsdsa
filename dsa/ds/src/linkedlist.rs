#[derive(Debug)]
pub struct Node<T>{
   pub val: T,
   pub next: Option<Box<Node<T>>>,
   pub prev: Option<Box<Node<T>>>
}



pub fn print(){
    dbg!("Inside LL");
}