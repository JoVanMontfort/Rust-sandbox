struct Dog {
    name: String,
    breed: String,
}

struct Cat {
    name: String,
}

trait Animal {
    fn speak(&self);
    fn name(&self) -> &str;
}

trait Mammal: Animal {
    fn feed_baby(&self) {
        println!("{} feeds her baby.", self.name());
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says: Meow!", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Mammal for Dog {}

fn make_it_speak<T: Animal>(animal: &T) {
    animal.speak();
}

fn main() {
    let dog = Dog {
        name: "Rex".into(),
        breed: "German Shepherd".into(),
    };

    let cat = Cat {
        name: "Luna".into(),
    };

    make_it_speak(&dog);
    make_it_speak(&cat);

    dog.feed_baby();
}
