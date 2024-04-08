/*
Rust has 2 string types, fixed size &str and the flexible one String
&str is fixed size, so rust will store it inside the stack
String is flexible in size, so rust will store it inside the heap

so since &str is stack, it means the data is immutable
so if we create something like `let mut a: &str` or manipulate that which some method
it doesn't change the value of a, instead it will create new &str value or converted into String
*/

// read more here: https://doc.rust-lang.org/std/primitive.str.html
#[test]
fn fixed_string() {
    let username: &str = "   Alexander Graham Bell    ";
    println!("{}", username);
    let username_trim: &str = username.trim(); // username and username_trim aren't same reference!
    println!("{}", username_trim);

    let mut changed_username: &str = "Alexander Graham Bel"; // this
    println!("{}", changed_username);
    changed_username = "Alexander Graham Bell"; // isn't same reference as this!
    println!("{}", changed_username);

    let mut other_username: &str = "   Alexander Graham Bell    "; // this
    println!("{}", other_username);
    other_username = other_username.trim(); // isn't same reference as this
    println!("{}", other_username);
}

/*
String by default is UTF-8

note: if String created without `let mut` which means it's immutable
it still stored in heap

String has method to changed it's own data or create a copy of data!
read more here: https://doc.rust-lang.org/std/string/struct.String.html

also note
without String::from(""); it consider as &str
String => String::from("");
&str => "";
*/
#[test]
fn string_type() {
    let username_immutable: String = String::from("Alexander Graham Bell"); // this stores in heap
    println!("{}", username_immutable);

    let mut username: String = String::from("Alexander");
    println!("{}", username);

    // to add String into String with push
    username.push_str(" Graham ");
    println!("{}", username);

    // to add String into String without push
    username = username + "Bel";
    println!("{}", username);

    let fix_typo_username = username.replace("Bel", "Bell");
    println!("{}", fix_typo_username);
}

#[test]
// this is stack frame
fn string_str_memory() {
    let name1 = "Alexander"; // this store inside stack
    println!("{name1}");

    let name2 = String::from("Bell"); // this store inside heap
    println!("{name2}");
}