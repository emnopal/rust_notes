#[cfg(test)]
struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8
}

// create method with impl which mean implement Person struct
#[cfg(test)]
impl Person {

    // every fn function which create here called Associated Functions
    // then if it has &self in it, the function is bound to their instance
    // if it hasn't &self in it, the function is not bound to their instance

    // why reference it (&self)? because to make struct ownership not change to another function which called it
    fn say_hello(&self) { // &self is instance of struct, same as python
        println!("Hello, {} {} {}", self.first_name, self.middle_name, self.last_name);
    }

    fn say_something(&self, text: &str) { // &self is instance of struct, same as python
        println!("{}, {} {} {}", text, self.first_name, self.middle_name, self.last_name);
    }

    fn say_happy_birthday(&self) { // &self is instance of struct, same as python
        println!("Happy Birthday {} {} {} {}", self.first_name, self.middle_name, self.last_name, self.age);
    }
}

#[test]
fn person_struct_test() {
    let person: Person = Person {
        first_name: String::from("Alexander"),
        middle_name: String::from("Graham"),
        last_name: String::from("Bell"),
        age: 59
    };
    person.say_hello();
    person.say_happy_birthday();
    person.say_something("Are you okay");
}

// or also we can create method without &self

#[cfg(test)]
struct GeoPoint(f64, f64);

#[cfg(test)]
impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint { // this method is not bound to their instance
        GeoPoint(long, lat)
    }
}

#[test]
fn test_geopoint() {
    let geo_point: GeoPoint = GeoPoint::new(-6.200000, 106.400000);
    println!("Longitude: {}", geo_point.0);
    println!("Latitude: {}", geo_point.1);
}