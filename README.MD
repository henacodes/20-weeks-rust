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
