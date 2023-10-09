/*
    2. write enum A with variants A2 A1, the former takes two chars, the latter three ints write enum B with variants B1 B2, the former takes 2 ints, the latter a String
    write a function bfroma that takes an A and then returns a B where B1 has ints that are the index of their A2 counterparts in the alphabet,and B2 has the sting that is the concatenation of the three floats in A1, separated by a -
*/
enum A
{
    A2(char, char),
    A1(i32, i32, i32)
}

enum B
{
    B1(i32, i32),
    B2(String)
}

fn bfroma(a: A) -> B {
    match a {
        A::A2(x, y) => B::B1((x as u32 - 'a' as u32) as i32, (y as u32 - 'a' as u32) as i32),
        A::A1(x, y, z) => B::B2(format!("{}-{}-{}",x.to_string(), y.to_string(), z.to_string()))
    }
}


/*
    3. write enum Z with variants Y1 Y2, the former takes 2 i32, the latter a bool and a string write a function maybesum that returns the optional sum of the i32's
*/
enum Z
{
    Y1(i32, i32),
    Y2(bool, String)
}

fn maybesum(z: Z) -> Option<i32> {
    match z {
        Z::Y1(a, b) => Some(a + b),
        Z::Y2(_, _) => None
    }
}


/*
    4. write an enum X with 1 variant Y with an i32 write a struct X with 1 field i with an i32 use the module system to give the two separate namespaces the enum's module is called enumx, the structs' module is called structx
    write a function larger in another module modfun that takes 2 arguments, enum X and struct X and it returns the larger content of the arguments
*/
pub mod enumx
{
    pub enum X
    {
        Y(i32)
    }
}

pub mod structx
{
    pub struct X
    {
        pub i: i32
    }
}

pub mod modfun
{
    use super::enumx;
    use super::structx;

    pub fn larger(enumx_x: enumx::X, structx_x: structx::X) -> i32 {
        let enumx::X::Y(i) = enumx_x;

        if i > structx_x.i {
            i
        } else {
            structx_x.i
        }
    }
}


/*
    5. write a maybesqrt function that takes a number: i32 and returns an optional i32 value with the result of the square root of the number // recall .sqrt is a function of f32
*/
fn maybesqrt(number: i32) -> Option<i32> {
    match number {
        0.. => Some((number as f32).sqrt() as i32),
        _ => None

    }
}


/*
    6. write a Balance struct with a field amt : i32 and a field active:bool Add a maybericher method that takes another Balance b and returns true if this one is richer, false if b is richer, unless either Balance is not active
*/
struct Balance
{
    amt: i32,
    active: bool
}

impl Balance
{
    fn maybericher(&self, other: &Balance) -> Option<bool> {
        if self.active && other.active {
            Some(self.amt > other.amt)
        } else {
            None
        }
    }
}

fn main()
{

}