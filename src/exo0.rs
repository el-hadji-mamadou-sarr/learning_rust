struct Person {
    age: i32,
}

impl Person {
    fn old_age() -> Self {
        Self { age: 88 }
    }
    fn show_age(&self) {
        println!("{:?}", self.age);
    }
}

pub fn function() {
    let person = Person { age: 22 };

    Person::show_age(&person);
    person.show_age();

    let personb = Person::old_age();
    personb.show_age();
}
