fn main() {
    println!("Hello, world!");

    // line comments

    /*
    Block comments
     */

    // Dco comments get parsed into HTML library documentation
    /// These generate library docs for the following item
    let y = 25000;
    // //! would generate library docs for the enclosing item.

    // You can manipulate expressions with block comments rather than line comments

    let x = 5 + /*90 + */ 5;
    println!("Is 'x' 10 or 100? x = {}", x);


    // Formatted printing
    // the '{}' will be automatically replaced with any arguments and be stringified
    println!("{} days", 31);


    // you could change the type of 31 by adding a suffix
    println!("{} days", 31i64);

    // The '{}' can use positional arguments
    println!(" {0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

    // println! can also use named arguments

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="The quick brown fox",
             verb="jumps over");

    // formating example withing strings
    println!("{} of {:b} people know binary, the other half do not", 1, 2);

    // specify the width of the text to right align it
    println!(" {number:>width$}", number=1, width=6);
    // pad numbers with extra zeros
    println!(" {number:>0width$}", number=1, width=6);

    // Rust will check to make sure the right number of arguments are used

    // #[allow(dead_code)]
    // struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.


    // std::fmt are utilities for formatting and printing Strings

    // fmt::Debug Uses the {:?} marker for debugging purposes
    // fmt::Display Uses the {} marker for formatting text

    // Implementing fmt:Display trait automatically implements the ToString trait which allows
    // us to convert the type to String

    let pi = 3.1415926535;
    println!("Pi is roughly {:.3}", pi);


    /*
    Debugging
    fmt::Debug trait, all types can derive the fmt::Debug implementation
     */

    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)] // This is an added attribute this struct is now printable with debug
    struct DebugPrintable(i32);

    // all std library types are automatically printable with {:} too

    // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
    // is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable
    // also.
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    // Pretty print
    println!("{:#?}", peter);

    // Regular print
    println!("Now Ugly {:?} will print!", peter);
}
