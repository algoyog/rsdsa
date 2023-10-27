use std::fmt::{Display, Formatter};
use std::num::NonZeroI8;

#[derive(Debug)]
pub struct Node<T>{
   pub val: T,
   pub next: Option<Box<Node<T>>>,
   pub prev: Option<Box<Node<T>>>
}

pub fn init_linkedlist<T>(inputval: T) -> Node<T> {
    let val = inputval;
    let n1 = Node{ val: val, next: None, prev: None };
    println!("Initializing linked list :");
    return n1;
}

pub fn print(){

    println!("Printing Linked List");
}