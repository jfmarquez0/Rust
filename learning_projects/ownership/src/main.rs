fn main()
{
    // String literal
    let s_basic = "hello";

    // String type
    let mut s = String::from("hello");

    // push_str() appends a literal to a String
    s.push_str(", world!");

    println!("{}", s_basic);

    // This will print `hello, world!`
    println!("{}", s);
}
