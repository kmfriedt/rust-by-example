fn main() {

    let logical: bool  = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32; // Suffix annotation, notice there is no ':'

    // default if no annotation is given
    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type= 4294967296i64;

    let mut mutable_int = 12;
    mutable_int = 21; // a mutable variable's value can be changed

    //mutable_int = true; // The type of any variable cannot be changed.

    // You can however override variables with shadowing.

    let mutable_int = true;


}
