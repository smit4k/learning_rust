fn main() {
    let mut s = String::from("hello"); // Can be mutated
    s.push_str(", world");

    println!("{}", s);

    let x = 5; // Binding value 5 to x
    let y = x; // Make a copy of x and bind it to y

    // String representation
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 goes out of scope

    println!("{}, world!", s2);
    // This actually works differently than the integer binding, instead of making a copy, s2
    // points to the same memory address that s1 is pointing to. This is efficient because rust
    // does not copy the entire memory heap
    //
    // Scope and assignment
    //
    let mut d = String::from("hello");
    d = String::from("ahoy");

    println!("{d}, world!");

    // We assign hello to d, and then after that, we assign ahoy to d. Now that nothing is pointing
    // to the original value in the heap (hello), that string goes out of scope and the memory is
    // freed. When we run, the printed statement will be "ahoy"
    //

    let g = 5;
    let j = g;

    println!("g = {g}, j = {j}");

    // g continues to be valid and does not need to be dropped, because integers have a known size
    // and are stored on the stack, so at compile time, so it wouldn't be reasonable to invalidate them,
    //
    //

    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }
    // Here, some_string goes out of scope and is dropped. The backing memory is freed

    fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
    }
    // Here, some_integer goes out of scope, nothing special happens

    let l = String::from("hallo"); // l comes into scope

    takes_ownership(l); // s moves into the function and is no longer valid here

    let z = 5; // z comes into scope

    makes_copy(z); // Because i32 inherits the copy trait, x does not move into the
    // function and so it is okay to use it afterwards.
    //

    fn calculate_length(s: &String) -> usize {
        s.len()
    } // Here, s goes out of scope. But because s does not have ownership of what it refers to, the
    // String is not dropped. This is called borrowing. s does not own the string, m does, so after
    // s is finished borrwing the string, it has to give it back.

    let m = String::from("hello");

    let len = calculate_length(&m);

    println!("The length of {m} is {len}");

    // But what happens when we try to change the value we are borrowing?

    // fn change(some_string: &String) {
    //  some_string.push_str(", world!");
    // } some_string is a & reference so the data it refers to cannot be borrowed as a mutable.
    //
    // Just as variables are immutable by default, so are references. We are not allowed to modify
    // something we "borrow" or have a refernce to.
    // We can fix this by adding &mut to the reference
    //

    fn change(some_string: &mut String) {
        some_string.push_str(", world!");
    }

    let mut my_str = String::from("hello");

    change(&mut my_str);

    // Mutable referencees have a major restriction however, if you have a mutable reference to a
    // value, you cannot have any other references to that value.
    //

    let mut cool_str = String::from("hey");

    let k1 = &mut cool_str;
    let k2 = &mut cool_str;

    // println!("{k1} and {k2}");
}
