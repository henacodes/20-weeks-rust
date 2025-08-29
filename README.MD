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

This code is very much self-explanatory. But it actually won't compile. When we call String::from() the owner of the value "hello" is s1. But when we bind s1 to s2 the ownership of the value moves from s1 to s2 and thus s2 becomes the owner ( remember the three ownership rules ?). But when try to print s1, we are referring to a variable thats not valid anymore and hence the compilor error.

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

Having to return the value from the function just to regain back the ownership kind sucks. Instead we can use references so that the function won't take the ownership.

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

## Packages, Crates and Modules
