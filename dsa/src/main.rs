mod ds;
use ds::linkedlist;

fn main() {
    println!("Hello, world!");
    linkedlist::print();
    let n = linkedlist::init_linkedlist("in");
    println!("The val returned: {:?}",n.val);
}
