// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


pub mod macros {
    #[macro_export]   //导出后全局可见
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}


fn main() {
    my_macro!();
}
