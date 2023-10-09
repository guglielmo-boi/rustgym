/*
    1. Write a function prev_str that takes an &str as input and returns a String. 
    This function converts the &str by replacing each character with its predecessor. For example:

    'b' becomes 'a' 
    'f' becomes 'e' 
    'B' becomes 'A' 
    'A' remains 'A' 
    'a' remains 'a'

    If the character is not a letter, it remains unchanged.
*/

fn prev_str(s: &str) -> String {
    let mut ret = String::new();

    for c in s.chars() {
        let chr = {
            if c.is_ascii_alphabetic() {
                match c {
                    'a' => 'a',
                    'A' => 'A',
                    c => char::from_u32(c as u32 - 1).unwrap() 
                }
            } else {
                c
            }
        };

        ret.push(chr);
    }

    ret
}


/*
    2. Write a struct X with two fields: s (an Option<String> ) and i (an i32 ). Then, implement the following methods for X :
    new : takes a &str and an i32 and returns an X instance
    take_str : takes a mutable reference to self and returns the s field of X , replacing it with None
*/
struct X
{
    s: Option<String>,
    i: i32 
}

impl X 
{
    fn new(s: &str, i: i32) -> X {
        X {
            s: Some(String::from(s)),
            i: i
        }
    }

    fn take_str(&mut self) -> Option<String> {
        let ret = self.s.clone();
        self.s = None;
        ret
    }
}


/*
    3. Create a function named replace_surname that takes a NameSurname struct and a String as input and returns a String.
    The function should replace the surname of the NameSurname struct with the String and return the old surname.
    use std::mem::swap;
*/
struct NameSurname
{
    name: String,
    surname: String
}

fn replace_surname(mut person: NameSurname, mut s: String) -> String {
    let ret = person.surname.clone();
    std::mem::swap(&mut person.surname, &mut s);
    ret
}


/*
    4. Write a struct Student with two fields: name (a String ) and id (a u32 ). Then, implement the following methods for Student :
    new: takes a &str and a u32 and returns a Student instance
    Display: implement the Display trait for Student so that it prints the name and the id of the student
    Then write a struct University with two fields: name (a   ) and Vec<Student> ). Then, implement the following methods for
    new: takes a &str and a &[Student] and returns a
    remove_student: takes an id: u32 and returns a Result<Student, &str> : 
    Ok(Student) if a student with the given id is found and removed (a String students : instance University 
    Err("Not found") otherwise
    Display: implement the Display trait for University so that it prints the name and the list of students of the university
*/
use std::fmt;

#[derive(Debug, Clone)]
struct Student
{
    name: String,
    id: u32
}

impl Student 
{
    fn new(name: &str, id: u32) -> Student {
        Student {
            name: String::from(name),
            id: id
        }
    }
}

impl fmt::Display for Student
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?})", self)
    }
}

struct University
{
    name: String,
    students: Vec<Student>
}

impl University
{
    fn new(name: &str, students: &[Student]) -> University {
        University {
            name: String::from(name),
            students: students.to_vec()
        }
    }

    fn remove_student(&mut self, id: u32) -> Result<Student, &str> {
        for (i, student) in self.students.iter().enumerate() {
            if student.id == id {
                let ret = student.clone();
                self.students.remove(i);
                return Ok(ret);
            }
        }

        Err("Not found")
    }
}

impl fmt::Display for University
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nStudents: {:?}", self.name, self.students)
    }
}


/*
    5. Write a struct AirFleet that contains a vector of Airplane. Airplane is a struct that contains a company (An enum called AirplaneCompany with options: 
    Airbus or Boeing) and a model (String). Implement the following methods for AirFleet :
    remove_boeing : remove all the airplanes of the company Boeing from the fleet
    add_airplane : add an airplane to the fleet
    search_airplane : search an airplane by model and return the company of the airplane, it must return a Result<AirplaneCompany, String>. 
    The function must return OK if the airplane is found, Err if the airplane is not found, or the fleet is empty.
*/
#[derive(Debug, Clone, PartialEq)]
enum AirplaneCompany
{
    Airbus,
    Boeing
}

#[derive(Debug)]
struct Airplane
{
    company: AirplaneCompany,
    model: String
}

#[derive(Debug)]
struct AirFleet
{
    fleet: Vec<Airplane>
}

impl AirFleet
{
    fn remove_boeing(&mut self) {
        self.fleet.retain(|airplane| airplane.company != AirplaneCompany::Boeing);
    }

    fn add_airplane(&mut self, airplane: Airplane) {
        self.fleet.push(airplane);
    }

    fn search_airplane(&self, model: &str) -> Result<AirplaneCompany, String> {
        for airplane in &self.fleet {
            if airplane.model == model {
                return Ok(airplane.company.clone());
            } 
        }

        Err(String::from("Not in this fleet"))
    }
}


/*
    6. create the module hashmaps that contains a struct Maps with a field map of type HashMap<Unumber, String>
    create the module unumber that contains a type Unumber = usize
    In another module create a function string_to_tuple that takes a Maps and returns a HashMap<Unumber, (Unumber, String)>
    the function should convert the HashMap<Unumber, String> to a HashMap<Unumber, (Unumber, String)>
    where the key remains the same, and the element is a tuple where its first field is the length
    of the string and its second field is the string itself
*/
use std::collections::HashMap;

pub mod unumber
{
    pub type Unumber = usize;
}

pub mod hashmaps
{
    use std::collections::HashMap;
    use unumber::Unumber;

    pub struct Maps
    {
        pub map: HashMap<Unumber, String>
    }
}

pub mod another
{
    use std::collections::HashMap;
    use unumber::Unumber;
    use hashmaps::Maps;

    pub fn string_to_tuple(maps: Maps) -> HashMap<Unumber, (Unumber, String)> {
        let mut ret = HashMap::new();

        for (key, value) in maps.map {
            ret.insert(key, (value.len(), value));
        }

        ret
    }
}

use unumber::Unumber;
use hashmaps::Maps;
use another::string_to_tuple;


/*
    7. write a struct Size that has two fields: width and height, both of type f32.
    Then, implement the following methods for Size:
    - new: takes two f32 arguments and returns a Size instance
    - area: takes a reference to self and returns the area of the Size instance
    - compare: takes a reference to self and another Size instance and returns
    an `Option<bool>`:
    - None if the two Size instances have the same area
    - Some(true) if the area of the first Size instance is greater than the area
    of the second Size instance
    - Some(false) if the area of the first Size instance is less than the area
    of the second Size instance
*/
struct Size
{
    width: f32,
    height: f32
}

impl Size
{
    fn new(width: f32, height: f32) -> Size {
        Size {
            width: width,
            height: height
        }
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn compare(&self, size: &Size) -> Option<bool> {
        let self_area = self.area();
        let other_area = size.area();

        if self_area > other_area {
            Some(true)
        } else if self_area < other_area {
            Some(false)
        } else {
            None
        }
    }
}


/*
    8. write a struct `MaybePoint` that has two fields: `x` and `y`, both of type
    `Option<i32>`. Then, implement the following methods for `MaybePoint`:
    - new: takes two `Option<i32>` arguments and returns a `MaybePoint` instance
    - is_some: takes a reference to self and returns `true` if both `x` and `y`
    are `Some` values, `false` otherwise
    - maybe_len: takes a reference to self and returns an `Option<f32>`:
    - `None` if `x` or `y` are `None`
    - `Some(len)` where `len` is the length of the vector from `(0, 0)` to `(x, y)`
*/
struct MaybePoint
{
    x: Option<i32>,
    y: Option<i32>
}

impl MaybePoint
{
    fn new(x: Option<i32>, y: Option<i32>) -> MaybePoint {
        MaybePoint {
            x: x,
            y: y
        }
    }

    fn is_some(&self) -> bool {
        self.x.is_some() && self.y.is_some()
    }

    fn maybe_len(&self) -> Option<f32> {
        if self.x.is_none() || self.y.is_none() {
            None
        } else {
            Some((((self.x.unwrap().pow(2)) + (self.y.unwrap().pow(2))) as f32).sqrt())
        }
    }
}


/*
    9. Write a function `res1` that takes an `i32` and returns a `Result<i32, String>`. The
    function returns `OkNo` if `n` is divisible by 10, `Err("error")` otherwise.
    Then write a function `res2` that takes a `Result<i32, &str>` and returns a `Result<i32, String>`.
    The function returns `OkNo` if `n` is divisible by 5, `Err("error")` otherwise.
    Then write a `wrapper` function that takes an `i32` and returns a `Result<i32, String>`.
    The function returns `OkNo` if `n` is divisible by 10 and 5, `Err("error")` otherwise.
    Errors should be propagated.
*/
fn res1(n: i32) -> Result<i32, String> {
    if n % 10 == 0 {
        Ok(n)
    } else {
        Err(String::from("error"))
    }
}

fn res2(n: i32) -> Result<i32, &'static str> {
    if n % 5 == 0 {
        Ok(n)
    } else {
        Err("error")
    }
}

fn wrapper(n: i32) -> Result<i32, String> {
    let ret1 = res1(n);
    let ret2 = res2(n);

    if ret1.is_ok() && ret2.is_ok() {
        Ok(n)   
    } else {
        Err(String::from("error"))
    }
}


/*
    10. create a function order that take a vector of strings and returns a vector of strings,
    where each string is prepended by its index in the vector followed by a dash and a space.
    For example, given the vector ["How", "Are", "You"] the function should return ["0 - How", "1 - Are", "2 - You"].
*/
fn order(v: Vec<String>) -> Vec<String> {
    let mut ret = Vec::new();

    for (i, s) in v.iter().enumerate() {
        ret.push(i.to_string() + " - " + &s.clone());
    }

    ret
}


/*
    11. Define two `enum`s both called `X`, place them in two different modules, `modx` and `mody`. Define the `enum`s like this:
    - With 2 variants `S` with a `char` and `C` with a `String`
    - With 1 variant `F` with a `f64` and a `usize`
    Write a function `sum` in the module `modsum` that takes a `X` from `modx` and a `X` from `mody`, 
    the function returns the `u8` equivalent of the `char` or the length of the `String` based on the variant, summed with the product of `f64` by the `usize`.
*/
pub mod modx
{
    pub enum X
    {
        S(char),
        C(String)
    }
} 

pub mod mody
{
    pub enum X
    {
        F(f64, usize)
    }
}


pub mod modsum
{
    use modx;
    use mody;

    pub fn sum(x: modx::X, y: mody::X) -> u8 {
        let val = {
            match x {
                modx::X::S(c) => c as u8,
                modx::X::C(s) => s.len() as u8
            }
        };

        let val2 = {
            match y {
                mody::X::F(f, u) => f * u as f64
            }
        };

        (val as f64 + val2) as u8
    }
}


pub fn main()
{
    
}