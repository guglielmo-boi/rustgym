use rand::Rng;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;

/*
    1. Write a function  find_equal, that takes two  &str  as input  s1  and  s2. The function
    look for a string slice of length 2 that appear in both  s1  and  s2. If successful the
    function returns a tuple with the two equal slices. one from  s1 , the other one from  s2 .
    otherwise it return  None.
    Then write a second function  lucky_slice , that take a &str named input_str as
    input. The function create a String with the same length as input_str, filled with
    random lowercase letters, and then call find_equal  on the two strings. If  find_equal  is
    successful the function  lucky_slice  return the slice of  input_str  that was found by
    find_equal. otherwise it returns  None.
*/
fn find_equal<'a, 'b>(a: &'a str, b: &'b str) -> Option<(&'a str, &'b str)> {
    let mut i = 0;

    while i + 2 < a.len() {
        let slice = &a[i..i + 2];

        match b.find(slice)  {
            Some(x) => return Some((slice, &b[x..x + 2])),
            None => { }
        }

        i += 1;
    }

    None
}

fn generate_string(size: usize) -> String {
    let mut ret = String::new();

    let mut rng = rand::thread_rng();

    for _ in 0..size {
        let n: u32 = rng.gen();
        let c = char::from_u32('a' as u32 + n % 26);
        ret.push(c.unwrap());
    }

    ret
}

fn lucky_slice(input_str: &str) -> Option<&str> {
    let generated = generate_string(input_str.len());

    match find_equal(input_str, &generated) {
        Some((a, _)) => Some(a),
        None => None
    }
}


/*
    2. Write a struct  Person  that has 3 fields. a  name  with type  String , and two parents
    ( father  and  mother ). Each parent is an  Option  of an immutable reference to a
    Person. Then implement the following methods:
    a  new  function, that takes a name and two options to the parents and then returns a
    new  Person .
    a  find_relatives method that take a  u32  input named  generations . the function
    returns a  Vec  containing all the relatives within the generations range... for
    example:
    if generations is 0, the return should be just the person itself
    if generations is 1, the return should be the person itself + his parents
    if generations is 2, the function should return person itself + his parents + his
    grandparents.
    a  find_roots  method that returns a  Vec  containing all the relatives that has at
    least one parent set to  None
    implement the second and the third methods recursively.
*/
#[derive(Debug, Clone)]
struct Person<'a>
{
    name: String,
    father: Option<&'a Person<'a>>,
    mother: Option<&'a Person<'a>>
}

impl<'a> Person<'a>
{
    fn new(name: String, father: Option<&'a Person<'a>>, mother: Option<&'a Person<'a>>) -> Person<'a> {
        Person {
            name: name,
            father: match father {
                Some(p) => Some(p),
                None => None
            },
            mother: match mother {
                Some(p) => Some(p),
                None => None
            }
        }
    }

    fn find_relatives(&self, generations: u32) -> Vec<&Person> {
        if generations == 0 {
            vec![self]
        } else {
            let mut ret = vec![self];

            match self.father {
                Some(p) => {
                    ret.append(&mut p.find_relatives(generations - 1));
                },
                None => { }
            }

            match self.mother {
                Some(p) => {
                    ret.append(&mut p.find_relatives(generations - 1));
                },
                None => { }
            }

            ret
        }
    }

    fn find_roots(&self) -> Vec<&Person> {
        self.find_relatives(u32::MAX).into_iter().filter(|p| {
            p.father.is_none() || p.mother.is_none()
        }).collect()
    }
}


/*
    3. Correct the following code:

    struct ImportantExcerpt<'a>
    {
        part: &'a str,
    }

    impl<'a, 'b> ImportantExcerpt<'a>
    {
        fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
*/
struct ImportantExcerpt<'a>
{
    part: &'a str,
}

impl<'a, 'b> ImportantExcerpt<'a>
{
    fn announce_and_return_part(&'a self, announcement: &'a str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


/*
    4. Annotate struct with lifetime:
    - r and s  must have different lifetimes
    - lifetime of s is bigger than that of r
    struct DoubleRef<T>
    {
        r: &T,
        s: &T
    }
*/

struct DoubleRef<'a, 'b: 'a, T> 
{
    r: &'a T,
    s: &'b T
}


/*
    5.
    Write a trait Split that has one generic type ReturnType. the trait has one method split
    that take a immutable reference to  self  and return a tuple (ReturnType, ReturnType).
    the function split the element of the trait in half.
    Implement the trait for:
    String , where  split  returns  (&str, &str)
    [&i32] , where  split  returns  (&[i32], &[i32])
    LinkedList<f64> , where  split  returns  (LinkedList<f64>, LinkedList<f64>)
*/
// trait Split<ReturnType>
// {
//     fn split(&mut self) -> (ReturnType, ReturnType);
// }

// impl Split<String> for String
// {
//     fn split(&mut self) -> (String, String) {
//         let mid = self.len() / 2;
//         (String::from(&self[0..mid]), String::from(&self[mid..self.len()]))
//     }    
// }

// impl Split<&[i32]> for &[i32]
// {
//     fn split(&mut self) -> (&[i32], &[i32]) {
//         let mid = self.len() / 2;
//         (&self[0..mid], &self[mid..self.len()])
//     }    
// }


/*
    6. Create the following structs:
    Point  that has a  x  and a  y  coordinates
    Circle  that has a  radius  and a  center
    Rectangle  that has a  top_left  and a  bottom_right  points
    Implement the trait  Default  (that is already defined in the standard library)
    A default point is a point center in (0,0)
    A default circle is has a center in (0,0) and radius one
    A default rectangle has the points in (-1,1) and (1,-1)
    for  Point  implement the traits  Add  and  Sub  (that are the trait for the  +  and  -
    operators).
    For example:
    (10,11) + (1,1) = (11,12)
    (0,0) - (20,30) = (-20,-30)
    Create the struct  Area  that contains a f32 value with the area. Then implement the
    Default  trait that returns an area with 0
    Implement the trait  Display  for  Area  with a custom message looking like: Area is 20
    cm²
    Create a custom trait  GetArea  that adds a method  get_area(&self)->Area  to the 3
    objects.
    The method returns the geometric area of the shape (a Point has area 0)
    implement the  Add  trait that for:
    Summing an  Area  with an  Area
    Summing an  Area  with an  &dyn GetArea
    create a function  sum_area  that take as input a slice of  &dyn GetArea  object and returns
    the area of all objects summed
*/
struct Point
{
    x: f32,
    y: f32
}

impl Default for Point
{
    fn default() -> Point {
        Point {
            x: 0.0,
            y: 0.0
        }
    }
}

impl Add for Point
{
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Point
{
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl GetArea for Point
{
    fn get_area(&self) -> Area {
        Area {
            area: 0.0
        }
    }
}

struct Circle
{
    radius: f32,
    center: Point
}

impl Default for Circle
{
    fn default() -> Circle {
        Circle {
            radius: 1.0,
            center: Point::default()
        }
    }
}

impl GetArea for Circle 
{
    fn get_area(&self) -> Area {
        Area {
            area: std::f32::consts::PI * self.radius.powf(self.radius)
        }
    }
}

struct Rectangle
{
    top_left: Point,
    bottom_right: Point
}

impl Default for Rectangle
{
    fn default() -> Rectangle {
        Rectangle {
            top_left: Point {x: -1.0, y: 1.0 },
            bottom_right: Point {x: 1.0, y: -1.0 }
        }
    }
}

impl GetArea for Rectangle
{
    fn get_area(&self) -> Area {
        Area {
            area: (self.bottom_right.x - self.top_left.x) * (self.top_left.y - self.bottom_right.y)
        }
    }
}

struct Area
{
    area: f32
}

impl Default for Area
{
    fn default() -> Area {
        Area {
            area: 0.0
        }
    }
}

impl Add for Area
{
    type Output = Area;

    fn add(self, other: Area) -> Area {
        Area {
            area: self.area + other.area
        }
    }
}

impl Add<&dyn GetArea> for Area
{
    type Output = Area;

    fn add(self, other: &dyn GetArea) -> Area {
        Area {
            area: self.area + other.get_area().area
        }
    }
}

impl fmt::Display for Area 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Area is {}", self.area)
    }
}

trait GetArea
{
    fn get_area(&self) -> Area;
}

fn sum_area(elements: &[&dyn GetArea]) -> Area {
    let mut ret = Area::default();

    for e in elements {
        ret.area += e.get_area().area
    }

    ret
}

fn main()
{
    println!("{:?}", find_equal(&"banana", &"anna"));
    println!("{:?}", lucky_slice(&"guglielmo"));

    let father = Person::new(String::from("Father"), None, None);
    let mother = Person::new(String::from("Mother"), None, None);
    let child = Person::new(String::from("Child"), Some(&father), Some(&mother));

    println!("{:?}", child.find_roots());
}