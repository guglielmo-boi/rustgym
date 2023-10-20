use std::collections::HashMap;

/* 
    1. Write a function string_reverse that takes a &str as input and returns it, reversed as a String.
*/
fn string_reverse(s: &str) -> String {
    s.chars().rev().collect()
}


/*
    2. Write a function bigger that takes two i32 and returns the bigger number (i32) without using
    another function call and additional variables.
*/
fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}


/*
    3. Write a function multiply that takes an i32, a f32 and a f64 and returns the multiplication of the
    three of them as a f64 value.
*/
fn multiply(a: i32, b: f32, c: f64) -> f64{
    (a as f64) * (b as f64) * c
}

/*
    4. Write a function e_equals_mc_squared that takes as input a f32 representing the mass, and that
    uses a globally-defined constant containing the value of the speed of light in a vacuum (expressed in
    m/s). The function outputs the energy equivalent to the mass input.
*/
const SPEED_OF_LIGHT: f32 = 299792458.0;

fn e_equals_mc_squared(mass: f32) -> f32{
    mass * SPEED_OF_LIGHT.powi(2)
}


/*
    5. Given a vector of i32, create a function max_min that returns the maximum and the minimum value
    inside that vector.
*/
fn max_min(vector: Vec<i32>) -> (i32, i32) {
    (*(vector.iter().max().unwrap()), *(vector.iter().min().unwrap()))
}


/*
    6. Write a function lord_farquaad that takes a String and outputs another String in which every
    character 'e' is substituted by the character '💥'.
*/
fn lord_farquaad(s: String) -> String {
    s.replace("e", "💥")
}


/*
    7. In the main function initialize a HashMap<String, f32> called furniture that stores the pair
    String as key and f32 as value, where the String is the name of the furniture and the f32 is its
    price. Then write a function that borrows the HashMap, takes a furniture: String as input and
    returns the corresponding f32. If there is no such furniture in the HashMap, return -1.0.
*/
fn get_furniture_price(furniture_map: &HashMap<String, f32>, furniture: String) -> f32 {
    match furniture_map.get(&furniture) {
        Some(x) => x.clone(),
        None => -1.0,
    }
}


/*
    8. Write a function append that takes a String, appends the word "foobar" to it and returns it.
    Then write a main function in which you.
    - Declare a String initialized with some text.
    - Pass the String to the function append.
    - Print the original String and the one returned by append.
*/
fn append_foobar(s: &String) -> String {
    s.to_owned() + &String::from("foobar")
}

fn append() {
    let s = String::from("Guglielmo");
    let s_foobar = append_foobar(&s);
    println!("{}", s);
    println!("{}", s_foobar);
}


/*
    9. An Armstrong number is a number that is the sum of its own digits each raised to the power of the
    number of digits.
    Write the function is_armstrong that determines whether a number is an Armstrong number.
*/
fn count_digits(num: i64) -> i64 {
    if num == 0 {
        0
    } else {
        count_digits(num / 10) + 1
    }
}

fn is_armstrong(num: i64) -> bool {
    let mut n = num;
    let d = count_digits(num);
    let mut sum = 0;

    while n > 0 {
        sum += (n % 10).pow(d as u32);
        n /= 10;
    }

    num == sum
}


/*
    10. Write a function that takes a 2x2 i32 "matrix" (2x2 tuple) as input, transposes and returns it.
*/
fn transpose(matrix: ((i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32)) {
    let ((a, b), (c, d)) = matrix;
    ((a, c), (b, d))
}


fn main()
{
    println!("{}", string_reverse("Guglielmo"));

    println!("{}", bigger(2, 5));
    
    println!("{}", multiply(2, 5.5, 10.7));
    
    println!("{}", e_equals_mc_squared(62.0));
    
    println!("{:?}", max_min(vec![0, 1, 2, 3, 4]));
    
    println!("{}", lord_farquaad(String::from("mele")));

    let mut furniture_map = HashMap::new();
    furniture_map.insert(String::from("chair"), 10.0);
    furniture_map.insert(String::from("table"), 18.0);
    furniture_map.insert(String::from("wardrobe"), 25.0);

    println!("{}", get_furniture_price(&furniture_map, String::from("chair")));

    append();

    println!("{}", is_armstrong(9));

    println!("{:?}", transpose(((0, 1), (2, 3))));
}