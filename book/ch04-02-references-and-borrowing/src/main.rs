fn main() {
    reference_demo();

    mutable_reference_demo();

    mutable_immutable_reference_demo();
}

fn reference_demo() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_reference_demo() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("New string is {}.", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn mutable_immutable_reference_demo() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
