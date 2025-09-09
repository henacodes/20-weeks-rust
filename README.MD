# 20 Weeks of Rust

This is me documenting my experience learning Rust for 20 weeks with The Rust book. In this note I skipped the introductory parts that deal with basic programming concepts.

## Ownership

Traditionally there have been two ways in which memory has been managed. The first way is through garbage collector that regularly looks for no-longer-used memory while the 2nd way is where the programmer has to explicitly allocate/diallocate memory. Rust introduces a third mechanism in which memory is managed through ownership with a set of rules that the compiler checks.

### Stack Vs Heap

Computers can possibly store data on stack or heap. "The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out." Meanwhile the heap is a less organized structure where "when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. ... when you want the actual data, you must follow the pointer."

The ownership mechanism then is a robust system that "keeps track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap"

### Ownership Rules

- Each value on the system has an owner
- There can only be one owner at a time
- When the owner goes out of scope,the value will be dropped

### How does ownership work?

Before we see how ownership works, we need to differentiate between the two categories of data types in Rust. Data types of known size (integers, booleans, string literals and tuples with elements of knowns size ) and of unknown size ( String, Vectors ... ). The former ones are stored on the stack while the latter are stored on the heap.

```rust
let s1 = String::from("hello"); // stored on the heap
let s2 = "world"; // stored on the stack
```

Once again, the difference between the two strings is that s1 is a string of unknown size and s2 is of known size, ie string of length 5. The advantage of string literals is that they are easily accessible. But the downside is that they are immutable while Strings are mutable like the following

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{s}"); // This will print `hello, world!` hence mutated
```

With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

- The memory must be requested from the memory allocator at runtime.
- We need a way of freeing the memory when we’re done with our String

The first part is done when we call String::from(). But the second part is where Rust takes a different approach unlike the traditional languages. Instead of manually deallocating the memory like C or using Garbage collector like C#, in Rust the memory is automatically freed once the variable that owns it goes out of scope.

```rust
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
} // s no longer available
```

When a variable goes out of scope, Rust calls a special function for us. This function is called drop and its called automatically at the closing curly bracket.

#### About time baby

Lets finally see ownership in action

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{s1}");
```

This code is very much self-explanatory. But it actually won't compile. When we call String::from() the owner of the value "hello" is s1. But when we bind s1 to s2 the ownership of the value moves from s1 to s2 and thus s2 becomes the owner ( remember the three ownership rules ?). But when we try to print s1, we are referring to a variable thats not valid anymore and hence the compilor error.

#### Ownership and Functions

When we pass variables as arguments to a functions, the fuction actually takes the ownership of the value. So further reference to the variable will result in compile time error. Take a look at this example

```rust
let s = String::from("hello"); // s1 is valid
takes_ownership(s1); // s1 is not owner anymore from this point on
                    // s1 is invalid

```

One quick way to gain back the ownership is to return the value from the function, which will transfer the ownership to the variable thats binding.

```rust
let s2 = takes_and_returns_ownership(s1);

fn takes_and_returns_ownership(a_string: String) -> String { // a_string comes into  scope
    a_string  // a_string is returned and moves out to the calling function
}
```

## References and Borrowing

Having to return the value from the function just to regain back the ownership kinda sucks. Instead we can use references so that the function won't take the ownership.

```rust

let s1 = String::from("hello");
let len = calculate_length(&s1);

// ....

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

By accepting &String instead of String, we make sure we are only getting the reference without taking ownership of it.
We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.

## Mutable References

Not only can a function borrow a value, it can also mutate the borrowed data. To make our references mutable, we add `mut` right after `&`. Take a look

```rust
let mut s = String::from("Hello");
mutate(&mut s);

fn mutate (myStr:&mut String) {
    myStr.push_str("world");
}
```

Note that if we are going to pass a value as a mutable reference, we need to update its declaration as mutable as well.

### Caveats in mutable references

References live as long as the last time they are used. If we use a reference just under its initialization and only at that place, then it will only last until the next line. Having said that, lets see a very tricky case in which having multiple mutable references to the same value causes a problem.

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2);
```

As we have said, mutable references live as long as their last valid usage. So in our case the live until `println!()`. But the problem is by that point, we have two references to the same value both of them intending to mutate it, which is not allowed in Rust. So the only way to make the code work is moving the println!() before `r2`, so that by the time `r2` is initialized `r1` wouldn't be there.

### Mutable and Immutable references

Another restriction regarding references is that we can't have mutable and immutable references at the same time. Because that would mean one reference expects for the value not to change while the other intended to change.

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = &mut s;
println!("{}, {}, and {}", r1, r2, r3); // using them here implies all three references live until this line
```

We can fix the above code by printing the immutable references before the mutable reference is introduced. Since its before the introduction of the mutable reference that the immutable references were used, it will cause no error.

## Dangling References

Dangling Reference is when there is a reference to a data that doesn't exist anymore. Its easy to make those mistakes in languages with pointers. But Rust makes sure we don't cause such errors.

```rust
fn dangle () -> &String {
    let s = String::from("Hello");
    &s
}
```

Rust will not let us compile this code as we are turning to a reference to a string to the upper calling function eventhough the underlying value will not live longer than the function's scope. The only way out is just returning the string itself, so that the binding variable takes ownership of it.

## Structs

Structs are a way of organizing related data. Just like tuples, they can have different types of data, except that they have a identifier [ name ] for each member. We define structs like so

```rust
struct User {
    username:String,
    email:String,
    age:u8,
    is_active:bool,
}
```

We can create a new instance of the struct and access its properties like this

```rust
let user1 = User {
    username:String::from("henacodes"),
    email:String::from("test@gmail.com"),
    age:20,
    is_active:true,
}

// access its properties
println!("My email is: {}", user1.email);
```

To mutate specific properties afterward, we need to make the entire instance mutable.

### Struct Update

If we want to create another instance, but with some similar fields as a preexisting instance, we can use the following mechanism

```rust

// move the values of the fields in user1 instance except for age field
let user2 = User {
    age:30,
    ..user1
}
```

But note that the properties of user1 we just moved to user2 are of unknown size we discussed earlier. So those fields don't actually exist in user1 anymore.

### Tuple Structs

There is also another type of struct that kinda acts like tuples. They don’t have names associated with their fields; rather, they just have the types of the fields. They are usefull when we want to have a named tuple

```rust
struct Rgb (i32, i32, i32 )

let color1 = Rgb(23,255,195);
```

### Methods

Not only fields, we call also associate functions to structs. The way to that is like this

```rust

struct Rectangle {
    width:u32,
    height:u32
}

// we need a method to calculate the area

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }
}


let rect1 = Rectangle {
       width: 30,
       height: 50,
   };

println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
);
```

The first parameter of a method is always &self. self in impl block always refers to the struct in questions. By taking &self as the first parameter, we are essentially taking a reference to the struct instance.

### Associated Functions

Associated functions are almoost similar to methods. The only difference is that methods can only be called on an actual instance while associated functions can be called without an instanced, directly from the struct itself ( they work almost like static methods in OOP languages ). What that means is that unlike methods, they don't take &self as the first parameter.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

And we can call associated functions like this

```rust
Rectangle::square(2);
```

String::from() is a good example of associated functions that we have been using so far.

## Enums

Enumeration (enum) is a way of defining possible values of a variable. For example a power_status variable may only have "on" or "off" values.

```rust
enum PowerStatus {
        Off,
        On,
    }
let current_power_status = PowerStatus::On;
```

If we were to accept a power status as a parameter in a functions

```rust

fn turn_on_tv ( power_status: PowerStatus ) {
    if (power_status == SwitchStatus::On) {
        println!("TV is turned on");
    }
}

turn_on_tv(current_power_status);
```

We can also attach data to any of the variants in our enums

```rust
enum Message {
    Quit,
    Write(String),
    Move { x:i32, y:i32 },
    ChangeColor  ( i32,i32,i32)
}
```

Now we can instantiate different messages of different kinds.

```rust
let quit_msg = Message::Quit;
let write_msg = Message::Write("Hello World");
let move_msg = Message::Move{ x:3, y:2 };
```

### Enum Methods

Just like structs, enums can also have methods

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
let m = Message::Write(String::from("hello"));
m.call();

```

### The Match Controll Flow

The match control flow allows us to compare value against a series of patterns and then executed a block of code based on the match ( kind of switch-case flow in Javascript). Lets see it in action. Lets say we have enum of different types of Ethiopian Coins (Santim) and using the match control flow we return the value of the coin

```rust

enum Santim {
    Chichifo,
    Simuni,
    Amsa,
}


fn value_of_cent(cent:Santim) -> u8 {
    match cent {
        Santim::Chichifo => 1,
        Santim::Simuni => 25,
        Santim::Amsa => 50
    }
}
```

Each matching case is called arm. Each arm has two parts; the pattern and some block of code to execute of that pattern matches.

#### Patterns with Binding Values

As we have already seen earlier, we can bind values to our enums. And we can actually access that data in our match block like so

```rust

enum Santim {
     Chichifo,
    Simuni,
    Amsa(String),
}


fn value_of_cent(cent:Santim) -> u8 {
    match cent {
        Santim::Chichifo => 1,
        Santim::Simuni => 25,
        Santim::Amsa(msg) =>{
         println!("{msg}"); // perform any operation using the data
         return 50
        }
    }
}
```

#### Catch-all patterns

One thing we forgot to mention so far is that Rust needs us to handle all possible cases of the match. But sometime we just want to handle some of the cases and leave other cases to a single default case, just like the default case in Javascript. We can do like this to handle that

```rust
match cent {
    Santim::Simuni => 25,
    other => Something_Else
}
```

The word other is just an arbitrary choice. We can put anything else to catch a binding value thats coming. And if we dont want to use that value we can just pass \_ in place of other, so that Rust won't bind to that value.

```rust
match cent {
    Santim::Simuni => 25,
    _ => ()
}
```

#### if-let and let-else

If we want to handle only one arm, we can do better than writing all that \_ => () boilerplate code just to pass the compiler police. How? using if-let. The above code can be rewriten as

```rust
if let Santim::Simuni = cent {
    25
}
```

## Common Collections

There are many data structures that are included in Rust's standard library that can contain multiple values, can have expandable/shrinkable memory size ( hence stored in heap ). Here we will discuss some of the most used ones.

### Vectors

Vectors allow us to store multiple data of same type next to each other. They are similar to arrays, except their size is not known at compile time.
We can define a new vector like so

```rust

let mut fruits:Vec<&str> = Vec::new();

```

If we want to add values to the vector. ( make sure we have a mutable vector )

```rust
fruits.push("mango");
fruits.push("bananas");
```

#### The vec! macro

Declaring new vector and pushing elements is kinda verbouse. so Rust has a macro that takes the list and infers their type

```rust
let fruits = vec!["mango", "bananas"];
```

#### Reading elements of vectors

There are essentially two ways to get an element of a vector. By indexing the element or by using the get() method.

```rust

let mango = &fruits[0];
let banans = fruits.get(1)

```

When we use the former method, we are storing referrence to the value being indexed. And if the value being indexed doesn't exist, our program panics. However, when we use the latter way, we get Option<T> type, where it could't be Some<T> or None, that way our program continues to run even if the index for the value is out of bounds of the vector.

#### Iteration

If we wanna iterate throught the elements of a vector

```rust

for fruit in &fruits {
    println!("{fruit}");
}
```

#### Multiple Types with Enums

As we have mentioned earlier, vectors can only store values of simila type. When we are in situation where we need to store values of multiple types, we can use Enums.
For example

```rust

enum DiffType {
    MyInt(i32),
    MyStr(String)
}

let v = vec![DiffType::MyInt(45), DiffType::MyStr("hello")]

```

### Strings

Strings are types with growable size and owner. We can create Strings in multiple ways

```rust

let str1 = String::new(); // initialize new empty string
let str2 = String::from("hello"); // initialize from string literal
let str3 = "hiii".to_string(); // convert to String from string literal
```

## Generics

Generic data types are abstract types that are used to define function signature/struct/enum.

```rust
// it doesn't compile
fn std_out<T> (output:&T) {
    println!("{output}")
}
```

So what this function does ideally is that it takes an argument of type unknown and prints it to the console. If we wanted to print a value to the console, we dont really need to know whether its a string or an integer or a float. we just need to print it. So we use Generic type `T` to represent the type of the value being used throughout the function's signature. One caveat here is that for Rust to print a value to the console, it needs to know whether or not the value is "printable", in that it implements the trait Display. Thats why the above code doesn't compile.

One thing we need to make sure is that we need to use ( and apply values to ) generics consistently. For example

```rust

struct Point<T> {
    x:T,
    y:T
}

let p1 = Point { x: 5, y: 4.0 };

```

The above doesn't compile and the reason is because we didnt apply values of similar type to fields with similar generic type T, i.e whatever the type of the value given to `x`, `y` must also have the same type of value.

Here are a couple of examples using generic types in different ways

```rust

// in enums
enum Result<T, E> {
    Ok(T),
    Err(E),
}


// in method definitions
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

```

One thing to note is that methods in struct implementation can use their own generic types.

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c' };

let p3 = p1.mixup(p2);
```

While the Point struct has generic types X1 and Y1, its method mixup maintains its own generic types X2 and Y2. In result p3 will look like `{ x:5, y:'c'  }`

## Traits

Earlier in Generics section we defined a function that takes a generic type T and prints it to the console. We also mentioned that it wont compile because the compilor doesnt know whether or not the generic type can be printed to the console. Well thats basically what traits do. They tell the compilor what specific functionalities a generic type has. In other words they specify the behaviors a generic type has and shares with other types. For example, integer type has a functionality of being able to be added/subtracted, printed to the console, multiplied and etc. In short terms, a trait is a group of methods we can call for different types.

Lets say we have a data of NewsArticles and Tweets and that we need to summarize both of them, meaning we need to have a trait called Summary that is found both in NewsArticle type and Tweet type.

Lets first define our trait.

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

Now lets implement the trait for each type

```rust
struct NewsArticle {
   pub headline: String,
   pub location: String,
   pub author: String,
   pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

```

There is one sharable trait that has a method called `summarize` that returns a string of summary. The implementation detail is delegated to each struct, so that each of them configure the summary in its own way. From then on, we can call the trait in any instantiated `NewsArticle` or `Tweet`.

### Traits as Parameters

We can have a functions that take a generic parameter that implements specific traits.

```rust
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

The function `notify` will accept any argument as long as that argument implements the Summary trait.

### The Trait Bound Syntax

The above syntax is a short hand notation of a long form notation which goes like

```rust

fn notify <T:Summary> (item:&T) {
        println!("Breaking news! {}", item.summarize());
}
```

So in this case, we are explicitly saying that the generic type `T` can only be a type that implements the Summary trait. The advantage of using this syntax over the former one is when we want to enforce similar types for two or more parameters.

```rust
fn notify <T:Summary> (item1: &T, item2: &T) {}
```

This way we can enforce that both item1 and item2 are of the same generic type `T`. But if we used the shorthand syntax, we wouldn't be able to do so.

#### Multiple Trains with +

If we wanted to add more traits to the function, we can just use the `+` operator.

```rust
fn notify(item: &(impl Summary + Display)) {}

// or
pub fn notify<T: Summary + Display>(item: &T) {}
```

#### Where clause

When out trait bounds get too many, our code starts getting messy and unredable. Thankfully we can use the where clause to delineate trait bounds from the generic definitions.

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

The above function signature can be rewritten as

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
```

#### Return Types that Implement a trait

To specify the return type of a function as a trait

```rust
fn returns_summarizable() -> impl Summary {
    // can return a tweet or a news article
}
```

### Trait bounds to conditionally implement methods

We can have specific methods that only work for structs with a generic type that has specific traits. For example

```rust

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

In this example, the method `cmp_display` can only be called in `Pair`s that have concrete type that implements `Display` and `PartialOrd` behaviors.

## Lifetimes

Lifetimes are basically generics, but for the lifetime of a reference instead of types. As we already know by now, every reference has a lifetime, which is the scope in which that referrence is considered valid. The main purpose of lifetime annotation is preventing dangling references.

```rust

let msg: &String;
{
    let newMessage = String::from("Hello World");
    msg = &newMessage // Error: newMessage doesnt live long enough
}
println!("{msg}");
```

Dangling references error occur when the reference lives longer than the value value being refered to ( ie the lifetime of `newMessage` is shorter than the lifetime of `msg` ). In our particular example the value `newMessage` goes out of scope the moment we try to use it outside the curly brackets. The borrow checker annotates the outer scope life time as say , 'a, and the lifetime of the inner scope as 'b. It then compares the lifetime of `newMessage` ('b) with the lifetime of `msg`('a) then sees that 'b is shorter than 'a. So the program fails.

### Generic Lifetimes

Before seeing how generic lifetimes work, lets first see a case where we need them. Lets say we want a function that takes a reference to two integers and returns a reference to the highest one.

```rust
let x = 7
{
    let y = 4
    let the_highest = highest(&x, &y)
}
```

Note that the two parameters have different lifetimes. `x` lives longer than `y`. Now lets define the function

```rust
// doesn't compile
fn highest ( left:&i32, right:&i32 ) -> &i32 {
    if (left > right) {
        left
    } else {
        right
    }
}
```

The reason the code doesnt compile is because of lifetimes. As we have noted above, the two arguments we passed to the function ( can ) have different lifetimes, and the function is returning a reference to either one of them. But the issue is that the compilor doesn't know the lifetime of the resulting reference. It could be `x`'s lifetime or `y`'s. So we somehow need to specify the lifetime of the value being returned. Thats where generic lifetimes come.

To annotate lifetimes we use `'something` syntax. We usually use alphabets instead of long words, like `'a`, `'b`, `'c` and etc. Lets annotate our function

```rust
fn highest <'a>(left:&'a i32, right:&'a i32) -> &'a i32 {
 if (left > right) {
        left
    } else {
        right
    }
}
```

So what this function does is it tells the compilor that for some unknown ( generic ) lifetime `'a`, it takes two reference parameters that live atleast as long as `'a` and returns a reference that also lives as long as `'a`. In other words, `'a` is a lifetime generic that will take a lifetime of the shortest from the two parameters.
In the earlier code we wrote

```rust
let x = 7
{
    let y = 4;
    let the_highest = highest(&x, &y)
}
```

Since the highest of the two is `x`, the function will return a reference to `x`. But interestingly, we can't use the reference returned from the function outside the inner scope ( the `y` scope ). Because as we have said earlier, the lifetime of the returned reference will be the shortest lifetime of the two references in the parameters, which in our case is the lifetime of `y`, ie the inner scope.

### Lifetime Elision

It should be known by now that we have been writing functions that return references without using lifetime annotations. The reason is that Rust tries to avoid the explicit definition of lifetime annotations by following three rules. And those are

- the compiler assigns a lifetime parameter to each parameter that’s a reference. ( one reference param? one lifetime param. two reference param? two lifetime param. )
- if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters. ( ie the return lifetime parameter gets a lifetime of that specific referece parameter )
- if the function is a method with &self or &mut self parameter, the return reference lifetime will be equal to the lifetime of `self`

For example if a function takes one parameter thats a reference, then the lifetime of the reference returned will be the lifetime of the function's parameter.

```rust
fn only_one_param ( x:&i32  ) -> &i32 {}
```

We dont need to explicitly state lifetime for the reference parameter and return reference. Rust will apply the 2nd rule and give the returned reference the lifetime of `x`.

Its only when those rules don't satisfy that the Rust compilor rejects our program requiring us to be explicity about the lifetime of returned reference.

## Random Tips

### Borrow pattern bindings

We can use the `ref` keyword in pattern matching to borrow bindngs and avoid moving ownership

```rust
struct Point {
    x: i32,
    y: i32,
}
let optional_point = Some(Point { x: 100, y: 200 });

match optional_point {
    Some( ref p) => println!("Coordinates are {},{}", p.x, p.y),
    _ => panic!("No match!"),
}
```
