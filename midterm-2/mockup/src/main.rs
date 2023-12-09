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

    let n1 = Rc::new(Node::new(1, vec![]));
    let n2 = Rc::new(Node::new(2,
    vec![n1.clone()]));
    let n3 = Rc::new(Node::new(3, vec![]));
    let n4 = Rc::new(Node::new(4,
    vec![n1.clone(), n3.clone()]));
    let n5 = Rc::new(Node::new(5,
    vec![n2.clone(), n4.clone()]));
    let n6 = Rc::new(Node::new(6,
    vec![n5.clone(), n4.clone()]));
    let n7 = Rc::new(Node::new(7,
    vec![n2.clone(), n4.clone()]));
    let graph = Graph::new(vec![
    n1.clone(),
    n2.clone(),
    n3.clone(),
    n4.clone(),
    n5.clone(),
    n6.clone(),
    n7.clone(),
    ]);
    let mut paths: Vec<Vec<NodeRef<i32>>> = vec![];
    for n in graph.nodes.iter() {
    paths.push(graph.dfs(n.clone()))
    }
    paths.iter().for_each(|path| {
    println!("{:?}", path);
    });

    macro_rules! sum_task {
        (let $task: ident =$n1: literal + $n2: literal) => {
            let $task: Box<dyn Task> = Box::new(SumTask::new($n1, $n2));
        };
    }
    macro_rules! len_task {
        (let $task: ident =$s: literal) => {
            let $task: Box<dyn Task> = Box::new(LenTask::new($s.to_owned()));
        };
    }

    sum_task!(let t1 = 10+1);
    len_task!(let t2 = "four");
    let mut tasker1 = Tasker::new();
    let mut tasker2 = tasker1.get_tasker();
    let mut executer1 = tasker2.get_executer();
    let mut executer2 = tasker1.get_executer();
    tasker1.schedule_task(t1);
    tasker2.schedule_task(t2);
    println!("{:?}",executer1.execute_task());
    println!("{:?}",executer2.execute_task());
}
