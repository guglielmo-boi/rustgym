use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Add, Sub, Mul};
use rand::{self, Rng};

/* 
    1. Define a trait Printable  that defines a method print  that printlns self  to the console.
    Implement this trait for i32 , String  and Vec<T>  where T  implements Printable .
    Create a function print  that takes a generic argument T  that implements Printable  and
    calls print on T.
*/
trait Printable
{
    fn print(&self);
}

impl Printable for i32
{
    fn print(&self) {
        println!("{}", self);
    }
}

impl Printable for String
{
    fn print(&self) {
        println!("{}", self);
    }
}

impl<T: Debug> Printable for Vec<T>
{
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn print<T: Printable>(printable: T) {
    printable.print();
}


/*
    2.  Write a struct Book  that contains two fields:
    title  of type &str
    cat  of type Category  (Category  is an enum with some variants)
    Write a struct Library , with a field bookcases  of type [Vec<Book>; 10] . Every bookcase
    have 10 floors, and every floor can hold a number of books.
    Derive the Debug  trait for Library , without manually implementing it for any of the types
    that you defined.
    Implement the trait Default  for Book  giving it a random cat  and a random title .
    Implement default_with_cat  for Book , that takes a Category  and returns an instance
    with the specified category and use the default  method for title . (hint: use the ..
    operator)
    Derive the trait Default  for Category  and Library .
    Implement a trait Populatable  that defines the populate  function. Implement
    Populatable  for Library , populating its bookcases with 3 books for each floor, using the
    default Category.
*/
#[derive(Debug)]
enum Category
{
    Literature,
    Thriller,
    Horror,
    Historical,
    Essay
}

impl Default for Category {
    fn default() -> Category {
        Category::Literature
    }
}

#[derive(Debug)]
struct Book<'a>
{
    title: &'a str,
    cat: Category
}

impl<'a> Default for Book<'a>
{
    fn default() -> Book<'a> {
        Book {
            title: "title",
            cat: Category::Literature
        }
    }
}

impl<'a> Book<'a>
{
    fn default_with_cat(cat: Category) -> Book<'a> {
        Book {
            cat: cat,
            ..Book::default()
        }
    }
}

#[derive(Debug)]
struct Library<'a>
{
    bookcases: [Vec<Book<'a>>; 10]
}

impl<'a> Default for Library<'a> {
    fn default() -> Library<'a> {
        Library {
            bookcases: [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]]
        }
    }
}

trait Populatable
{
    fn populate(&mut self);
}

impl<'a> Populatable for Library<'a>
{
    fn populate(&mut self) {
        for bookcase in self.bookcases.iter_mut() {
            bookcase.push(Book::default_with_cat(Category::default()))
        }
    }
}


/*
    3. Define a function restricted  that is generic over T and U .
    T  types should be comparable and debuggable.
    U  types should be displayable.
    The function should take two arguments t1  and t2  of type T  and u  of type U
    respectively.
    It should then print the smaller of t1  and t2  to the console together with u like this:
    and return the smaller between t1  and t2 
*/
fn restricted<T: Ord + Debug, U: Display>(t1: T, t2: T, u: U) -> T {
    if t1 < t2 {
        println!("minor: {:?}", t1);
        println!("{}", u);
        t1
    } else {
        println!("minor: {:?}", t2);
        println!("{}", u);
        t2
    }
}


/*
    4. Define a struct Tasks  that has the following fields: tasks : a Vec<Task> en define a struct
    Task  that has the following fields: name: String, priority: i32, done: bool.
    Implement useful methods for both structs (e.g. new ). 
    Implement the Iterator  trait for Tasks  such that it returns each task that has not been completed 
    yet and removes the completed ones from the Vec.
*/
struct Task
{
    name: String,
    priority: i32,
    done: bool
}

impl Task
{
    fn new(name: String, priority: i32) -> Task {
        Task {
            name: name,
            priority: priority,
            done: false
        }
    }
}

struct Tasks
{
    tasks: Vec<Task>
}

impl Tasks
{
    fn new() -> Tasks {
        Tasks {
            tasks: Vec::new()
        }
    }
}

impl Iterator for Tasks {
    type Item = Task;

    fn next(&mut self) -> Option<Self::Item> {
        self.tasks.iter().position(|t| !t.done).map(|i| self.tasks.remove(i))
    }
}


/*
    5. Define the struct Pair(i32, String)  and then implement the traits contained in std::ops
    and needed for adding the possibility to do the following operations, every operation must
    return another Pair:
    Pair  + Pair : like doing Pair1 + Pair2.0 + Pair2.1
    Pair  - Pair : same as above
    Pair  + i32 : add the i32  to Pair.0
    Pair  - i32 : same as above
    Pair  + &str : append the &str  to Pair.1
    Pair  - &str : search the &str  in Pair.1  and replace it with ""
    Pair  * u32 : with n the u32, Pair.0  to the nth power, Pair.1  repeated n times.
*/
struct Pair(i32, String);

impl Add<Pair> for Pair
{
    type Output = Pair;

    fn add(self, other: Pair) -> Pair {
        let mut string = self.1;
        string.push_str(&other.1);

        Pair(self.0 + other.0, string)
    }
}

impl Add<i32> for Pair
{
    type Output = Pair;

    fn add(self, other: i32) -> Pair {
        Pair(self.0 + other, self.1)
    }
}

impl Add<&str> for Pair
{
    type Output = Pair;

    fn add(self, other: &str) -> Pair {
        let mut string = self.1;
        string.push_str(other);

        Pair(self.0, string)
    }
}

impl Sub<Pair> for Pair
{
    type Output = Pair;

    fn sub(self, other: Pair) -> Pair {
        let mut string = self.1;
        string = string.replace(&other.1, "");

        Pair(self.0 - other.0, string)
    }
}

impl Sub<i32> for Pair
{
    type Output = Pair;

    fn sub(self, other: i32) -> Pair {
        Pair(self.0 + other, self.1)
    }
}

impl Sub<&str> for Pair
{
    type Output = Pair;

    fn sub(self, other: &str) -> Pair {
        let mut string = self.1;
        string = string.replace(other, "");

        Pair(self.0, string)
    }
}

impl Mul<u32> for Pair
{
    type Output = Pair;

    fn mul(self, other: u32) -> Pair {
        let mut string = String::new();
        for i in 0..other {
            string.push_str(&self.1);
        }

        Pair(self.0.pow(other), string)
    }
}


/*
    6. Write a struct Gate generic over S that represents the state of the gate:
    Open
    Closed
    Stopped, with a field reason: String
    The gate can be in one of these states at any given time. Each state has a dierent set of
    available methods:
    Open: close
    Closed: open
    Stopped: open, close
    Each method takes ownership of self.
    The close  method returns a Result:
    Ok<Gate<Closed>>  if the gate was successfully closed
    Err<Gate<Stopped>>  if the gate could not be closed
    The open  method returns a Result:
    Ok<Gate<Open>>  if the gate was successfully opened
    Err<Gate<Stopped>>  if the gate could not be opened
    The open  and close  methods of the Open  and Closed  states have a random chance of
    failing (user defined).
    The open  and close  methods of the Stopped  state always succeed.
*/
struct Open;

struct Closed;

struct Stopped;

struct Gate<S>
{
    _state: S
}

impl Gate<Open> 
{
    pub fn new() -> Gate<Open> {
        Gate {
            _state: Open
        }
    }

    pub fn close(&self) -> Result<Gate<Closed>, Gate<Stopped>> {
        let r = rand::thread_rng().gen_range(1..100);

        match r {
            1..=80 => Ok(Gate {
                _state: Closed
            }),
            _ => Err(Gate {
                _state: Stopped
            })
        }
    }
}

impl Gate<Closed>
{
    pub fn new() -> Gate<Closed> {
        Gate {
            _state: Closed
        }
    }

    pub fn open(&self) -> Result<Gate<Open>, Gate<Stopped>> {
        let r = rand::thread_rng().gen_range(1..100);

        match r {
            1..=80 => Ok(Gate {
                _state: Open
            }),
            _ => Err(Gate {
                _state: Stopped
            })
        }
    }
}

impl Gate<Stopped>
{
    pub fn new() -> Gate<Stopped> {
        Gate {
            _state: Stopped
        }
    }

    pub fn open(&self) -> Gate<Open> {
        Gate {
            _state: Open
        }
    }

    pub fn close(&self) -> Gate<Closed> {
        Gate {
            _state: Closed
        }
    }
}


/*
    7. Define two traits Heatable  and Friable  that have one method each named cook . Then
    define the trait Heater  and Frier  that have respectively two methods: heat  and fry ,
    where each method will take a reference to self  and a trait object of Heatable  and
    Friable .
    Then create two "cooker" structs:
    Oven  that implements Heater  that simply calls the method cook  of the Heatable
    trait.
    Pan  that implements Heater  and Frier , the implementation is the same as above for
    each trait.
    Then create two "food" structs:
    Pie  that has one bool field ready
    Carrot  that has one field of type CarrotState
    CarrotState  is an enum  that has 4 variants: Raw , Cooked , Fried , Burnt .
    Define the trait Edible  that defines the method eat . Now implement the following traits:
    Heatable  for Pie : if the pie is already ready it prints "you burned the pie!", otherwise it
    sets ready  to true;
    Heatable  for Carrot : if the carrot is not raw, then the carrot is burnt otherwise the
    carrot is cooked.
    Friable  for Carrot : if the carrot is already fried, the carrot is burnt otherwise the
    carrot is fried.
    Edible  for Pie : if the pie isn't ready it prints "you got stomach ache" otherwhise
    "yummy"
    Edible  for Carrot : it prints,
    "mmh, crunchy" if it's raw;
    "mmh, yummy" if it's cooked;
    "mmh, crispy" if it's fried;
    "mmh, burnt" if it's burnt;
*/
trait Heatable
{
    fn cook(&mut self);
}

trait Friable
{
    fn cook(&mut self);
}

trait Heater
{
    fn heat(&self, heatable: &mut dyn Heatable);
}

trait Frier
{
    fn fry(&self, friable: &mut dyn Friable);
}

trait Edible
{
    fn eat(&self);
}

struct Oven {}

impl Heater for Oven
{
    fn heat(&self, heatable: &mut dyn Heatable) {
        heatable.cook();
    }
}

struct Pan {}

impl Heater for Pan
{
    fn heat(&self, heatable: &mut dyn Heatable) {
        heatable.cook();
    }
}

impl Frier for Pan
{
    fn fry(&self, friable: &mut dyn Friable) {
        friable.cook();
    }
}

struct Pie
{
    ready: bool
}

impl Heatable for Pie
{
    fn cook(&mut self) {
        if self.ready {
            println!("You burned the pie!");
        } else {
            self.ready = true;
        }
    }
}

impl Edible for Pie 
{
    fn eat(&self) {
        if self.ready {
            println!("Yummy!");
        } else {
            println!("You got stomac ache!");
        }
    }
}

enum CarrotState
{
    Raw,
    Cooked,
    Fried,
    Burnt
}

struct Carrot
{
    state: CarrotState
}

impl Heatable for Carrot
{
    fn cook(&mut self) {
        match self.state {
            CarrotState::Raw => self.state = CarrotState::Cooked,
            _ => self.state = CarrotState::Burnt
        }
    }
}

impl Friable for Carrot
{
    fn cook(&mut self) {
        match self.state {
            CarrotState::Raw => self.state = CarrotState::Fried,
            _ => self.state = CarrotState::Burnt
        }
    }
}

impl Edible for Carrot
{
    fn eat(&self) {
        match self.state {
            CarrotState::Raw => println!("mmh, crunchy!"),
            CarrotState::Cooked => println!("mmh, yummy!"),
            CarrotState::Fried => println!("mmh, crispy!"),
            CarrotState::Burnt => println!("mmh, burnt!")
        }
    }
}


fn main() 
{

}