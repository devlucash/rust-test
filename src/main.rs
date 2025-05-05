fn main() {
    let my_string = "hello, world";
    println!("{}", my_string);
    // create linked list
    let mut list = LinkedList::new();   // creates a LL

    // add to linked list
    list.insert(10);
    list.insert(20);
    list.insert(30);

    // print linked list
    list.print();

    // remove from linked list
    list.remove();

    // print to linked list
    list.print();


}
// Linked List class
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}
// Node class
pub struct Node<T> {
    pub val: T,
    pub next: Option<Box<Node<T>>>,
    pub prev: Option<Box<Node<T>>>,
}

// LinkedList implemented methods
impl<T> LinkedList<T> 
where
    T: std::fmt::Display,   // linked list condition of T
{
    // constructor
    pub fn new() -> Self {
        LinkedList { 
            head: None, 
        }
    }


    pub fn insert(&mut self, val: T){
        let new_node = Box::new(Node {
            val, 
            next: self.head.take(),
            prev: None,
        });
        self.head = Some(new_node);
    }

    pub fn remove(&mut self){
        if let Some(node) = self.head.take() {
            self.head = node.next;
        } else {
            println!("List is empty, nothing to remove.");
        }
    }

    pub fn print(&self){
        let mut curr = &self.head;
        while let Some(node) = curr {
            print!("{} -> ", node.val);
            curr = &node.next;
        }
        print!("None\n");
    }
}


/*
    if references self = immutable, read only
    if ref &mut self   = mutable (once), and read and write 

    e.g. 
    let mut x = 5;
    let mut y = &mut x; // will work as its first use of x
    let mut z = &mut x; // wont work, x has already been borrowed

*/
