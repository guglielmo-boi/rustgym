use std::collections::VecDeque;
use std::hash::{Hash, self};
use std::ops::{Add};
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
pub mod odd_module
{
    pub const CONSTANT: i32 = 123;
}

pub mod even_module
{
    pub const CONSTANT: i32 = 246;
}

pub mod getter_function
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
        ret.bits.resize(l as usize, false);

        let mut i: usize = 0;

        while num > 0 && i < l as usize {
            ret.bits[i] = (num % 2 != 0);
            num /= 2;

            i += 1;
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

impl<T> Node<T>
{
    fn get_element(&self) -> &T {
        &self.element
    }

    fn set_prev(&mut self, node: Option<Rc<RefCell<Node<T>>>>) {
        self.prev = node;
    }

    fn set_next(&mut self, node: Option<Rc<RefCell<Node<T>>>>) {
        self.next = node;
    }
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
                
                    if let Some(next) = current.clone().borrow().next.clone() {
                        current = next.clone();
                    }
                }
            }
        }

        Ok(())
    }
}

impl<T: Display + Default + Clone> List<T>
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

                    if let Some(next) = current.clone().borrow().next.clone() {
                        current = next.clone();
                    }
                }
            }
        }
    }

    fn push(&mut self, element: T) {
        let new_node = Rc::new(RefCell::new(Node{element, prev: None, next: self.head.clone()}));

        if let Some(head) = &mut self.head {
            head.borrow_mut().set_prev(Some(new_node.clone()));
        }

        self.head = Some(new_node.clone());

        if self.size == 0 {
            self.tail = Some(new_node.clone());
        }

        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None 
        } else if self.size == 1 {
            let mut ret = T::default();

            if let Some(head) = &mut self.head {
                ret = head.borrow_mut().get_element().clone();
            }

            self.head = None;
            self.tail = None;
            self.size -= 1;

            Some(ret)
        } else {
            let mut ret = T::default();
            let mut node = self.head.clone();

            if let Some(head) = &mut self.tail {
                ret = head.borrow_mut().get_element().clone();

                if let Some(next) = head.borrow_mut().next.clone() {
                    next.borrow_mut().set_next(None);
                    node = Some(next.clone());
                }
            }

            self.head = node;
            self.size -= 1;

            Some(ret)
        }
    }

    fn push_back(&mut self, element: T) {
        let new_node = Rc::new(RefCell::new(Node{element, prev: self.tail.clone(), next: None}));

        if self.size == 0 {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else if self.size == 1 {
            if let Some(head) = &mut self.head {
                head.borrow_mut().set_next(Some(new_node.clone()));
            }

            self.tail = Some(new_node.clone());
        } else {
            if let Some(tail) = &mut self.tail {
                tail.borrow_mut().set_next(Some(new_node.clone()));
            }

            self.tail = Some(new_node.clone());
        }

        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 {
            None 
        } else if self.size == 1 {
            let mut ret = T::default();

            if let Some(head) = &mut self.head {
                ret = head.borrow_mut().get_element().clone();
            }

            self.head = None;
            self.tail = None;
            self.size -= 1;

            Some(ret)
        } else {
            let mut ret = T::default();
            let mut node = self.tail.clone();

            if let Some(tail) = &mut self.tail {
                ret = tail.borrow_mut().get_element().clone();

                if let Some(prev) = tail.borrow_mut().prev.clone() {
                    prev.borrow_mut().set_next(None);
                    node = Some(prev.clone());
                }
            }

            self.tail = node;
            self.size -= 1;

            Some(ret)
        }

    }

}


/*
    6. Write the necessary structs to represent an oriented graph generic over T , where T
    implements Hash , PartialEq and Eq .
    Node , with a value of type T and a vector of adjacent nodes
    Graph , with a vector of nodes
    Then, implement the following methods for Node :
    new , which creates a new Node with the given value and the given vector of adjacents
    get_value , which returns a reference to the value of the node
    Implement Debug for Node , so that it prints the value of the node and the values of its
    adjacents.
    For example, if the node has value 1 and its adjacents are 2 and 3 , it should print:
    [value: 1, adjacents: [2, 3]]
    Then, implement the following methods for Graph :
    new , which creates a Graph from a vector of nodes, with the respective adjacents set
    dfs , which performs a depth-first search on the graph, starting from the given node. It
    returns a vector of nodes, in the order in which they were visited.
*/
type NodeRef<T> = Rc<Node<T>>;

#[derive(PartialEq)]
struct Node<T: Hash + PartialEq + Eq>
{
    value: T,
    adjacents: Vec<Rc<Node<T>>>
}

impl<T: Hash + PartialEq + Eq> Node<T>
{
    fn new(value: T, adjacents: Vec<Rc<Node<T>>>) -> Node<T> {
        Node {
            value,
            adjacents
        }
    }

    fn get_value(&self) -> &T {
        &self.value
    }
}

impl<T: Hash + PartialEq + Eq + Debug + Copy> Debug for Node<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let adj: Vec<T> = self.adjacents.iter().map(|node| node.value).collect();
        write!(f, "[value: {:?}, adjacents: \"{:?}\"]", self.value, adj)
    }
}

struct Graph<T: Hash + PartialEq + Eq>
{
    nodes: Vec<Rc<Node<T>>>
}

impl<T: Hash + PartialEq + Eq> Graph<T>
{
    fn new(nodes: Vec<Rc<Node<T>>>) -> Graph<T> {
        Graph {
            nodes
        }
    }

    fn dfs_rec(&self, visited: &mut Vec<Rc<Node<T>>>, node: Rc<Node<T>>) {
        visited.push(node.clone());

        for next in node.adjacents.clone().iter() {
            if !visited.contains(&next) {
                self.dfs_rec(visited, next.clone());
            }
        }
    }

    fn dfs(&self, node: Rc<Node<T>>) -> Vec<Rc<Node<T>>> {
        let mut ret = Vec::new();

        self.dfs_rec(&mut ret, node);

        ret
    }
}


/*
    7. Write a trait Task that define a method execute(&self)->usize .
    implement the Task trait for the following structs:
    SumTask is a struct with a method new(n1: usize, n2: usize) were executing task returns
    the sum of n1 and n2
    LenTask is a struct with a method new(s: String) were executing task returns
    the len of s
    Write two structs: Tasker and Executer , that interact following this protocol:
    At any given time any number of tasker and executer can be linked together.
    Tasker can ask for a task to be scheduled using the method schedule_task(&mut self, task: ...) that
    take as input a
    box with inside an object that implements Task
    Executer can execute a task using the method execute_task(&mut self)->Option<usize> . this method can
    fail if no task is scheduled
    Tasks are executed inf a FIFO queue
    Tasker has a method new that return am instance with an empty queue, linked to no one.
    Tasker has a method get_tasker(&self)->Tasker that return a new Tasker linked with self.
    Tasker has a method get_executer(&self)->Executer that return a new Executer linked with self.
*/
trait Task
{
    fn execute(&self) -> usize;
}

struct SumTask
{
    n1: usize,
    n2: usize
}

impl SumTask
{
    fn new(n1: usize, n2: usize) -> SumTask {
        SumTask {
            n1,
            n2
        }
    }
}

impl Task for SumTask
{
    fn execute(&self) -> usize {
        self.n1 + self.n2
    }
}

struct LenTask
{
    s: String
}

impl LenTask
{
    fn new(s: String) -> LenTask {
        LenTask {
            s
        }
    }
}

impl Task for LenTask
{
    fn execute(&self) -> usize {
        self.s.len()
    }
}

struct Tasker
{
    tasks: Rc<RefCell<VecDeque<Box<dyn Task>>>>
}

impl Tasker
{
    fn new() -> Tasker {
        Tasker {
            tasks: Rc::new(RefCell::new(VecDeque::new()))
        }
    }

    fn schedule_task(&mut self, task: Box<dyn Task>) {
        self.tasks.borrow_mut().push_back(task);
    }

    fn get_tasker(&self) -> Tasker {
        Tasker {
            tasks: self.tasks.clone()
        }
    }

    fn get_executer(&self) -> Executer {
        Executer {
            tasks: self.tasks.clone()
        }
    }
}

struct Executer
{
    tasks: Rc<RefCell<VecDeque<Box<dyn Task>>>>
}

impl Executer
{
    fn execute_task(&mut self) -> Option<usize> {
        if let Some(ret) = self.tasks.borrow_mut().pop_front() {
            Some(ret.execute())
        } else {
            None
        }
    }
}

fn main()
{
    
}