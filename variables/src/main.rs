fn main() {
    const COOL_VAR: u32 = 199;

    let mut x: u32 = 5;
    println!("The value of x is {x}");

    x = 1 + COOL_VAR;
    println!("the value of x is {x}");

    // shadowing ------------------------------------
    let y = 5;

    let y = y + 1; // shadowing in the main scope - this sticks

    // shadowing happens PER SCOPE - ie. you can shadow y in the inner scope - will revert back out
    // of scope
    {
        let y = y * 2;
        println!("The value of y in the inner scope : {y}");
    }

    println!("The value of y is: {y}");

    // since shadowing is essentially creating a new variable with the same name we can even change
    // types
    let spaces = "    ";
    println!("<{spaces}>");

    let spaces = spaces.len();
    println!("How many spaces above: {spaces}");
}
