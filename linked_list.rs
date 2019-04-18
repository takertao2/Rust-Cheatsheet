use crate::List::*;

enum List {
    // Node: a value and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {

    fn new() -> List {
        Nil // Nil has type list
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self)) // Cons has type list
    }

    fn len(&self) -> u32 {
        // self matched because behavior depends on the variant of self
        // self has type &List, *self has type List
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0 // Base Case: An empty list has zero length
        }
    }

    // Return as a heap-allocated string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {

    let mut list = List::new();
    list = list.prepend(14);
    list = list.prepend(28);
    list = list.prepend(57);
    println!("Linked list: {}", list.stringify());
    println!("Length: {}", list.len());

}
