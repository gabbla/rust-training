
const MY_CONST: u32 = 100000;


fn main() {
    let mut x = 5;
    println!("X: {}", x);
    x = 6;
    println!("X: {}", x);

    println!("MYCONST: {}", MY_CONST);

    let tuple: (u32, f64, u8) = (34, 45.5, 4);
    // Adding an _ (underscore) before a var that is unused
    // prevents the warining abou its usage
    // this assign is called destructuring
    let (_a, b, _c) = tuple;
    println!("Value of b: {}", b);

    // We can also access the tuple elements with a . (period)
    let k = tuple.0;
    println!("tuple[0]: {}", k);

    let array = [0, 1, 2, 3, 4, 5];
    let mut another_array: [i32; 5] = [1, 2, 3, 4, 5];

    another_array[4] = 44;

    println!("array[0]: {}", array[0]);
    println!("another_array[4]: {}", another_array[4]);

    function(array[3]);

}

// This function is defined after the main
// Rust doesn't care the definition order, 
// only that a function is defined somewhere
fn function(x: u32) {
    println!("Wow, a function with a param: {}", x);
}
