// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me();
}

/*
fn call_me() {
    println!("call_me was called");
}
*/

fn call_me() -> () {
    println!("call_me function with explicitly void return type was called");
}

/*
fn call_me() -> ! {
    panic!("This version of call_me will never return.");
}
*/