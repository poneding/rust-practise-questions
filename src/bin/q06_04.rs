/// # Chapter 6 - Operator Overloading & Utility Traits
///
/// Create a struct named `Class` that contains the class size, section and grade. Overload
/// the `>`, `<`, `>=`, `<=`, `==` operators to compare class sizes of various Classes.
fn main() {
    let class1 = Class::new(30, "A".to_string(), "10th".to_string());
    let class2 = Class::new(25, "B".to_string(), "10th".to_string());
    let class3 = Class::new(30, "C".to_string(), "11th".to_string());

    assert!(class1 > class2);
    assert!(class1 >= class3);
    assert!(class2 < class1);
    assert!(class2 <= class3);
    assert!(class1 == class3);
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Class {
    size: u32,
    section: String,
    grade: String,
}

impl Class {
    fn new(size: u32, section: String, grade: String) -> Self {
        Class {
            size,
            section,
            grade,
        }
    }
}

impl std::cmp::PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl std::cmp::PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.size.cmp(&other.size))
    }
}
