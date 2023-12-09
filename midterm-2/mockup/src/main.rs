use std::ops::Add;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::cell::RefCell;

/*
    1. define an i32 constant named "CONSTANT" inside a module named "odd_module" and assign to it the value 123
    define an i32 constant named "CONSTANT" inside a module named "even_module" and assign to it the value
    246
    define a public function "get_constant" inside the module "getter_function" that take as input an u32 named
    "value", and return
    the constant inside "odd_module" if "value" is odd. otherwise it returns the constant inside "even_module"
    just to avoid confusion remember that in Italian: odd = dispari, even = pari
*/
mod odd_module
{
    pub const CONSTANT: i32 = 123;
}

mod even_module
{
    pub const CONSTANT: i32 = 246;
}

mod getter_function
{
    use crate::{even_module, odd_module};

    pub fn get_constant(value: u32) -> i32 {
        if value % 2 == 0 {
            even_module::CONSTANT
        } else {
            odd_module::CONSTANT
        }
    }
}


/*
    2. define a trait CloneAndDouble with a function clone_and_double(&self)->Self
    the function clone_and_double clone the item and double it.
    Implement the trait for all items that implement the traits Clone and Add (use a simple addition to double)
*/
trait CloneAndDouble
{
    fn clone_and_double(&self) -> Self;
}

impl<T: Clone + Add<Output = T>> CloneAndDouble for T
{
    fn clone_and_double(&self) -> Self {
        self.clone() + self.clone()
    }
}


/*
    3. The trait Unknown defines a method serialize that returns the implementer's String representation.
    implement it for i32
    implement it for String
    implement it for Vec<T> , where T implements Debug
    write a function get_vec that returns an empty vec of Unknown data
    write a function print_vec that takes as input a reference of a vec of Unknown data and prints its content
*/
trait Unknown
{
    fn serialize(&self) -> String;
}

impl Unknown for i32
{
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl Unknown for String
{
    fn serialize(&self) -> String {
        self.clone()
    }
}

impl<T: Debug> Unknown for Vec<T>
{
    fn serialize(&self) -> String {
        format!("{:?}", self)
    }
}

fn get_vec() -> Vec<Box<dyn Unknown>> {
    Vec::new()
}

fn print_vec(vec: &Vec<Box<dyn Unknown>>) {
    for item in vec {
        println!("{}", item.serialize());
    }
}


/*
    4. Write a struct `BinIter` that implements `Iterator` over `bool`s.
    - `BinIter` must have a function `new` that takes as input `n` the number and `l` the length.
    - The iterator must yield bits according to the binary form of `n`, after returning the `l`-th bit the iterator stops.
    - The bits yielded must be in "little-endian" order, so the most significant bit must be yielded last.
*/
struct BinIter
{
    bits: Vec<bool>,
    _index: usize
}

impl BinIter
{
    fn new(n: u128, l: u32) -> BinIter {
        let mut num = n;
        let mut ret = BinIter{bits: Vec::new(), _index: 0};

        while num > 0 {
            ret.bits.push(num % 2 != 0);
            num /= 2;
        }

        ret
    }
}

impl Iterator for BinIter
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self._index == self.bits.len() {
            self._index = 0;
            None
        } else {
            let index = self._index;
            self._index += 1;
            Some(self.bits[index])
        }
    }
}


/*
    5. Implement a doubly linked list
    Create the necessary structs to represent it
    - Node<T> with an element of type T and two fields, prev and next, both of type Option<Rc<RefCell<Node<T>>>>.
    - List<T> with two fields, head and tail, both of type Option<Rc<RefCell<Node<T>>>>, and a size field of type usize.
    Implement the following traits for Node<T>:
    - PartialEq that compares the elements of two nodes.
    - Display that prints the element of a node.
    Implement the following traits for List<T>:
    - PartialEq that checks if two lists are equal, by comparing the elements of the nodes, one by one.
    - Debug that prints the elements of the list.
    Implement the following methods for List<T>:
    - new() that creates a new empty list.
    - print_list(&self) that prints the elements of the list.
    - push(&mut self, element: T) that adds an element to the front of the list.
    - pop(&mut self) -> Option<T> that removes an element from the front of the list.
    - push_back(&mut self, element: T) that adds an element to the back of the list.
    - pop_back(&mut self) -> Option<T> that removes an element from the back of the list.
*/

struct Node<T>
{
    element: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>
}

impl<T: PartialEq> PartialEq for Node<T>
{
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}

impl<T: Display> std::fmt::Debug for Node<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

struct List<T>
{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize
}

impl<T: PartialEq> PartialEq for List<T>
{
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            false
        } else {
            if self.size == 0 {
                false
            } else {
                if let Some(mut self_current) = self.head.clone() {
                    if let Some(mut other_current) = other.head.clone() {
                        for _ in 0..self.size {
                            if *self_current.borrow() != *other_current.borrow() {
                                return false;
                            }

                            if let Some(self_next) = self_current.clone().borrow().next.clone() {
                                self_current = self_next.clone();
                            }

                            if let Some(other_next) = other_current.clone().borrow().next.clone() {
                                other_current = other_next.clone();
                            }
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }

                true
            }
        }
    }
}

impl<T: Display> Debug for List<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.size > 0 {
            if let Some(mut current) = self.head.clone() {
                for _ in 0..self.size {
                    println!("{}", current.borrow().element);
                }

                if let Some(next) = current.clone().borrow().next.clone() {
                    current = next.clone();
                }
            }
        }

        Ok(())
    }
}

impl<T: Display + Clone> List<T>
{
    fn new() -> List<T> {
        List {
            head: None,
            tail: None,
            size: 0
        }
    }

    fn print_list(&self) {
        if self.size > 0 {
            if let Some(mut current) = self.head.clone() {
                for _ in 0..self.size {
                    println!("{}", current.borrow().element);
                }

                if let Some(next) = current.clone().borrow().next.clone() {
                    current = next.clone();
                }
            }
        }
    }

    fn push(&mut self, element: T) {
        let new_node = Rc::new(RefCell::new(Node{element, prev: None, next: self.head.clone()}));
        self.head = Some(new_node.clone());

        if self.size == 0 {
            self.tail = Some(new_node.clone());
        }

        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.clone() {
            let e = head.clone().borrow().element.clone();
            self.head = head.borrow().next.clone();

            if self.size == 1 {
                self.tail = None;
            }

            self.size -= 1;

            return Some(e);
        }

        None
    }

    fn push_back(&mut self, element: T) {
        let new_node = Rc::new(RefCell::new(Node{element, prev: self.tail.clone(), next: None}));
        self.tail = Some(new_node.clone());

        if self.size == 0 {
            self.head = Some(new_node.clone());
        }

        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        if let Some(tail) = self.tail.clone() {
            let e = tail.clone().borrow().element.clone();
            self.tail = tail.borrow().next.clone();

            if self.size == 1 {
                self.head = None;
            }

            self.size -= 1;

            return Some(e);
        }

        None
    }

}

fn main()
{
    println!("odd constant: {}",odd_module::CONSTANT);
    println!("even constant: {}",even_module::CONSTANT);
    println!("test function: {}", getter_function::get_constant(105));

    println!("{}",1u8.clone_and_double());
    println!("{}",1i8.clone_and_double());
    println!("{}",2u16.clone_and_double());
    println!("{}",2i16.clone_and_double());
    println!("{}",3u32.clone_and_double());
    println!("{}",3i32.clone_and_double());
    println!("{}",4u64.clone_and_double());
    println!("{}",4i64.clone_and_double());

    let mut v = get_vec();
    v.push(Box::new("hiii!".to_string()));
    v.push(Box::new(-587));
    v.push(Box::new("xyz".to_string()));
    v.push(Box::new(vec![4, 5, 6]));
    print_vec(&v);

    for n in
    BinIter::new(4641312598359562305508665788689819792,128) {
    print!("{}", if n { 1 } else { 0 })
    }
    println!();
    for n in BinIter::new(18956403638425120546, 64)
    {
    print!("{}", if n { 1 } else { 0 })
    }
    println!();
    for n in BinIter::new(15, 4) {
    print!("{}", if n { 1 } else { 0 })
    }

    // let mut list: List<i32> = List::new();
    // list.push_back(1);
    // list.push_back(2);
    // list.push_back(3);
    // list.print_list();
    // debug_assert_eq!(list.pop_back(), Some(3));
    // list.print_list();
    // debug_assert_eq!(list.pop_back(), Some(2));
    // list.print_list();
    // debug_assert_eq!(list.pop_back(), Some(1));
    // list.print_list();
    // debug_assert_eq!(list.pop_back(), None);
    // debug_assert_eq!(list.size, 0);
    // debug_assert_eq!(list.head, None);
    // println!("{:?}", list.head);
    // debug_assert_eq!(list.tail, None);
    // println!("{:?}", list.tail);
}
