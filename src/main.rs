fn main() {
    move_problem()
}

fn variable_scope() {
    // salutation is not declared : out of scope

    // This is declaring a string literal type
    // They are pushed/popped from the stack
    // They are immutable, hardcoded values
    let salutation: &str = "Hello"; // salutation is declared : enters the scope

    // salutation is in scope, available for computation
}
// variable out of scope,

fn string_type() {
    // This declares a String type
    // Allocated on the heap
    // Able to store amount of text unknown at runtime
    let stringy = String::from("Hello");
    // It can be mutable
    let mut stringy_mut = String::from("Heya");
    stringy_mut.push_str(", boy");

    // String::from allocates the memory needed
    // When stringy or stringy_mut get out of scope, that memory is returned
    // It is returned by the drop function called automatically
}

fn move_problem() {
    // Simple types are put on the stack because they have a known size at compile time
    // They have the Copy trait (special annotation)
    // Only possible if they don't implement the Drop trait
    let x = 5;
    let y = x;
    // Types that are copy :
    // - integers (u32, u64, i32, etc)
    // - booleans
    // - float (f64, f32)
    // - char
    // - tuples of only Copy types

    // A string has :
    // - stored on the stack :
    //   -> length : memory in bytes used by contents
    //   -> capacity : total amount of memory allocated
    //   -> pointer to the memory storing the contents
    // - stored on the heap :
    //   -> the contents of the string (index/value pairs for each char)


    // How about that ?
    let s1 = String::from("hello");
    let s2 = s1;

    // To protect against double free bug
    // (freeing the same memory when going out of scope)
    // (because s1 and s2's pointers both refer to the same contents on the heap)
    // Rust considers s1 invalid when you perform this kind of shallow copy

    // This doesn't compile
    // println!("{}", s1);

    // let s2 = s1; is called a move. s1 was moved into s2.
    println!("{}", s2);

    // Rust never creates "deep" copies of your data
    // "automatic" copies are always cheap in perf
}

fn clone_wars() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // The heap data gets copied as well as length, capacity and pointer
    // It may be expensive, and signals unusual process
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn fn_ownership() {
    // s comes into scope
    let s = String::from("hello");

    // s's value moves into the function...
    takes_ownership(s);
    // ... and so is no longer valid here
    // println!("I am invalid : {}", s);

    // x comes into scope
    let x = 5;

    // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward
    makes_copy(x);
    // x still in scope
    println!("I am still here : {}", x)
}
// Here, x goes out of scope, then s.
// But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
}
// Here, some_string goes out of scope
// drop is called because it is not Copy.
// The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}
// Here, some_integer goes out of scope.
// Nothing special happens because it is Copy

fn return_values_ownership() {
    // gives_ownership_string moves its return
    // value into s1
    let s1 = gives_ownership_string();

    // s2 comes into scope
    let s2 = String::from("hello");

    // s2 is moved into takes_and_gives_back
    // and it ALSO moves its return value into s3

    let s3 = takes_and_gives_back(s2);
}
// Here, s3 goes out of scope and is dropped.
// s2 goes out of scope but was moved, so nothing happens
// s1 goes out of scope and is dropped.

fn gives_ownership_string() -> String {
    // gives_ownership_string will move its return value
    // into the function that calls it

    // some_string comes into scope
    let some_string = String::from("hello");

    // some_string is returned and moves out to the calling function
    some_string
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes intoscope
    a_string
    // a_string is returned and moves out to the calling function
}
