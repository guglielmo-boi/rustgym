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
    
            if p == 0 {
                new_node.next = self.head.take();
                self.head = Some(new_node);
            } else {
                let mut current = &mut self.head;

                for _ in 0..p {
                    current = &mut current.as_mut().unwrap().next;
                }
                
                new_node.next = current.take();
                *current = Some(new_node);
            }

            self.len += 1;

            Ok(())
        }
    }

    fn prepend(&mut self, e: T) {
        let new_node = Box::new(Node{elem: e, next: self.head.take()});
        self.head = Some(new_node);
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
pub struct NodeContent
{
    pub i: i32,
    pub b: bool
}

impl NodeContent
{
    pub fn new_with(i: i32, b: bool) -> NodeContent {
        NodeContent { i, b }
    }
}

impl PartialEq for NodeContent
{
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}

impl PartialOrd for NodeContent
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.i.partial_cmp(&other.i)
    }
}

impl SameBool for NodeContent
{
    fn samebool(&self, other: &Self) -> bool {
        self.b == other.b
    }
}

fn main() 
{
    let x = 5;
    printdouble(x);
    let s = "what".to_string();
    println!("normal s: {:?}",s);
    printdouble(s);
    let y = 8;
    printdouble(y);

    let w = Wrapper{v:vec![11,12,13,14,15,16,17,18]};
    let mut iw = w.iter();
    println!("first: {}",iw.next().unwrap());
    for el in iw{
        println!("evens: {}",el);
    }

    let s = vec!["nope".to_string(), "game".to_string(), "bananas".to_string()];
    println!("boxed s: {:?}", basicbox_sum(s));

    let elem0 = Content::new_with("test".to_string(),true,2);
    let elem1 = Content::new_with("what".to_string(),true,2);
    let elem2 = Content::new_with("this".to_string(),false,8);
    let elem3 = Content::new_with("dope".to_string(),true,18);
    let mut l = List::new();
    println!("{:?}",l.add(elem0,1));
    println!("{:?}",l);
    l.add(elem1,0);
    println!("{:?}",l);
    l.add(elem2,1);
    println!("{:?}",l);
    l.add(elem3,2);
    println!("{:?}",l);
    let elem4 = Content::new_with("nope".to_string(),false,1);
    l.add(elem4,4);
    println!("{:?}",l);
    let s : String = format!("{:?}",l);
    assert_eq!(s.contains("Vec"),false);

    let mut el1 = NodeContent{i:10, b:true};
    let mut el2 = NodeContent{i:11, b:true};
    let mut el3 = NodeContent{i:12, b:false};
    let mut g = Graph::new();
    println!("{:?}",g);
    g.add_node(el1);
    println!("{:?}",g);
    g.add_node(el2);
    println!("{:?}",g);
    g.add_node(el3);
    println!("{:?}",g);

    let mut el1 = NodeContent{i:10, b:true};
    let mut el2 = NodeContent{i:8, b:false};
    let mut g = Graph::new();
    println!("{:?}",g);
    g.add_node(el1);
    println!("{:?}",g);
    g.add_node(el2);
    println!("{:?}",g);

    let mut el1 = NodeContent{i:10, b:true};
    let mut el2 = NodeContent{i:11, b:true};
    let mut el3 = NodeContent{i:12, b:true};
    let mut g = Graph::new();
    println!("{:?}",g);
    g.add_node(el1);
    println!("{:?}",g);
    g.add_node(el2);
    println!("{:?}",g);
    g.add_node(el3);
    println!("{:?}",g);

    let mut el1 = NodeContent{i:10, b:true};
    let mut el2 = NodeContent{i:9, b:false};
    let mut el3 = NodeContent{i:8, b:true};
    let mut g = Graph::new();
    println!("{:?}",g);
    g.add_node(el1);
    println!("{:?}",g);
    g.add_node(el2);
    println!("{:?}",g);
    g.add_node(el3);
    println!("{:?}",g);
}