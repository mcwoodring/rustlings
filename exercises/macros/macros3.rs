// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

/*
    The 'use macros::*' directive below works, but I'm not sure whether it is the intended solution.

    Per the "hint" for macros3.rs:
        In order to use a macro outside of its module, you need to do something
        special to the module to lift the macro out into its parent.

        The same trick also works on "extern crate" statements for crates that have
        exported macros, if you've seen any of those around.
*/
use macros::*;

fn main() {
    my_macro!();
}
