use rand::Rng;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::collections::LinkedList;
use std::collections::HashMap;

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
    5. Write a trait Split that has one generic type ReturnType. the trait has one method split
    that take a immutable reference to  self  and return a tuple (ReturnType, ReturnType).
    the function split the element of the trait in half.
    Implement the trait for:
    String , where  split  returns  (&str, &str)
    &[i32] , where  split  returns  (&[i32], &[i32])
    LinkedList<f64> , where  split  returns  (LinkedList<f64>, LinkedList<f64>)
*/
trait Split<'a, ReturnType>
{
    fn split(&'a mut self) -> (ReturnType, ReturnType);
}

impl<'a> Split<'a, &'a str> for String
{
    fn split(&'a mut self) -> (&'a str, &'a str) {
        let mid = self.len() / 2;
        (&self[0..mid], &self[mid..self.len()])
    }    
}

impl<'a, 'b: 'a> Split<'a, &'a [i32]> for &'b [i32]
{
    fn split(&mut self) -> (&'a [i32], &'a [i32]) {
        let mid = self.len() / 2;
        (&self[0..mid], &self[mid..self.len()])
    }    
}

impl<'a, 'b: 'a> Split<'a, LinkedList<f64>> for LinkedList<f64>
{
    fn split(&mut self) -> (LinkedList<f64>, LinkedList<f64>) {
        let mid = self.len() / 2;
        (self.clone().into_iter().take(mid).collect(), self.clone().into_iter().skip(mid).collect())
    }    
}


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


/*
    7. Create a function 'skip_prefix' that, given non mutable references to  telephone_number:
    &str  and  prefix: &str , returns  &str . The function removes the prefix from the
    number, and if there isn't the prefix contained at the start of  number, return  number
    itself.
*/
fn skip_prefix<'a, 'b: 'a>(telephone_number: &'b str, prefix: &'b str) -> &'a str {
    if prefix.len() <= telephone_number.len() {
        if telephone_number.find(prefix).is_some() {
            return &telephone_number[prefix.len()..telephone_number.len()];
        }
    }

    telephone_number
}


/*
    8. Create a struct  Chair  that has two fields:  color: &str  and  quantity: &usize . Create
    another structure  Wardrobe  that has the same fields of  Chair . Create a trait  Object
    that has two function declarations:  build  that has  &self  as argument and returns a
    &str ;  get_quantity  that has  &self  as argument and return a  String .
    Then, implement the trait  Object  for  Chair  and  Wardrobe .
    build  should return a  &str  with "Chair/Wardobe has been built"
    get_quantity  should return a formatted message with the number of
    chairs/wardrobes.
    Implement also the Display trait for Chair and Wardrobe, that returns a different
    formatted message if there are zero, one or two or more chairs/wardrobes.
    The message should also contain the color of the objects if there are one or more.
*/
struct Chair<'a> 
{
    color: &'a str,
    quantity: &'a usize
}

impl<'a, 'b: 'a> Object for Chair<'b>
{
    fn build(&self) -> &str {
        "Chair has been built"
    }

    fn get_quantity(&self) -> String {
        format!("chairs quantity: {}", self.quantity)
    }
}

impl<'a> fmt::Display for Chair<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.quantity {
            0 => write!(f, "There are no chairs"),
            1 => write!(f, "There is only one {} chair", self.color),
            _ => write!(f, "There are {} {} chairs", self.quantity, self.color)
        }
    }
}
struct Wardrobe<'a>
{
    color: &'a str,
    quantity: &'a usize
}

impl<'a, 'b: 'a> Object for Wardrobe<'b>
{
    fn build(&self) -> &str {
        "Wardrobe has been built"
    }

    fn get_quantity(&self) -> String {
        format!("wardrobes quantity: {}", self.quantity)
    }
}

impl<'a> fmt::Display for Wardrobe<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.quantity {
            0 => write!(f, "There are no wardrobes"),
            1 => write!(f, "There is only one {} wardrobe", self.color),
            _ => write!(f, "There are {} {} wardrobes", self.quantity, self.color)
        }
    }
}

trait Object
{
    fn build(&self) -> &str;
    fn get_quantity(&self) -> String;   
}


/*
    Create a simple permission manager for certain actions on an Operating System.
    Create an enum  Role  that has the following fields:  GUEST, USER, ADMIN . Create another
    enum called  Permission  with the following fields:  READ, WRITE, EXECUTE . For these
    enums derive the trait  PartialEq ,  Eq  and, for Permission,  Hash . You'll need them later
    for some comparison and for using the enum Permission as a key of an HashMap.
    Create a struct  Actions  that has the field  action: String  and  permission:
    HashMap<Permission, bool> .
    Create then the struct  User  with these fields:  name: String ,  role: Role ,  actions:
    Vec<Actions>
    Write the trait  Auth  with the following methods:
    check_permission  with arguments  &self ,  action: &str ,  permission_type:
    &Permission  and returns a bool. This method checks if there is an action in self with
    as a name the string passed in the arguments. If it exists return if the
    permission_type for that action is true or false. If this action doesn't exist in self,
    then return false
    can_write  with arguments  &self  and  string &str  and returns a bool. This
    method use  check_permission  and checks if an actions identified with the string
    argument is writeable.
    can_read  with arguments  &self  and  string &str  and returns a bool. This method
    use  check_permission  and checks if an actions identified with the string argument
    is readable.
    can_execute  with arguments  &self  and  string &str  and returns a bool. This
    method use  check_permission  and checks if an actions identified with the string
    argument is executable
    Apply the trait  Auth  for  User , and write the relative methods implementations.
    Implement the  Default  trait for Actions. Write the method  default  that return  Self  and
    set the action field with an empty string and the permission field with an hashmap that
    contains as keys the Permission values  READ, WRITE, EXECUTE , and, as values, all set
    to  false .
    Create the method  new  for Actions, that, given the arguments  action: String ,  read:
    bool ,  write: bool  and  execute: bool , return Self. In this method, create a new Self,
    setting the action field to  action  argument and the  permission  field with the key-value
    pairs corresponding to the three Permission values ( READ, WRITE, EXECUTE ) and the
    relative bool passed as arguments.
    Implement the  Default  trait for User. Write the method  default  that return  Self  and
    set the name field with the string "Guest", the Role with GUEST and the actions field as
    a new vector.
    Create also the method  change_role  for  User  that take a mutable reference to self and
    the argument  role: Role , and return  Result<(), String>
    This method should
    change the User's role as the one passed in the argument if the user is  ADMIN ,
    returning an  Ok(())
    otherwise if the user is  USER  its role can be modified to  GUEST  or it can remain
    USER , returning an  Ok(()) , but cannot be changed to  ADMIN . In this last case it
    should return a  Err()  with an error message
    Lastly if the user is  GUEST  its role cannot be changed, can be only re-set to  GUEST ,
    returning an  Ok(()) . Otherwise, it should return a  Err()  with an error message
    Create the function  sudo_change_permission  that takes as arguments  user: User ,
    string: String ,  permission: Permission  and  value: bool .
    This function should change the user permission for the action specified in the  string
    argument.
*/
#[derive(PartialEq, Eq)]
enum Role 
{
    ADMIN,
    USER,
    GUEST
}

#[derive(PartialEq, Eq, Hash)]
enum Permission
{
    READ,
    WRITE,
    EXECUTE
}

struct Action
{
    action: String,
    permissions: HashMap<Permission, bool>
}

impl Action
{
    fn new(action: String,  read: bool, write: bool, execute: bool) -> Action {
        Action {
            action: action,
            permissions: HashMap::from([(Permission::READ, read), (Permission::WRITE, write), (Permission::EXECUTE, execute)])
        }
    }
}

impl Default for Action
{
    fn default() -> Action {
        Action {
            action: String::new(),
            permissions: HashMap::from([(Permission::READ, false), (Permission::WRITE, false), (Permission::EXECUTE, false)])
        }
    }
}

struct User
{
    name: String,
    role: Role,
    actions: Vec<Action>
}

impl User
{
    fn check_action(&self, action: &str) -> bool{
        self.actions.iter().find(|a| action == a.action).is_some()
    }

    fn change_role(&mut self, role: Role) -> Result<(), String> {
        match self.role {
            Role::ADMIN => self.role = role,
            Role::USER => {
                match role {
                    Role::ADMIN => return Err(String::from("USER can't upgrade to ADMIN")),
                    _ => self.role = role
                }
            },
            Role::GUEST => {
                match role {
                    Role::ADMIN => return Err(String::from("GUEST can't upgrade to ADMIN")),
                    Role::USER => return Err(String::from("GUEST can't upgrade to USER")),
                    _ => self.role = role
                }
            }
        }

        Ok(())
    }
}

impl Default for User
{
    fn default() -> User {
        User {
            name: String::from("Guest"),
            role: Role::GUEST,
            actions: Vec::new()
        }
    }
}


impl Auth for User
{
    

    fn check_permission(&self, action: &str, permission_type: &Permission) -> bool {
        let action = self.actions.iter().find(|a| action == a.action);

        match action {
            Some(a) => {
                let permission = a.permissions.get(permission_type);

                match permission {
                    Some(b) => *b,
                    None => false
                }
            }
            None => false
        }
    }

    fn can_read(&self, action: &str) -> bool {
        self.check_permission(action, &Permission::READ) && self.check_action(action)
    }

    fn can_write(&self, action: &str) -> bool {
        self.check_permission(action, &Permission::WRITE) && self.check_action(action)
    }

    fn can_execute(&self, action: &str) -> bool {
        self.check_permission(action, &Permission::EXECUTE) && self.check_action(action)
    }
}

trait Auth
{
    fn check_permission(&self, action: &str, permission_type: &Permission) -> bool;
    fn can_read(&self, action: &str) -> bool;
    fn can_write(&self, action: &str) -> bool;
    fn can_execute(&self, action: &str) -> bool;
}

fn sudo_change_permission(user: &mut User, action: String, permission: Permission, value: bool) {
    let action = user.actions.iter_mut().find(|a| action == a.action);

    match action {
        Some(a) => {
            a.permissions.insert(permission, value);
        },
        None => { }
    }
}


fn main()
{
    println!("{:?}", find_equal(&"banana", &"anna"));
    println!("{:?}", lucky_slice(&"guglielmo"));

    let father = Person::new(String::from("Father"), None, None);
    let mother = Person::new(String::from("Mother"), None, None);
    let child = Person::new(String::from("Child"), Some(&father), Some(&mother));

    println!("{:?}", child.find_roots());

    println!("{}", skip_prefix("+39123456789", "+39"));
}