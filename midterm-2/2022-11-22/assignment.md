# Ex 1
define the Doublable trait with a method `gimme_double`
implement Doublable for i32, `gimme_double` returns a new i32 that is twice self
implement Doublable for String, `gimme_double` returns a new String that is self concatenated with self

implement a function `printdouble` that takes a `Doublable`
and prints the argument and its `gimme_double` using the ":?" formatter
    it behaves as the example:
    doubling 5 is 10
    doubling "what" is "whatwhat"

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0">
<td class="cell c0" style=""><pre class="tablecell">pub fn main() {
            let x = 5;
            printdouble(x);
            let s = "what".to_string();
            println!("normal s: {:?}",s);
            printdouble(s);
            let y = 8;
            printdouble(y);
        }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">doubling 5 is 10
normal s: "what"
doubling "what" is "whatwhat"
doubling 8 is 16</pre></td>
</tr>
<tr class="r1 lastrow">
<td class="cell c0" style=""><pre class="tablecell">pub fn main() {
            let x = 10;
            printdouble(x);
            let s = "who".to_string();
            println!("normal s: {:?}",s);
            printdouble(s);
            let y = 8;
            printdouble(y);
        }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">doubling 10 is 20
normal s: "who"
doubling "who" is "whowho"
doubling 8 is 16</pre></td>
</tr>
</tbody>
</table>


# Ex 2
Define a struct `Wrapper` that contains a field `v` of type `Vec<i32>`
define an iterator for `Wrapper` to cycle over the elements of the vector
the iterator will skip every other element, effectively accessing only those
at odd index in the inner vector (the first element is at index 0)

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0">
<td class="cell c0" style=""><pre class="tablecell">    pub fn main(){
        let w = Wrapper{v:vec![1,2,3,4,5,6,7,8]};
        let mut iw = w.iter();
        println!("first: {}",iw.next().unwrap());
        for el in iw{
            println!("evens: {}",el);
        }
    }
</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">first: 2
evens: 4
evens: 6
evens: 8</pre></td>
</tr>
<tr class="r1 lastrow">
<td class="cell c0" style=""><pre class="tablecell">    pub fn main(){
        let w = Wrapper{v:vec![11,12,13,14,15,16,17,18]};
        let mut iw = w.iter();
        println!("first: {}",iw.next().unwrap());
        for el in iw{
            println!("evens: {}",el);
        }
    }
</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">first: 12
evens: 14
evens: 16
evens: 18</pre></td>
</tr>
</tbody>
</table>


# Ex 3
write a function `basicbox_sum` that takes a vector of Strings and returns a vector of Boxes of usizes
the returned vector contains all the lengths of the input vector followed by a final element that sums all the previous lengths

<table class="coderunnerexamples" id="yui_3_17_2_1_1674583154699_91">
<thead id="yui_3_17_2_1_1674583154699_90">
<tr id="yui_3_17_2_1_1674583154699_89">
<th class="header c0" style="" scope="col" id="yui_3_17_2_1_1674583154699_88">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0">
<td class="cell c0" style=""><pre class="tablecell">        pub fn main() {
            let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
            println!("boxed s: {:?}", basicbox_sum(s));
        }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">boxed s: [3, 5, 4, 12]</pre></td>
</tr>
<tr class="r1 lastrow">
<td class="cell c0" style=""><pre class="tablecell">        pub fn main() {
            let s = vec!["nope".to_string(), "game".to_string(), "bananas".to_string()];
            println!("boxed s: {:?}", basicbox_sum(s));
        }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">boxed s: [4, 4, 7, 15]</pre></td>
</tr>
</tbody>
</table>

# Ex 4
take the following `List` and `Node` structs
define these functions and methods for `List`, each one defines how many points it yields
    - [0.5] new: returns an empty list
    - [0.5] size: returns the i32 length of the list
    - [6] add: takes an element `e:T` and a position `p:i32` where to insert the element in the list
        and it returns a Result<(),String> 
        The function inserts the element `e` at position `p` or returns the Err string "wrong position" if
        the list has fewer than `p` elements. That is:
            adding `16` at position `2` to `[10,20,30]` will return `[10,20,16,30]`.
            adding `16` at position `3` to `[10,20,30]` will return `[10,20,30,16]`.
            adding `16` at position `4` to `[10,20,30]` will return `[10,20,30]`.
    - [2] prepend: takes an element `e:T` and adds it at the head of the list
    - [2] append: takes an element `e:T` and adds it as the last element of the list
    - [4] get: takes a position `p` and returns an optional pointer to the `p`th T-typed element in the list
         (That is, a pointer to the element, not a pointer to the Node)
Note: the tests already include the code below, all you need to paste as the answer are the `impl` blocks
and possible imports (use ...).

```rust
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    len: i32,
}
type Link<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
#[derive(Debug)]
pub struct Content {
    s : String, b : bool, i : i32,
}
impl Content {
    pub fn new_with(s:String, b:bool, i:i32) -> Content {
        return Content{s,b,i};
    }
}

```

<table class="coderunnerexamples">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody><tr class="r0">
<td class="cell c0" style=""><pre class="tablecell">#[derive(Debug)]
    pub struct List&lt;T&gt; {
        head: Link&lt;T&gt;,
        len: i32,
    }
    type Link&lt;T&gt; = Option&lt;Box&lt;Node&lt;T&gt;&gt;&gt;;
    #[derive(Debug)]
    struct Node&lt;T&gt; {
        elem: T,
        next: Link&lt;T&gt;,
    }

#[derive(Debug)]
    pub struct Content {
        s : String, b : bool, i : i32,
    }
    impl Content {
        pub fn new_with(s:String, b:bool, i:i32) -&gt; Content {
            return Content{s,b,i};
        }
    }

        pub fn main(){
            let l : List&lt;i32&gt; = List::new();
            println!("{:?}",l);
            assert_eq!(l.size(),0);
            let l = List{ head: Some(Box::new(Node{ elem: 4, next: None })), len: 1 };
            assert_eq!(l.size(),1);
            let s : String = format!("{:?}",l);
            assert_eq!(s.contains("Vec"),false);
        }
</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">List { head: None, len: 0 }</pre></td>
</tr>
<tr class="r1">
<td class="cell c0" style=""><pre class="tablecell">#[derive(Debug)]
    pub struct List&lt;T&gt; {
        head: Link&lt;T&gt;,
        len: i32,
    }
    type Link&lt;T&gt; = Option&lt;Box&lt;Node&lt;T&gt;&gt;&gt;;
    #[derive(Debug)]
    struct Node&lt;T&gt; {
        elem: T,
        next: Link&lt;T&gt;,
    }

#[derive(Debug)]
    pub struct Content {
        s : String, b : bool, i : i32,
    }
    impl Content {
        pub fn new_with(s:String, b:bool, i:i32) -&gt; Content {
            return Content{s,b,i};
        }
    }

        pub fn main(){
            let l : List&lt;i32&gt; = List::new();
            println!("{:?}",l);
            assert_eq!(l.get(0),None);
            assert_eq!(l.get(1),None);
            let l = List{ head: Some(Box::new(Node{ elem: 4, next: None })), len: 1 };
            assert_eq!(l.get(0),Some(&amp;4));
            let s : String = format!("{:?}",l);
            assert_eq!(s.contains("Vec"),false);
        }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">List { head: None, len: 0 }</pre></td>
</tr>
<tr class="r0">
<td class="cell c0" style=""><pre class="tablecell">#[derive(Debug)]
    pub struct List&lt;T&gt; {
        head: Link&lt;T&gt;,
        len: i32,
    }
    type Link&lt;T&gt; = Option&lt;Box&lt;Node&lt;T&gt;&gt;&gt;;
    #[derive(Debug)]
    struct Node&lt;T&gt; {
        elem: T,
        next: Link&lt;T&gt;,
    }

#[derive(Debug)]
    pub struct Content {
        s : String, b : bool, i : i32,
    }
    impl Content {
        pub fn new_with(s:String, b:bool, i:i32) -&gt; Content {
            return Content{s,b,i};
        }
    }

       pub fn main(){
            let elem0 = Content::new_with("test".to_string(),true,2);
            let elem1 = Content::new_with("what".to_string(),true,2);
            let elem2 = Content::new_with("this".to_string(),false,8);
            let elem3 = Content::new_with("dope".to_string(),true,18);
            let mut l : List&lt;Content&gt; = List::new();
            println!("{:?}",l.add(elem0,1));
            println!("{:?}",l);
            l.add(elem1,0);
            println!("{:?}",l);
            l.add(elem2,1);
            println!("{:?}",l);
            l.add(elem3,2);
            println!("{:?}",l);
            let elem4 = Content::new_with("nope".to_string(),false,1);
            l.add(elem4,4);
            println!("{:?}",l);
            let s : String = format!("{:?}",l);
            assert_eq!(s.contains("Vec"),false);
        }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">Err("wrong position")
List { head: None, len: 0 }
List { head: Some(Node { elem: Content { s: "what", b: true, i: 2 }, next: None }), len: 1 }
List { head: Some(Node { elem: Content { s: "what", b: true, i: 2 }, next: Some(Node { elem: Content { s: "this", b: false, i: 8 }, next: None }) }), len: 2 }
List { head: Some(Node { elem: Content { s: "what", b: true, i: 2 }, next: Some(Node { elem: Content { s: "this", b: false, i: 8 }, next: Some(Node { elem: Content { s: "dope", b: true, i: 18 }, next: None }) }) }), len: 3 }
List { head: Some(Node { elem: Content { s: "what", b: true, i: 2 }, next: Some(Node { elem: Content { s: "this", b: false, i: 8 }, next: Some(Node { elem: Content { s: "dope", b: true, i: 18 }, next: None }) }) }), len: 3 }</pre></td>
</tr>
<tr class="r1">
<td class="cell c0" style=""><pre class="tablecell">#[derive(Debug)]
    pub struct List&lt;T&gt; {
        head: Link&lt;T&gt;,
        len: i32,
    }
    type Link&lt;T&gt; = Option&lt;Box&lt;Node&lt;T&gt;&gt;&gt;;
    #[derive(Debug)]
    struct Node&lt;T&gt; {
        elem: T,
        next: Link&lt;T&gt;,
    }

#[derive(Debug)]
    pub struct Content {
        s : String, b : bool, i : i32,
    }
    impl Content {
        pub fn new_with(s:String, b:bool, i:i32) -&gt; Content {
            return Content{s,b,i};
        }
    }

       pub fn main(){
            let elem0 = Content::new_with("test".to_string(),true,2);
            let elem1 = Content::new_with("what".to_string(),true,2);
            let elem2 = Content::new_with("this".to_string(),false,8);
            let elem3 = Content::new_with("dope".to_string(),true,18);
            let mut l : List&lt;Content&gt; = List::new();
            l.prepend(elem0);
            println!("{:?}",l);
            l.prepend(elem1);
            println!("{:?}",l);
            l.prepend(elem2);
            println!("{:?}",l);
            let s : String = format!("{:?}",l);
            assert_eq!(s.contains("Vec"),false);
        }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">List { head: Some(Node { elem: Content { s: "test", b: true, i: 2 }, next: None }), len: 1 }
List { head: Some(Node { elem: Content { s: "what", b: true, i: 2 }, next: Some(Node { elem: Content { s: "test", b: true, i: 2 }, next: None }) }), len: 2 }
List { head: Some(Node { elem: Content { s: "this", b: false, i: 8 }, next: Some(Node { elem: Content { s: "what", b: true, i: 2 }, next: Some(Node { elem: Content { s: "test", b: true, i: 2 }, next: None }) }) }), len: 3 }</pre></td>
</tr>
<tr class="r0 lastrow">
<td class="cell c0" style=""><pre class="tablecell">#[derive(Debug)]
    pub struct List&lt;T&gt; {
        head: Link&lt;T&gt;,
        len: i32,
    }
    type Link&lt;T&gt; = Option&lt;Box&lt;Node&lt;T&gt;&gt;&gt;;
    #[derive(Debug)]
    struct Node&lt;T&gt; {
        elem: T,
        next: Link&lt;T&gt;,
    }

#[derive(Debug)]
    pub struct Content {
        s : String, b : bool, i : i32,
    }
    impl Content {
        pub fn new_with(s:String, b:bool, i:i32) -&gt; Content {
            return Content{s,b,i};
        }
    }

        pub fn main(){
            let elem0 = Content::new_with("test".to_string(),true,2);
            let elem1 = Content::new_with("what".to_string(),true,2);
            let elem2 = Content::new_with("this".to_string(),false,8);
            let elem3 = Content::new_with("dope".to_string(),true,18);
            let mut l : List&lt;Content&gt; = List::new();
            l.append(elem0);
            println!("{:?}",l);
            l.append(elem1);
            println!("{:?}",l);
            l.append(elem2);
            println!("{:?}",l);
            let s : String = format!("{:?}",l);
            assert_eq!(s.contains("Vec"),false);
        }
  </pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">List { head: Some(Node { elem: Content { s: "test", b: true, i: 2 }, next: None }), len: 1 }
List { head: Some(Node { elem: Content { s: "test", b: true, i: 2 }, next: Some(Node { elem: Content { s: "what", b: true, i: 2 }, next: None }) }), len: 2 }
List { head: Some(Node { elem: Content { s: "test", b: true, i: 2 }, next: Some(Node { elem: Content { s: "what", b: true, i: 2 }, next: Some(Node { elem: Content { s: "this", b: false, i: 8 }, next: None }) }) }), len: 3 }</pre></td>
</tr>
</tbody>
</table>

# Ex 5
SameBool is a Trait
It has a method `samebool` that takes a SameBool and it returns a bool

Content is a struct with an i32 and a bool
Two Contents can be compared (<, >, ==) by comparing their i32 field ([2 points])
Content implements SameBool: the method of the trait returns
    whether `self` has the same bool as the parameter ([1] point)

Define a Graph as a vector of Nodes whose elements are arbitrary T
    - add a function for creating an empty graph ([1] points)
When T implements `SameBool` and `PartialOrd`,
    define function `add_node` that adds a Node to the graph with these connections:
    - the added node gets as neighbour all nodes in the graph that are < than it
    - the added node becomes a neighbour of all the nodes with the `samebool`
    ([6] points)

Note: the tests already include the code below, all you need to paste as the answer are the `impl` blocks
and possible imports (use ... ).

```rust
type NodeRef<T> = Rc<RefCell<Node<T>>>;
struct Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}
impl<T:Debug> Debug for Node<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
    }
}
struct Graph<T> {
    nodes: Vec<NodeRef<T>>,
}
pub trait SameBool{
    fn samebool(&self, other:&Self)->bool;
}
#[derive(Debug)]
pub struct Content{
    pub i:i32,
    pub b:bool
}
impl Content {
    pub fn new_with(i: i32, b: bool) -> Content {
        Content { i, b }
    }
}
```

<table class="coderunnerexamples" id="yui_3_17_2_1_1674583154699_103">
<thead>
<tr>
<th class="header c0" style="" scope="col">Test</th>
<th class="header c1 lastcol" style="" scope="col">Result</th>
</tr>
</thead>
<tbody id="yui_3_17_2_1_1674583154699_102"><tr class="r0" id="yui_3_17_2_1_1674583154699_101">
<td class="cell c0" style="" id="yui_3_17_2_1_1674583154699_100"><pre class="tablecell" id="yui_3_17_2_1_1674583154699_99">    pub trait SameBool{
        fn samebool(&amp;self, other:&amp;Self)-&gt;bool;
    }
    #[derive(Debug)]
    pub struct Content{
        pub i:i32,
        pub b:bool
    }
    impl Content {
        pub fn new_with(i: i32, b: bool) -&gt; Content {
            Content { i, b }
        }
    }
    type NodeRef&lt;T&gt; = Rc&lt;RefCell&lt;Node&lt;T&gt;&gt;&gt;;
    struct Node&lt;T&gt; {
        inner_value: T,
        adjacent: Vec&lt;NodeRef&lt;T&gt;&gt;,
    }
    impl&lt;T:Debug&gt; Debug for Node&lt;T&gt;{
        fn fmt(&amp;self, f: &amp;mut Formatter&lt;'_&gt;) -&gt; std::fmt::Result {
            write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
        }
    }
    #[derive(Debug)]
    struct Graph&lt;T&gt; {
        nodes: Vec&lt;NodeRef&lt;T&gt;&gt;,
    }


pub fn main(){
            let mut el1 = Content{i:10, b:true};
            let mut el2 = Content{i:11, b:true};
            let mut el3 = Content{i:11, b:false};
            assert_eq!(el1&lt;el2,true);
            assert_eq!(el2&lt;el1,false);
            assert_eq!(el2==el3,true);
        }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell"></pre></td>
</tr>
<tr class="r1">
<td class="cell c0" style=""><pre class="tablecell">    pub trait SameBool{
        fn samebool(&amp;self, other:&amp;Self)-&gt;bool;
    }
    #[derive(Debug)]
    pub struct Content{
        pub i:i32,
        pub b:bool
    }
    impl Content {
        pub fn new_with(i: i32, b: bool) -&gt; Content {
            Content { i, b }
        }
    }
    type NodeRef&lt;T&gt; = Rc&lt;RefCell&lt;Node&lt;T&gt;&gt;&gt;;
    struct Node&lt;T&gt; {
        inner_value: T,
        adjacent: Vec&lt;NodeRef&lt;T&gt;&gt;,
    }
    impl&lt;T:Debug&gt; Debug for Node&lt;T&gt;{
        fn fmt(&amp;self, f: &amp;mut Formatter&lt;'_&gt;) -&gt; std::fmt::Result {
            write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
        }
    }
    #[derive(Debug)]
    struct Graph&lt;T&gt; {
        nodes: Vec&lt;NodeRef&lt;T&gt;&gt;,
    }

pub fn main(){
            let mut el1 = Content{i:10, b:true};
            let mut el2 = Content{i:11, b:true};
            let mut el3 = Content{i:11, b:false};
            assert_eq!(el1.samebool(&amp;el2), true);
            assert_eq!(el1.samebool(&amp;el3), false);
        }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell"></pre></td>
</tr>
<tr class="r0">
<td class="cell c0" style=""><pre class="tablecell">    pub trait SameBool{
        fn samebool(&amp;self, other:&amp;Self)-&gt;bool;
    }
    #[derive(Debug)]
    pub struct Content{
        pub i:i32,
        pub b:bool
    }
    impl Content {
        pub fn new_with(i: i32, b: bool) -&gt; Content {
            Content { i, b }
        }
    }
    type NodeRef&lt;T&gt; = Rc&lt;RefCell&lt;Node&lt;T&gt;&gt;&gt;;
    struct Node&lt;T&gt; {
        inner_value: T,
        adjacent: Vec&lt;NodeRef&lt;T&gt;&gt;,
    }
    impl&lt;T:Debug&gt; Debug for Node&lt;T&gt;{
        fn fmt(&amp;self, f: &amp;mut Formatter&lt;'_&gt;) -&gt; std::fmt::Result {
            write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
        }
    }
    #[derive(Debug)]
    struct Graph&lt;T&gt; {
        nodes: Vec&lt;NodeRef&lt;T&gt;&gt;,
    }

pub fn main() {
            let mut g : Graph&lt;Content&gt; = Graph::new();
            println!("{:?}",g);
        }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">Graph { nodes: [] }</pre></td>
</tr>
<tr class="r1 lastrow">
<td class="cell c0" style=""><pre class="tablecell">    pub trait SameBool{
        fn samebool(&amp;self, other:&amp;Self)-&gt;bool;
    }
    #[derive(Debug)]
    pub struct Content{
        pub i:i32,
        pub b:bool
    }
    impl Content {
        pub fn new_with(i: i32, b: bool) -&gt; Content {
            Content { i, b }
        }
    }
    type NodeRef&lt;T&gt; = Rc&lt;RefCell&lt;Node&lt;T&gt;&gt;&gt;;
    struct Node&lt;T&gt; {
        inner_value: T,
        adjacent: Vec&lt;NodeRef&lt;T&gt;&gt;,
    }

    impl&lt;T:Debug&gt; Debug for Node&lt;T&gt;{
        fn fmt(&amp;self, f: &amp;mut Formatter&lt;'_&gt;) -&gt; std::fmt::Result {
            write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
        }
    }
    #[derive(Debug)]
    struct Graph&lt;T&gt; {
        nodes: Vec&lt;NodeRef&lt;T&gt;&gt;,
    }

pub fn main(){
        let mut el1 = Content{i:10, b:true};
        let mut el2 = Content{i:11, b:true};
        let mut el3 = Content{i:12, b:false};
        let mut g = Graph::new();
        println!("{:?}",g);
        g.add_node(el1);
        println!("{:?}",g);
        g.add_node(el2);
        println!("{:?}",g);
        g.add_node(el3);
        println!("{:?}",g);

        let mut el1 = Content{i:10, b:true};
        let mut el2 = Content{i:8, b:false};
        let mut g = Graph::new();
        println!("{:?}",g);
        g.add_node(el1);
        println!("{:?}",g);
        g.add_node(el2);
        println!("{:?}",g);

        let mut el1 = Content{i:10, b:true};
        let mut el2 = Content{i:11, b:true};
        let mut el3 = Content{i:12, b:true};
        let mut g = Graph::new();
        println!("{:?}",g);
        g.add_node(el1);
        println!("{:?}",g);
        g.add_node(el2);
        println!("{:?}",g);
        g.add_node(el3);
        println!("{:?}",g);

        let mut el1 = Content{i:10, b:true};
        let mut el2 = Content{i:9, b:false};
        let mut el3 = Content{i:8, b:true};
        let mut g = Graph::new();
        println!("{:?}",g);
        g.add_node(el1);
        println!("{:?}",g);
        g.add_node(el2);
        println!("{:?}",g);
        g.add_node(el3);
        println!("{:?}",g);
    }</pre></td>
<td class="cell c1 lastcol" style=""><pre class="tablecell">Graph { nodes: [] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 0 }] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 1 }, RefCell { value: iv: Content { i: 11, b: true }, adj: 1 }] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 1 }, RefCell { value: iv: Content { i: 11, b: true }, adj: 1 }, RefCell { value: iv: Content { i: 12, b: false }, adj: 2 }] }
Graph { nodes: [] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 0 }] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 0 }, RefCell { value: iv: Content { i: 8, b: false }, adj: 0 }] }
Graph { nodes: [] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 0 }] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 1 }, RefCell { value: iv: Content { i: 11, b: true }, adj: 1 }] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 2 }, RefCell { value: iv: Content { i: 11, b: true }, adj: 2 }, RefCell { value: iv: Content { i: 12, b: true }, adj: 2 }] }
Graph { nodes: [] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 0 }] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 0 }, RefCell { value: iv: Content { i: 9, b: false }, adj: 0 }] }
Graph { nodes: [RefCell { value: iv: Content { i: 10, b: true }, adj: 1 }, RefCell { value: iv: Content { i: 9, b: false }, adj: 0 }, RefCell { value: iv: Content { i: 8, b: true }, adj: 0 }] }
</pre></td>
</tr>
</tbody>
</table>