use std::{cell::RefCell, ops::Sub};

/// # Chapter 6 - Operator Overloading & Utility Traits
///
/// Create a struct name `Set` that contains a `Vector` of chars and overload the `minus`
/// operator so that when 2 structs subtract it removes the chars of 2nd from 1st one.
fn main() {
    let set1 = Set::from(vec!['a', 'b', 'c', 'd']);
    let set2 = Set::from(vec!['b', 'd']);
    let result = set1 - set2;

    assert_eq!(result.chars.borrow().len(), 2);
    assert_eq!(result.chars.borrow()[0], 'a');
    assert_eq!(result.chars.borrow()[1], 'c');
}

struct Set {
    chars: RefCell<Vec<char>>,
}

impl Set {
    fn from(chars: Vec<char>) -> Self {
        Set {
            chars: RefCell::new(chars),
        }
    }
}

impl Sub for Set {
    type Output = Set;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut chars = self.chars.borrow_mut();
        let rhs_chars = rhs.chars.borrow();
        chars.retain(|c| !rhs_chars.contains(c));

        Set::from(chars.clone())
    }
}
