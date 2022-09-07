// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0); // SOLUTION 1: change fill_vec(vec0) to fill_vec(vec0.clone()) | SOLUTION 2: fill_vec(&vec0) | SOLUTION 3: fill_vec(&mut vec0) w/o catching return value

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    let mut vec1 = vec0; // SOLUTION 3

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

/*
SOLUTION 1: No change to fill_vec; only change the call to fill_vec from fill_vec(vec0) to fill_vec(vec0.clone())
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
*/

/*
SOLUTION 2: Borrow caller's vector by changing parameter type from Vec<i32> to &Vec<i32>, then create separate vector that's initialized with a copy/clone of the caller's; returing *that* vector to the caller (and transferring ownership of it to caller).
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
 */

 // SOLUTION 3: Change parameter type to a mutable reference to the caller's collection, modify directly, don't return anything.
 fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}