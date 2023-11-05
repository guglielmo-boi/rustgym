use std::fmt::Debug;
use std::fmt::Display;

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


/*

*/

fn main() 
{

}
