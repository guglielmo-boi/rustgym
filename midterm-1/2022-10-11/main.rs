use std::fmt;

/*
    9. write enum XX with variants Y1 Y2, the former takes 2 ints, the latter a string.
    write a function data that takes an XX and then returns the contents of y1 multiplied by each other, or the length of Y2.
*/
enum XX
{
    Y1(i32, i32),
    Y2(String)
}

fn data(xx: &XX) -> i32 {
    match xx {
        XX::Y1(a, b) => a * b,
        XX::Y2(s) => s.len() as i32
    }
}


/*
    10. write enum Z with variants Y1 Y2, the former takes 2 i32, the latter a bool and a string
    write a function maybelength that returns the optional length of the string
*/
enum Z
{
    Y1(i32, i32),
    Y2(bool, String)
}

fn maybelength(z: &Z) -> Option<usize> {
    match z {
        Z::Y1(_, _) => None,
        Z::Y2(_, s) => Some(s.len())
    }
}


/*
    11. write an enum X with 1 variant Y with a String
    write a struct X with 1 field i with a String
    use the module system to give the two separate namespaces
    the enum's module is called enumx, the structs' module is called structx
    write a function longer in another module modfun that takes 2 arguments, enum X and struct X
    and it returns the longer length of the content of the arguments
*/
pub mod enumx
{
    pub enum X
    {
        Y(String)
    }
}

pub mod structx
{
    pub struct X
    {
        pub i: String
    }
}

pub mod modfun
{
    use enumx;
    use structx; 

    pub fn longer(enumx_x: &enumx::X, structx_x: &structx::X) -> usize {
        match enumx_x {
            enumx::X::Y(s) => {
                if s.len() > structx_x.i.len() {
                    s.len()
                } else {
                    structx_x.i.len()
                }
            }
        }
    }
}


/*
    12. write a maybesqrt function that takes a number: i32 and returns an optional i32 value with the result of the square root of the number
*/
fn maybesqrt(number: i32) -> Option<i32> {
    if number < 0 {
        None
    } else {
        Some((number as f32).sqrt() as i32)
    }
}


/*
    13. write a Balance struct with a field amt : i32.
    Add a maybewithdraw method that takes a n: i32 and subtracts n from the amount and returns the optional new total, unless that would be a negative number.
*/
struct Balance
{
    amt: i32
}

impl Balance
{
    fn maybewithdraw(&self, n: i32) -> Option<i32> {
        let ret = self.amt - n;

        match ret {
            0.. => Some(ret),
            _ => None
        }
    }
}


/*
    14. write a function prevchar that takes a char and returns its precedent value as i32, in case of 'a' it returns '`'
    check out what functions std offers for char
    write a function replwithprev that takes a string and replaces all its characters with the previous one, unless the string contains an 'a'.
    the return should be a result of either the replaced string, or a void error
*/
fn prevchar(c: char) -> char {
    char::from_u32((c as u32) - 1).unwrap()
}

fn replwithprev(s: &mut String) -> Result<String, ()> {
    if s.contains('a') {
        Err(())
    } else {
        let mut ret = String::new();

        for c in s.chars() {
            ret.push(prevchar(c));
        }

        *s = ret.clone();

        Ok(ret)
    }
}


/*
    15. write a struct X with 2 fields s : string and i : i32
    write a struct Y with 2 fields b : bool and c : string
    give each struct a constructor function new. The default values are "xxx", 10 for X and true, "op" for Y.
    give each struct a method getstring for replacing the string with "", moving the string out of the struct and returning said string
    // use std::mem::replace
    write a function swapstr that takes a X and a Y and then moves s into c and c into s
    make D displayable both with :? the formatter as well as with a {} argument in println. With a {} argument, see the example for the result
*/
#[derive(Debug)]
struct X
{
    s: String,
    i: i32
}

impl X
{
    fn new() -> X {
        X {
            s: String::from("xxx"),
            i: 10
        }
    }

    fn getstring(&mut self) -> String {
        let ret = self.s.clone();
        self.s = String::from("");
        ret
    }
}

impl fmt::Display for X 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S {}, I {})", self.s, self.i)
    }
}

#[derive(Debug)]
struct Y
{
    b: bool,
    c: String
}

impl Y 
{
    fn new() -> Y {
        Y {
            b: true,
            c: String::from("op")
        }
    }

    fn getstring(&mut self) -> String {
        let ret = self.c.clone();
        self.c = String::from("");
        ret
    }
}

impl fmt::Display for Y
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "B {}, C {})", self.b, self.c)
    }
}

fn swapstr(mut x: X, mut y: Y) -> (X, Y) {
    (X { s: y.getstring(), ..x }, Y { c: x.getstring(), ..y })
}


/*
    16. write enum C with variants C1, C2, the former takes two i32's, the latter a bool and a string
    write struct D with a c field of type C and a s field of type String
    give D a function new with no parameter that returns a D
    give D a function new_with_C that returns a D given a C, and initialises the s field to the content of C2, or to "not there"
    give D a method larger that returns the max length between s and the enum's possible string.
    make D displayable both with :? the formatter as well as with a {} argument in println. With a {} argument, see the example for the result
*/
#[derive(Debug)]
enum C
{
    C1(i32, i32),
    C2(bool, String)
}

#[derive(Debug)]
struct D
{
    c: C,
    s: String
}

impl D
{
    fn new() -> D {
        D {
            c: C::C1(0, 0),
            s: String::from("")
        }
    }

    fn new_with_C(c: C) -> D {
        let d_s = match c {
            C::C1(_, _) => String::from("not there"),
            C::C2(_, ref s) => s.clone()
        };

        D {
            c: c,
            s: d_s
        }
    }

    fn larger(&self) -> usize {
        match &self.c {
            C::C1(_, _) => self.s.len(),
            C::C2(_, s) => {
                if s.len() > self.s.len() {
                    s.len()
                } else {
                    self.s.len()
                }
            }
        }
    }
}

impl fmt::Display for D
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} with {:?})", self.s, self.c)
    }
}


/*
    18. write a function veclengths that takes 1 vector of Strings and returns a vector of i32 where an element at position j is the length 
    of the string at position j f the input vector
*/
fn veclengths(v: &Vec<String>) -> Vec<i32> {
    let mut ret = Vec::new();

    for s in v.iter() {
        ret.push(s.len() as i32);
    }

    ret
}


/*
    19. write a function removeelfromvector that takes a Vector of String and a number n: usize and it removes the first element from 
    the vector whose content has length n
*/
fn removeelfromvector(v: &mut Vec<String>, n: usize) {
    for (i, s) in v.iter().enumerate() {
        if s.len() == n {
            v.remove(i);
            return;
        }
    }
}

pub fn main(){
    let mut v: Vec<String> = vec![String::from("what");4];
    v.push(String::from("now"));    v.push(String::from("what"));
    println!("{:?}",v);
    removeelfromvector(&mut v, 3);
    println!("{:?}",v);
}