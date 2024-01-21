use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

/*
    1. define the Doublable trait with a method gimme_double implement Doublable for i32, gimme_double returns a new i32 
    that is twice self implement Doublable for String, gimme_double returns a new String that is self concatenated with self
    implement a function printdouble that takes a Doublable and prints the argument and its gimme_double using the ":?" 
    formatter it behaves as the example: doubling 5 is 10 doubling "what" is "whatwhat".
*/

trait Doublable: Debug
{
    fn gimme_double(&self) -> Self;
}

impl Doublable for i32
{
    fn gimme_double(&self) -> Self {
        self + self
    }
}

impl Doublable for String
{
    fn gimme_double(&self) -> Self {
        let mut ret = self.clone();
        ret.push_str(self);
        ret
    }
}

fn printdouble(doublable: impl Doublable) {
    let doubled = doublable.gimme_double();
    println!("doubling {:?} is {:?}", doublable, doubled);
}


/*
    2. Define a struct Wrapper that contains a field v of type Vec<i32> define an iterator for Wrapper to cycle over 
    the elements of the vector the iterator will skip every other element, effectively accessing only those at odd index 
    in the inner vector (the first element is at index 0).
*/
struct Wrapper
{
    v: Vec<i32>
}

struct OddIndexIterator<'a>
{
    inner_iter: std::slice::Iter<'a, i32>,
}

impl Wrapper 
{
    fn iter(&self) -> OddIndexIterator {
        OddIndexIterator {
            inner_iter: self.v.iter(),
        }
    }
}

impl<'a> Iterator for OddIndexIterator<'a> 
{
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().and_then(|_| self.inner_iter.next())
    }
}

/*
    3. write a function basicbox_sum that takes a vector of Strings and returns a vector of Boxes of usizes the returned vector 
    contains all the lengths of the input vector followed by a final element that sums all the previous lengths
*/
fn basicbox_sum(v: Vec<String>) -> Vec<Box<usize>> {
    let mut ret = Vec::new();
    let mut total = 0;

    for s in v.iter() {
        ret.push(Box::new(s.len()));
        total += s.len();
    }

    ret.push(Box::new(total));

    ret
}


/*
    4. take the following List and Node structs define these functions and methods for List, each one defines how many points it 
    yields - [0.5] new: returns an empty list - [0.5] size: returns the i32 length of the list - [6] add: takes an element e:T and 
    a position p:i32 where to insert the element in the list and it returns a Result<(),String> The function inserts the element e 
    at position p or returns the Err string "wrong position" if the list has fewer than p elements. That is: adding 16 at position 2 
    to [10,20,30] will return [10,20,16,30]. adding 16 at position 3 to [10,20,30] will return [10,20,30,16]. adding 16 at position 4 
    to [10,20,30] will return [10,20,30]. - [2] prepend: takes an element e:T and adds it at the head of the list - [2] 
    append: takes an element e:T and adds it as the last element of the list - [4] get: takes a position p and returns an optional pointer 
    to the pth T-typed element in the list (That is, a pointer to the element, not a pointer to the Node).
*/
#[derive(Debug)]
pub struct List<T> 
{
    head: Link<T>,
    len: i32
}

impl<T> List<T>
{
    fn new() -> List<T> {
        List {
            head: Link::default(),
            len: 0
        }
    }

    fn size(&self) -> i32 {
        self.len
    }

    pub fn add(&mut self, e: T, p: i32) -> Result<(), String> {
        if p > self.len {
            Err(String::from("wrong position"))
        } else {
            let mut new_node = Box::new(Node{elem: e, next: None});
    
            let mut node = &mut self.head;

            for _ in 0..p {
                node = &mut node.as_mut().unwrap().next;
            }
            
            new_node.next = node.take();
            *node = Some(new_node);

            self.len += 1;

            Ok(())
        }
    }

    fn prepend(&mut self, e: T) {
        let new_node = Box::new(Node{elem: e, next: self.head.take()});
        self.head = Some(new_node);
        self.len += 1;
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T>
{
    elem: T,
    next: Link<T>
}

#[derive(Debug)]
struct Content
{
    s : String, 
    b : bool, 
    i : i32
}

impl Content
{
    pub fn new_with(s: String, b: bool, i: i32) -> Content {
        return Content{s, b, i};
    }
}


/*
    5. SameBool is a Trait It has a method samebool that takes a SameBool and it returns a bool
    Content is a struct with an i32 and a bool Two Contents can be compared (<, >, ==) by comparing their i32 field ([2 points])
    Content implements SameBool: the method of the trait returns whether self has the same bool as the parameter ([1] point)
    Define a Graph as a vector of Nodes whose elements are arbitrary T - add a function for creating an empty graph ([1] points)
    When T implements SameBool and PartialOrd, define function add_node that adds a Node to the graph with these connections: - the added node 
    gets as neighbour all nodes in the graph that are < than it - the added node becomes a neighbour of all the nodes with the samebool ([6] points).
*/
type NodeRef<T> = Rc<RefCell<GraphNode<T>>>;
struct GraphNode<T> 
{
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}

impl<T:Debug> Debug for GraphNode<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
    }
}

#[derive(Debug)]
struct Graph<T>
{
    nodes: Vec<NodeRef<T>>,
}

impl<T> Graph<T>
{
    fn new() -> Graph<T> {
        Graph {
            nodes: Vec::new()
        }
    }
}

impl<T: SameBool + PartialOrd> Graph<T>
{
    fn add_node(&mut self, node: T) {
        let mut new_node = Rc::new(RefCell::new(GraphNode {
            inner_value: node,
            adjacent: Vec::new()
        }));

        for node in self.nodes.iter_mut() {
            if node.borrow().inner_value < new_node.borrow().inner_value {
                new_node.borrow_mut().adjacent.push(node.clone());
            }

            if node.borrow().inner_value.samebool(&new_node.borrow().inner_value) {
                node.borrow_mut().adjacent.push(node.clone());
            }
        }

        self.nodes.push(new_node);
    }   
}

pub trait SameBool
{
    fn samebool(&self, other: &Self) -> bool;
}

#[derive(Debug)]
pub struct Content
{
    pub i: i32,
    pub b: bool
}

impl Content
{
    pub fn new_with(i: i32, b: bool) -> Content {
        Content { i, b }
    }
}

impl PartialEq for Content
{
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}

impl PartialOrd for Content
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.i.partial_cmp(&other.i)
    }
}

impl SameBool for Content
{
    fn samebool(&self, other: &Self) -> bool {
        self.b == other.b
    }
}

fn main() 
{
    
}