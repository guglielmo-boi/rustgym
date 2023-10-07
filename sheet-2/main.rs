use std::collections::HashMap;
use std::slice::Iter;

/*
    1. Write a function called modify_odd that takes a mutable reference to an vecay slice of
    integers slice and sets all odd numbers to 0.
    Then write a second function that create a Vec, filled with all numbers from 0 to 100,
    and pass it to modify_odd.
*/
fn modify_odd(slice: &mut [i32]) {
    for num in slice {
        if *num % 2 == 1 {
            *num = 0;
        }
    }
}


/*
    2. Write a function count_character that takes a string consisting of ASCII characters
    string as input and returns a HashMap. The keys of the HashMap should be the
    characters in the string, and the values should be an u32 representing how many times
    each character appears in the string.
*/
fn count_character(str: String) -> HashMap<char, u32> {
    let mut ret = HashMap::new();

    for c in str.chars() {
        ret.insert(c, 1 + if ret.contains_key(&c) { ret[&c] } else { 0 });
    }

    ret
}


/*
    3. Write a function named split_at_value that takes two arguments: a slice of i32 called
    slice and a single i32 value called value. The function should find the first element
    equal to value inside slice. It should then split the slice at the corresponding index
    and return the two resulting slices wrapped in an Option.
    If value is not found in slice, the function should return None.
*/
fn split_at_value(slice: &[i32], value: i32) -> Option<(&[i32], &[i32])> {
    if let Some(index) = slice.iter().position(|&x| x == value) {
        Some(slice.split_at(index + 1))
    } else {
        None
    }
}


/*
    4. Write a function sub_slice that takes two &Vec<i32> as input. If the second vector is
    contained inside the first one it print the corresponding slice, otherwise it print Not
    found.
*/
fn sub_slice(first: &Vec<i32>, second: &Vec<i32>) {
    if let Some(start_index) = first.iter().position(|&x| x == second[0]) {
        if first[start_index..].starts_with(second) {
            println!("{:?}", &first[start_index..start_index + second.len()]);
            return
        }
    }

    println!("Not found");
}


/*
    5. Write the following functions, for each of the functions think carefully about what is the
    best way to pass the arguments (&, &mut or passing ownership):
    Write a function max that takes a Vec of i32 and returns the maximum value inside
    it.
    Write a function swap that swaps the first and last element of a vector of i32.
    Write a function is_sorted that takes a Vec of i32 and returns a boolean indicating
    whether the vector is sorted in non-decreasing order.
    Write a function insert_if_longer that takes a Vec of String (vec) and a String
    (string). This function should insert string into vec only if the length of string
    is greater than 10.
*/
fn max(vec: &Vec<i32>) -> i32 {
    *vec.iter().max().unwrap()
}

fn swap(vec: &mut Vec<i32>) {
    let len = vec.len();

    if len >= 2 {
        vec.swap(0, len - 1);
    }
}

fn is_sorted(vec: &Vec<i32>) -> bool {
    let mut ret = true;

    for i in 1..vec.len() {
        if vec[i] < vec[i - 1] {
            ret = false
        }
    }

    ret
}

fn insert_if_longer(vec: &mut Vec<String>, str: String) {
    if str.len() > 10 {
        vec.push(str);
    }
}


/*
    6. Write a function build_vector that takes a Iter<i32> and returns the Vec<i32>
    containing all the elements of the iterator.
*/
fn build_vector(iter: Iter<i32>) -> Vec<i32> {
    let ret: Vec<i32> = iter.cloned().collect();
    ret
}


/*
    7. Write a function pancake_sort that takes a &mut Vec<i32> and sorts it using the
    pancake sort algorithm.
*/
fn find_max_index(slice: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut max_value = slice[0];
    
    for (i, &value) in slice.iter().enumerate() {
        if value > max_value {
            max_value = value;
            max_index = i;
        }
    }
    
    max_index as i32
}

fn flip(vec: &mut Vec<i32>, end: usize) {
    let mut start = 0;
    let mut end = end;

    while start < end {
        vec.swap(start, end);
        start += 1;
        end -= 1;
    }
}

fn pancake_sort(vec: &mut Vec<i32>) {
    let len = vec.len() as i32;
    let mut current_size = len;

    while current_size > 1 {
        let max_index = find_max_index(&vec[0..current_size as usize]);

        if max_index != current_size - 1 {
            flip(vec, max_index as usize);
            flip(vec, (current_size - 1) as usize);
        }

        current_size -= 1;
    }
}


/*
    8. Write a function merge that takes two &[i32] and returns a Vec<i32>. The returned
    vector should contain the elements of the two passed elements sorted, you can assume
    that the passed slices are sorted.
*/
fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut ret = Vec::new();
    
    ret.extend(a);
    ret.extend(b);
    ret.sort();
    
    ret
}


/*
    9. Create a Vec that can contain both an i32 and a String.
*/
enum Foo
{
    Num(i32),
    Str(String)
}


/*
    10. Write these enums to represent a mathematical expression:
    One enum is called  Operator  and can be:  Add, Sub, Mul, Div.
    One enum is called  Expression  an can be:
    Number (contain inside an i32)
    Operation (contain inside a left Expression, a right Expression and an
    Operator)
    Note: the left and right expression must be wrapped around a Box
    You will see Boxes further into the course, from now you just need to know that you can
    build a box using
    and you can get the value inside the box by dereferencing it
    Write a function  evaluate_expression  that take as input an Expression, and return a
    Result with a i32 if the result is evaluated correctly, or a string if an error occurs.
    Box<Expression>.
    let my_box = Box::new(my_expression)
    let value_inside = *my_box
*/

enum Operator
{
    Add,
    Sub,
    Mul,
    Div
}

enum Expression
{
    Number(i32),
    Operation(Box<Expression>, Box<Expression>, Operator)
}

fn evaluate_expression(expression: Expression) -> Result<i32, String> {
    match expression {
        Expression::Number(x) => Ok(x),
        Expression::Operation(e1, e2, op) => {
            let lhs = evaluate_expression(*e1);
            let rhs = evaluate_expression(*e2);

            match lhs {
                Ok(x) => {
                    match rhs {
                        Ok(y) => {
                            match op {
                                Operator::Add => Ok(x + y),   
                                Operator::Sub => Ok(x - y),
                                Operator::Mul => Ok(x * y),
                                Operator::Div => {
                                    if y == 0 {
                                        Err(String::from("Can't divide by 0!"))
                                    } else {
                                        Ok(x / y)
                                    }
                                }
                            }
                        },

                        Err(s) => Err(s)
                    }
                },

                Err(s) => Err(s)
            }
        }
    }
}

fn main()
{ 
    let mut slice = [0, 1, 2, 3, 4];
    modify_odd(&mut slice);
    println!("{:?}", slice);

    let cc = count_character(String::from("apple"));
    println!("{:?}", cc);

    println!("{:?}", split_at_value(&[0, 1, 2, 3, 4], 2).unwrap().0);

    sub_slice(&vec![0, 1, 2, 3, 4], &vec![2, 3, 4]);

    max(&vec![0, 1, 2, 3, 4]);
    swap(&mut vec![0, 1, 2, 3, 4]);
    println!("{}", is_sorted(&vec![0, 1, 2, 3, 4]));

    insert_if_longer(&mut vec![String::from("str")], String::from("reallylongstring"));

    build_vector(vec![0, 1, 2, 3, 4].iter());
    
    pancake_sort(&mut vec![0, 2, 3, 1, 4]);

    merge(&[0, 1, 2, 3, 4], &[0, 1, 2, 3, 4]);

    let mut vec = Vec::new();
    vec.push(Foo::Num(10));
    vec.push(Foo::Str(String::from("apple")));

    let e1 = Expression::Operation(Box::new(Expression::Number(10)), Box::new(Expression::Number(15)), Operator::Add);
    let result = evaluate_expression(Expression::Operation(Box::new(expression), Box::new(Expression::Number(2)), Operator::Mul));
    println!("{:?}", result);
}