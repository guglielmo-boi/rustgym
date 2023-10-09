## Question 1
###### Will the following code compile?
```
let x = String::from(”hello”);
    let y = x;
    println!("{}, world!", y);
    println!("{}, world!", x);
```
- [ ] no, x is moved
- [ ] no, x does not implement Display
- [ ] yes
- [ ] no, x is re-borrowed

## Question 2
###### What does this evaluate to?
```
{
    let mut s1 = String::from(“Hello!“);
    {
        let mut s2 = &s1;
        s2.push_str("World!");
        println!("{}", s2)
    }
}
```
- [ ] prints: "Hello!World!"
- [ ] prints: "Hello! World!"
- [ ] prints: "Hello!"
- [ ] error 


## Question 3
###### What does this evaluate to?
```
{
	let mut a = 50;
	let b = &mut a;
	a = 20;
	println!(“a: {}, b: {}“,a,b);
}
```

- [ ] error
- [ ] print: "a: 20, b: 50"
- [ ] print: "a: 50, b: 50"
- [ ] print: "a: 20, b: 20"

## Question 4
###### What is printed?
```
fn foo(s: &mut String) -> usize {
    s.push_str("Bob");
    s.len()
}
fn main() {
    let mut s1 = String::from(”Alice”);
    println!("{}",foo(&s1));
}
```

- [ ] 5
- [ ] 8
- [ ] error
- [ ] 0

## Question 5
###### What is printed?
```
let mut s1 = String::from(“hello”);
{ 
    let s2 = &s1;
    let s3 = &s1;
    let s4 = &mut s1;
    println!(”String is {}”,s1);
    println!(”String is {}”,s2);
    println!(”String is {}”,s3);
    println!(”String is {}”,s4);

}
s1.push_str(“ there”);
println!(”String is {}”,s1);
```

- [ ] \
String is hello there\
String is hello there\
String is hello there\
String is hello there\
String is hello there

- [ ] \
String is hello\
String is hello\
String is hello\
String is hello there\
String is hello there


- [ ] error

- [ ] \
String is hello\
String is hello\
String is hello\
String is hello\
String is hello there

## Question 6
###### What does this code print?
```
pub fn main(){
  let mut array = vec![10, 20, 30];
   function(&mut array);
  println!(”{}”, array[0]);
}
fn function(array: &mut Vec<u32>) {
    array[0] = 100;
}
```

- [ ] error
- [ ] 10
- [ ] 100
- [ ] 10, 20, 30


## Question 7
###### Owner of str’s data at HERE ?
```
fn foo(str:String) {
    let x = str; 
    let y = &x;
    let z = x;
    let w = &z;
    // HERE
}
```
- [ ] w
- [ ] z
- [ ] y
- [ ] x

## Question 8
###### What does this code print?
```
fn main() {
  let array = vec![10, 20, 30];
  function(array);
  println!("{}", array[0]);
}

fn function(array: Vec<u32>) {
    println!(”{}”, array[0]);
}
```

- [ ] \
10\
10, 20, 30\



- [ ]  \
error


- [ ] \
10\
10


- [ ] \
  10, 20, 30\
10, 20, 30

## Question 9
write enum XX with variants Y1 Y2, the former takes 2 ints, the latter a string.

write a function `data` that takes an XX and then returns the contents of y1 multiplied by each other, or the length of Y2

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0 lastrow">
<td class="cell c0" style=""><pre class="tablecell">
pub fn main(){
    let xx1 = XX::Y1(2,3);
    println!("data {}",data(&amp;xx1));
    let xx1 = XX::Y2(String::from("test"));
    println!("data {}",data(&amp;xx1));
}
</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">
data 6
data 4</pre></td>
</tr>
</tbody>
</table>

## Question 10
write enum Z with variants Y1 Y2, the former takes 2 i32, the latter a bool and a string

write a function `maybelen` that returns the optional length of the string

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0 lastrow">
<td class="cell c0" style=""><pre class="tablecell">pub fn main(){
    let z1 = Z::Y1(1,2);
    let z2 = Z::Y2(true,String::from("new"));
    println!("len {:?}", maybelength(&amp;z1));
    println!("len {:?}", maybelength(&amp;z2));
}</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">len None
len Some(3)</pre></td>
</tr>
</tbody>
</table>

## Question 11
write an enum X with 1 variant Y with a String

write a struct X with 1 field `i` with a String

use the module system to give the two separate namespaces

the enum's module is called `enumx`, the structs' module is called `structx`



write a function `longer` in another module `modfun` that takes 2 arguments, enum X and struct X

and it returns the longer length of the content of the arguments

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0 lastrow">
<td class="cell c0" style=""><pre class="tablecell">pub fn main(){
    let ex = enumx::X::Y(String::from("test"));
    let sx = structx::X{i:String::from("asd")};
    println!("Longer {}", modfun::longer(&amp;ex,&amp;sx));
    let ex = enumx::X::Y(String::from("asdasd"));
    println!("Longer {}", modfun::longer(&amp;ex,&amp;sx));
}</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">Longer 4
Longer 6</pre></td>
</tr>
</tbody>
</table>



## Question 12
write a `maybesqrt` function that takes a number: i32 and returns an optional i32 value with the result of the square root of the number

// recall .sqrt is a function of f32

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0 lastrow">
<td class="cell c0" style=""><pre class="tablecell">pub fn main(){
    println!("Maybesqrt 4 {:?} ", maybesqrt(4));
    println!("Maybesqrt -4 {:?} ", maybesqrt(-4));
}</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">Maybesqrt 4 Some(2)
Maybesqrt -4 None</pre></td>
</tr>
</tbody>
</table>

## Question 13

write a `Balance` struct with a field `amt : i32`.

Add a `maybewithdraw` method that takes a n: i32 and subtracts n from the amount and returns the optional new total, unless that would be a negative number.

<table class="coderunnerexamples" id="yui_3_17_2_1_1674510452737_178">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody id="yui_3_17_2_1_1674510452737_177"><tr class="r0 lastrow" id="yui_3_17_2_1_1674510452737_176">
<td class="cell c0" style="" id="yui_3_17_2_1_1674510452737_175"><pre class="tablecell" id="yui_3_17_2_1_1674510452737_174">pub fn main(){
    let b = Balance{amt:100};
    println!("maybewith {:?}", b.maybewithdraw(10));
    println!("maybewith {:?}", b.maybewithdraw(200));
}</pre></td>
<td class="cell c1 lastcol" style="" id="yui_3_17_2_1_1674510452737_181"><pre class="tablecell">maybewith Some(90)
maybewith None</pre></td>
</tr>
</tbody>
</table>

## Question 14

write a function `prevchar` that takes a char and returns its precedent value as i32, in case of 'a' it returns '`' 

// check out what functions `std` offers for `char` 



write a function `replwithprev` that takes a string and replaces all its characters with the previous one, unless the string contains an 'a'.

the return should be a result of either the replaced string, or a void error

<table class="coderunnerexamples" id="yui_3_17_2_1_1674510452737_185">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody id="yui_3_17_2_1_1674510452737_184"><tr class="r0 lastrow" id="yui_3_17_2_1_1674510452737_183">
<td class="cell c0" style=""><pre class="tablecell">pub fn main(){
    println!("char {}, prev {}", 'c', prevchar('c'));
    println!("char {}, prev {}", 'a', prevchar('a'));
    println!("char {}, prev {}", 'z', prevchar('z'));
    let mut s = String::from("Pign");
    println!("S {}",s);
    let ret = replwithprev(&amp;mut s);
    println!("Returned: {:?}",ret);
    let mut s = String::from("pigna");
    println!("S {}",s);
    let ret = replwithprev(&amp;mut s);
    println!("Returned: {:?}",ret);
}</pre></td>
<td class="cell c1 lastcol" style="" id="yui_3_17_2_1_1674510452737_182"><pre class="tablecell">char c, prev b
char a, prev `
char z, prev y
S Pign
Returned: Ok("Ohfm")
S pigna
Returned: Err(())</pre></td>
</tr>
</tbody>
</table>

## Question 15

write a struct X with 2 fields s : string and i : i32

write a struct Y with 2 fields z : bool and c : string



give each struct a constructor function `new`. The default values are "xxx", 10 for X and true, "op" for Y.

give each struct a method `getstring` for replacing the string with "", moving the string out of the struct and returning said string

// use std::mem::replace



write a function `swapstr` that takes a X and a Y and then moves s into c and c into s

make D displayable both with :? the formatter as well as with a {} argument in println. With a {} argument, see the example for the result

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0 lastrow">
<td class="cell c0" style=""><pre class="tablecell">pub fn main(){
let x = X::new();
    let y = Y::new();
    println!("X {:?} - Y {:?}", x, y);
    let (x,y) = swapstr(x,y);
    println!("X {} - Y {}", x, y);
}</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">X X { s: "xxx", i: 10 } - Y Y { b: true, c: "op" }
X S op, I 10 - Y B true, C xxx</pre></td>
</tr>
</tbody>
</table>

## Question 16

write enum C with variants C1, C2, the former takes two i32's, the latter a bool and a string

write struct D with a c field of type C and a s field of type String



give D a function `new` with no parameter that returns a D

give D a function `newfromc` that returns a D given a C, and initialises the s field to the content of C2, or to "not there"

give D a method `larger` that returns the max length between s and the enum's possible string.

make D displayable both with :? the formatter as well as with a {} argument in println. With a {} argument, see the example for the result

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0 lastrow">
<td class="cell c0" style=""><pre class="tablecell">pub fn main () {
    let c1 = C::C1(0,1);
    let c2 = C::C2(true, String::from("no way jose"));
    println!("gotten {:?}",D::new());
    let d1 = D::new_with_C(c1);
    println!("dbg {:?}",d1);
    println!("fmt {}",d1);
    let d2 = D::new_with_C(c2);
    println!("dbg {:?}",d2);
    println!("fmt {}",d2);
    println!("larger {}",d1.larger());
    println!("larger {}",d2.larger());
}</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">gotten D { c: C1(0, 0), s: "" }
dbg D { c: C1(0, 1), s: "not there" }
fmt D: not there with C1(0, 1)
dbg D { c: C2(true, "no way jose"), s: "no way jose" }
fmt D: no way jose with C2(true, "no way jose")
larger 9
larger 11</pre></td>
</tr>
</tbody>
</table>

## Question 17

write a function `swapelconcat` that takes a vector of Strings and 2 indices i, j.
the function returns an optional pointer to the same vector

the function mutates the vector and replaces each element at index i and j with the concatenation of the previous element at position i and j followed by the other element (at position j and i respectively)


## Question 18

write a function `veclengths` that takes 1 vector of Strings and returns a vector of i32 where an element at position j is the length of the string at position j f the input vector

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0 lastrow">
<td class="cell c0" style=""><pre class="tablecell">pub fn main () {
    let mut v1 = vec![String::from("some");12];
    println!("Lengths {:?}", veclengths(&amp;v1));
}</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">Lengths [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4]</pre></td>
</tr>
</tbody>
</table>

## Question 19

write a function `removeelfromvector` that takes a Vector of String and a number n: usize and it removes the first element from the vector whose content has length n

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0 lastrow">
<td class="cell c0" style=""><pre class="tablecell">pub fn main(){
    let mut v: Vec&lt;String&gt; = vec![String::from("what");4];
    v.push(String::from("now"));    v.push(String::from("what"));
    println!("{:?}",v);
    removeelfromvector(&amp;mut v, 3);
    println!("{:?}",v);
}</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">["what", "what", "what", "what", "now", "what"]
["what", "what", "what", "what", "what"]</pre></td>
</tr>
</tbody>
</table>

