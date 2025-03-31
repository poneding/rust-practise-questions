/// # Chapter 6 - Operator Overloading & Utility Traits
///
/// Create the struct named `World` that contains the previous named struct `Hero`, define `drop`
/// for it so that it prints "The world ends here !!!". Observe the order in which `World` and `Hero`
/// contained by it are dropped.
fn main() {
    let _w = World {
        _hero: Hero {
            name: "Superman".to_string(),
        },
    };

    // 结果：先打印 "The world ends here !!!" 再打印 "Oh no !!! Our hero Superman is defeated"
}

struct Hero {
    name: String,
}

impl Drop for Hero {
    fn drop(&mut self) {
        println!("Oh no !!! Our hero {} is defeated", self.name);
    }
}

struct World {
    _hero: Hero,
}

impl Drop for World {
    fn drop(&mut self) {
        println!("The world ends here !!!");
    }
}
