// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.


/* Cow 类型的克隆操作是惰性的，只有在修改数据时才会进行克隆操作。 */
use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned
            let input_mut = input.to_mut();     /* 返回的是自身可变引用 */
            input_mut[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Borrowed(_)));
    }

    #[test]
    fn owned_no_mutation() {
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn owned_mutation() {
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }
}
