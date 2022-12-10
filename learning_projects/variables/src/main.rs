fn main()
{
    // Modification of x requires the mut qualifier or it won't compile
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // This is a const, unlike a variable its always immutable and must be defined immediately
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of const THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // Let essentially declares the variable again, so even if the first statement uses mut, this still works
    let x = 5;
    let x = x + 1;

    // Shadowing allows for x to be something within one scope, and another outside of it
    {
        // Here x is 12, but once this scope is exited, x is back to 6
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
