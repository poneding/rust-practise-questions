/// # Chapter 3 - Structs
///
/// Write a program to add, subtract and multiply two complex numbers using structures to function.
fn main() {
    let n1 = Complex {
        real: 1.0,
        imaginary: 2.0,
    };
    let n2 = Complex {
        real: 3.0,
        imaginary: 4.0,
    };

    assert_eq!(
        add_complex_numbers(n1, n2),
        Complex {
            real: 4.0,
            imaginary: 6.0
        }
    );

    assert_eq!(
        subtract_complex_numbers(n1, n2),
        Complex {
            real: -2.0,
            imaginary: -2.0
        }
    );

    assert_eq!(
        multiply_complex_numbers(n1, n2),
        Complex {
            real: -5.0,
            imaginary: 10.0
        }
    );
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

fn add_complex_numbers(n1: Complex, n2: Complex) -> Complex {
    Complex {
        real: n1.real + n2.real,
        imaginary: n1.imaginary + n2.imaginary,
    }
}

fn subtract_complex_numbers(n1: Complex, n2: Complex) -> Complex {
    Complex {
        real: n1.real - n2.real,
        imaginary: n1.imaginary - n2.imaginary,
    }
}

fn multiply_complex_numbers(n1: Complex, n2: Complex) -> Complex {
    Complex {
        real: n1.real * n2.real - n1.imaginary * n2.imaginary,
        imaginary: n1.real * n2.imaginary + n1.imaginary * n2.real,
    }
}
