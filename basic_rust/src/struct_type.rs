// Example of struct syntax
#[cfg(test)]
struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8
}

#[test]
fn struct_people_test() {
    let person: Person = Person {
        first_name: String::from("Alexander"),
        middle_name: String::from("Graham"),
        last_name: String::from("Bell"),
        age: 59
    };
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

// or you can shorten the struct initialization with init shorthand
#[test]
fn struct_people_test2() {
    let first_name = String::from("Alexander");
    let last_name = String::from("Bell");
    let person: Person = Person {
        first_name,
        middle_name: String::from("Graham"),
        last_name,
        age: 59
    };
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

// Struct as params
#[cfg(test)]
fn struct_as_params(person: &Person) { // consider to use `&` here for safety
    println!("First Name: {}", person.first_name);
    println!("Middle Name: {}", person.middle_name);
    println!("Last Name: {}", person.last_name);
    println!("Age: {}", person.age);
}

#[test]
fn struct_as_params_test() {
    let person: Person = Person {
        first_name: String::from("Alexander"),
        middle_name: String::from("Graham"),
        last_name: String::from("Bell"),
        age: 59
    };
    struct_as_params(&person);
}

#[test]
fn struct_people_update_test() {
    let first_name = String::from("Alexander");
    let last_name = String::from("Bell");
    let mut person: Person = Person { // make struct mutable
        first_name,
        middle_name: String::from("Graham"),
        last_name,
        age: 59
    };
    println!();
    println!("Before Update");
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);

    // by default, struct is immutable
    // but you can update struct if the struct is mutable

    person.age = 74;
    println!();
    println!("After Update");
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

// or you can create new struct with this
#[test]
fn struct_people_update_test2() {
    let first_name = String::from("Alexander");
    let last_name = String::from("Bell");
    let person1: Person = Person { // make struct mutable
        first_name,
        middle_name: String::from("Graham"),
        last_name,
        age: 59
    };
    println!();
    println!("person1");
    println!("{}", person1.first_name);
    println!("{}", person1.middle_name);
    println!("{}", person1.last_name);
    println!("{}", person1.age);

    // using person1 data to person2 with change their ownership
    // warning: consider it first for using statement like this
    // since it will change the ownership
    let person2: Person = Person { ..person1 }; // value move here
    println!();
    println!("person2");
    println!("{}", person2.first_name);
    println!("{}", person2.middle_name);
    println!("{}", person2.last_name);
    println!("{}", person2.age);
    // println!("person1 attemp, {}, {}", person1.first_name, person1.age); // will panic here since person1 has changed their ownership to person2

    // so better to use this
    // we want to use person2 data which is person1 data and person1 is invalid
    // since the ownership have taken by person2
    let person3: Person = Person { 
        first_name: person2.first_name.clone(),
        middle_name: person2.middle_name.clone(),
        last_name: person2.last_name.clone(),
        // age: person2.age.clone(),
        age: 72, // or u can change the age value
     };
    println!();
    println!("person3");
    println!("{}", person3.first_name);
    println!("{}", person3.middle_name);
    println!("{}", person3.last_name);
    println!("{}", person3.age);
}

// or we can create struct with tuple (no named struct)
#[cfg(test)]
struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.200000, 106.400000);
    println!("Longitude: {}", geo_point.0);
    println!("Latitude: {}", geo_point.1);
}

// or even we can create struct without field
// this is equivalent with Unit() or ()
#[cfg(test)]
struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing{};
}

// what about reference in struct
// such as &str (string slice)
// we need to use Lifetime to use reference in struct
// see more at: Lifetime.rs