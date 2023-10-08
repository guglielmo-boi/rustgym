use std::collections::HashMap;
use std::fmt;

/*
    1. Given a number determine whether it is valid per the Luhn formula, creating the function is_it_luhn
    The Luhn algorithm is a simple checksum formula used to validate a variety of identification numbers, such as credit card numbers. Check
    if a given string is valid
    Strings of length 1 or less are not valid. Spaces are allowed in the input, but they should be stripped before checking. All other non-digit
    characters are disallowed.
    Ex 1: valid card number
    4539 3195 0343 6467
    The first step is to double every second digit, starting from the right. We will be doubling
    43 39 04 66
    If doubling the number results in a number greater than 9 then subtract 9 from the product. The results of our doubling:
    8569 6195 0383 3437
    Then sum all the digits is 80. If the sum is divisible by 10, then the number is valid.
*/
fn is_it_luhn(s: String) -> bool {
    if s.len() <= 1 {
        false
    } else {
        let vec: Vec<u32> = s.chars().rev().flat_map(|c| c.to_digit(10)).collect();
        let mut sum: u32 = 0;

        for (i, v) in vec.iter().enumerate() {
            if i % 2 == 1 {
                sum += if (v * 2) > 9 { (v * 2) - 9 } else { v * 2 };
            } else {
                sum += v
            }
        }

        sum % 10 == 0
    }
}


/*
    2. For the following examples, decide which of the composite data structures is better (enum or structs). Then implement them
    - you are Rick, a car shop owner, and you have to choose the fuel of your car between Diesel, Gasoline, LPG, Methane and Electric
    - you have to program the recognition of the IP version of a router. Remember that IPv4 is formatted with 4 groups of 3 integer values
    (from 0 to 255), IPv6 is instead formatted with 8 groups of 4 hexadecimal (so no strings!) values.
    - you have to track points in a 3-dimensional space, with the f64 values for each dimension.
*/
enum Fuel
{
    Diesel,
    Gasoline,
    LPG,
    Methane,
    Electric
}

enum IP
{
    IPv4(u8, u8, u8, u8),
    IPv6(u16, u16, u16, u16, u16, u16, u16, u16)
}

struct Point
{
    x: f64,
    y: f64,
    z: f64
}


/*
    3. In Trento there is an automated car park with a camera that recognises the number plate of the car. Your task is to associate the number
    plate with the owner of the car in order to track the price for each car owner. Create a main with an appropriate data structure already
    initialised with some data. Create a function recognise_owner that, given the data structures mentioned above and the number of car
    plate, returns an Option al value of the owner of the car.
*/
fn recognise_owner(map: &HashMap<String, i32>, plate_number: String) -> Option<i32> {
    map.get(&plate_number).copied()
}


/*
    4. Create a vending machine:
    1. Define an enum Item that lists the available items inside the machine (e.g. Coke, Coffee, ecc).
    2. Define the enum Coin that contains the coin type accepted by the machine (e.g. Coin::FiftyCents , Coin::Euro2 ). And
    implement the method to_cents that convert a Coin variant into a u32 representing the number of cents (e.g. 1€ => 100 , 20¢ =>
    20 ).
    3. Define the struct VendingMachine that has the following fields:
    coins: u32
    This field represents the number of cents currently held inside the machine (e.g. if the user inserted 1€ and 10¢, then the coins
    should be at 110 ).
    items: HashMap<Item, usize>
    This field should associate an Item type to the number of available items to buy.
    Now implement the following methods for VendingMachine :
    new method that takes an HashMap of Items contained in the VendingMachine initially and returns a new instance of the
    struct , set coins to 0.
    add_item takes an Item variant and a usize ; increments the number of the specified type of items contained by the machine
    insert_coin takes a Coin variant and increment the field coins by the right value.
    get_item_price takes an &Item variant and returns a u32 item price.
    buy takes a Item variant and returns a Result<u32, &str> if you have enough money it returns the change, if you don't, it
    returns the error as a &str .
    Note:
    For using Item as an HashMap key it needs to implement PartialEq , Eq and Hash . Keep in mind that you can derive them.
*/
#[derive(Eq, Hash, PartialEq)]
enum Item
{
    Water,
    Coffee,
    Coke,
    Juice
}

enum Coin
{
    TwentyCents,
    FiftyCents,
    OneEuro,
    TwoEuros
}

impl Coin
{
    fn to_cents(&self) -> u32 {
        match self {
            TwentyCents => 20,
            FiftyCents => 50,
            OneEuro => 100,
            TwoEuros => 200
        }
    }
}

struct VendingMachine
{
    coins: u32,
    items: HashMap<Item, usize>
}

impl VendingMachine
{
    fn new(items: HashMap<Item, usize>) -> VendingMachine {
        VendingMachine {
            coins: 0,
            items: items
        }
    }

    fn add_item(&mut self, item: Item, count: usize) {
        let val = self.items.get_mut(&item);

        if val.is_none() {
            self.items.insert(item, count);
        } else {
            *val.unwrap() += count;
        }
    }

    fn insert_coin(&mut self, coin: Coin) {
        self.coins += coin.to_cents();
    }

    fn get_item_price(item: &Item) -> u32 {
        match item {
            Water => 50,
            Coffee => 70,
            Coke => 150,
            Juice => 100
        }
    }

    fn buy(&mut self, item: Item) -> Result<u32, &str> {
        let price = Self::get_item_price(&item);

        if price <= self.coins {
            let ret = self.coins - price;
            self.coins = 0;
            Result::Ok(ret)
        } else {
            Result::Err("Not enough money!")
        }
    }
}


/*
    5. Implement two tuple structs named Date and Hour . The former takes u8 , u8 and u16 and the latter two u8
    Implement a BoxShipping struct, with the fields name: String , barcode: String , shipment_date: Date and shipment_hour: Hour`
    Make BoxShipping displayable both with {:?} as well as with {} argument in the println! macro.
    Note: Date and Hour structs should be formatted correctly, ex. 12/01/2001 and 09:00
*/
#[derive(Debug)]
struct Date(u8, u8, u16);
#[derive(Debug)]
struct Hour(u8, u8);

#[derive(Debug)]
struct BoxShipping
{
    name: String,
    barcode: String,
    shipment_date: Date,
    shipment_hour: Hour
}

impl std::fmt::Display for BoxShipping 
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "name: {}, barcode: {}, shipment_date: {:?}, shipment_hour: {:?}", self.name, self.barcode, self.shipment_date, self.shipment_hour)
    }
}


/*
    6. How was that book called? Programming crust? Nevermind.
    Create BUP's library system. It should be able to store books, articles and magazines.
    Each book, article and magazine should have a name, a code and a year of publication.
    Books should also have an author and a publishing company.
    Articles should have an orchid.
    Magazines should have a number and a month.
    Then implement the methods to add a book, an article and a magazine to the library system.
    Finally, implement a method to print the library system via the {} argument in the println! macro.
*/
#[derive(Debug)]
struct PaperItem
{
    name: String,
    code: String,
    publication_year: i32
}

#[derive(Debug)]
struct Book
{
    paperItem: PaperItem,
    author: String,
    publishing_company: String
}

#[derive(Debug)]
struct Article
{
    paperItem: PaperItem,
    orchid: String
}

#[derive(Debug)]
struct Magazine
{
    paperItem: PaperItem,
    number: u32,
    month: u8
}

struct LibrarySystem
{
    books: Vec<Book>,
    articles: Vec<Article>,
    magazines: Vec<Magazine>
}

impl LibrarySystem
{
    fn new() -> LibrarySystem {
        LibrarySystem {
            books: Vec::new(),
            articles: Vec::new(),
            magazines: Vec::new()
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn add_article(&mut self, article: Article) {
        self.articles.push(article);
    }

    fn add_magazine(&mut self, magazine: Magazine) {
        self.magazines.push(magazine);
    }
}

impl std::fmt::Display for LibrarySystem 
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "books: {:?}\narticles: {:?}\nmagazines: {:?}", self.books, self.articles, self.magazines)
    }
}


/*
    7. Create a module called point that inside has a struct Point with the fields x: f32, y: f32.
    Create the following methods:
    - new that initializes the Point
    - distance that borrow a Point and returns the distance between the two points
    Create then another module called line that has a struct `Line` with the fields `start: Point`, `end: Point`,
    `m: f32` and `q: f32
    you have to implement the new method that takes two points and calculates the slope and the intercept of the line m and q
    contains that borrow a p: Point and returns a Result<_, String> . The function should check if the Line contains the borrowed
    point
    Create a third module called test that has a function `test` that creates a line and a point and tests the
    `contains` method.
*/
pub mod point
{
    pub struct Point
    {
        pub x: f32,
        pub y: f32
    }

    impl Point
    {
        pub fn new(x: f32, y: f32) -> Point {
            Point {
                x: x,
                y: y
            }
        }

        pub fn distance(&self, point: &Point) -> f32 {
            ((self.x.powi(2) - point.x.powi(2)) + (self.y.powi(2) - point.y.powi(2))).sqrt()
        }
    }
}

pub mod line
{
    use point::Point;

    pub struct Line 
    {
        start: Point,
        end: Point,
        m: f32,
        q:f32
    }

    impl Line
    {
        pub fn new(start: Point, end: Point) -> Line {
            let m = (end.y - start.y) / (end.x - start.x);
            let q = start.y - (start.x * m);

            Line {
                start: start,
                end: end,
                m: m,
                q: q
            }
        }

        pub fn contains(&self, point: &Point) -> Result<String, String> {
            if point.y == (self.m * point.x) + self.q {
                Result::Ok(String::from("Line contains the point!"))
            }  else {
                Result::Err(String::from("Line does not contain the point!"))
            }
        }
    }
}


/*
    8. Create a module called sentence that has a struct Sentence with a field words: Vec<String>.
    Create the following methods for the struct Sentence:
    new_default that initializes the field words with nothing in it.
    new that takes a &str, splits it by whitespaces and inserts every word inside the words field.
    Create another module test with the function magic_sentence that mutually borrows a HashMap<i32, Sentence>, a i: i32 and
    a j: i32 and returns a Result<Sentence, &str>
    The function checks if the sentences at the two indexes exist and if so, creates a Sentence with all the equal words in the same
    position (same index) present in the Sentence s.
    If no words are found or if the indexes are not present in the HashMap, reutrn an Err(&str) .
    Ex. the sentence "Hello my name was cool yesterday" and the sentence "Hi my name is cool" should result in the sentence "my name
    cool".
*/
pub mod sentence 
{
    #[derive(Debug)]
    pub struct Sentence
    {
        pub words: Vec<String>
    }

    impl Sentence
    {
        pub fn new_default() -> Sentence {
            Sentence {
                words: Vec::new()
            }
        }

        pub fn new(s: &str) -> Sentence {
            Sentence {
                words: s.split(' ').map(|x| String::from(x)).collect()
            }
        }
    }
}

pub mod test
{
    use std::collections::HashMap;
    use sentence::Sentence;

    pub fn magic_sentence(map: &HashMap<i32, Sentence>, i: i32, j: i32) -> Result<Sentence, &str> {
        let a = map.get(&i);
        let b = map.get(&j);

        if a.is_some() && b.is_some() {
            let s1 = &a.unwrap().words;
            let s2 = &b.unwrap().words;
            let mut i = 0;
            let mut j = 0;

            let mut ret = Sentence::new_default();

            while i < s1.len() && j < s2.len() {
                if s1[i] == s2[i] {
                    ret.words.push(s1[i].clone());
                }

                i += 1;
                j += 1;
            }

            if !ret.words.is_empty() {
                return Ok(ret);
            }
        }

        Err("Error")
    }
}


fn main()
{
    println!("{}", is_it_luhn(String::from("4539 3195 0343 6467")));

    let mut map = HashMap::new();
    map.insert(String::from("AAA"), 10);
    println!("{:?}", recognise_owner(&map, String::from("AAA")));

    let boxShipping = BoxShipping {
        name: String::from("foo"),
        barcode: String::from("bar"),
        shipment_date: Date(07, 10, 2023),
        shipment_hour: Hour(14, 33)
    };

    println!("{}", boxShipping);
    println!("{:?}", boxShipping);

    let mut librarySystem = LibrarySystem::new();

    let book = Book {
        paperItem: PaperItem {
            name: String::from("Book"),
            code: String::from("AAA"),
            publication_year: 2023
        },
        author: String::from("Guglielmo"),
        publishing_company: String::from("Unitn")
    };

    librarySystem.add_book(book);
    println!("{}", librarySystem);

    let s = point::Point::new(1.0, 2.0);
    let e = point::Point::new(3.0, 5.0);
    let ln = line::Line::new(s, e);
    println!("{:?}", ln.contains(&point::Point::new(1.0, 3.0)));

    let s1 = sentence::Sentence::new("this is a sentence");
    let s2 = sentence::Sentence::new("this is also a sentence");
    let mut map = HashMap::new();
    map.insert(0, s1);
    map.insert(1, s2);
    println!("{:?}", test::magic_sentence(&map, 0, 1));
}   