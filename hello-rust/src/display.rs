use std::fmt;

// Display Examples using std::fmt

struct Structure2(i32);

impl fmt::Display for Structure2 {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

// You CANNOT use fmt::Display for any generic containers. fmt::Debug must be used for these
//  generic cases.


// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use self.number to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1) // leaving out the ';' makes this the same as a
        //                                          return statement

    }
}

// Define a structure where the fields are nameable for comparison
#[derive(Debug)]
struct Point2D{
    x: f64,
    y: f64,
}

// Now implement Display for 2D
impl fmt::Display  for Point2D {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}


pub fn display_test() {
    let minmax = MinMax(0, 14);

    println!("Compare Structures");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range);

    let point = Point2D{x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display {}", point);
    println!("Debug {:?}", point);

    let complex = Complex{real: 3.3, imag:7.2};

    println!("Compare Complex:");
    println!("Display {}", complex);
    println!("Debug {:?}", complex);

}