
trait Feline {
    fn eats(&self, kg_of_meat: f32) -> String;
}

struct Leon {
    age: i32,
    name: String,
}

impl Feline for Leon {
    fn eats(&self, kg_of_meat: f32) -> String {
        String::from(format!("I'm a Leon named {} and I ate {} kgs of meat today", self.name, kg_of_meat))
    }
}

impl Feline for Cat {
    fn eats(&self, kg_of_meat: f32) -> String {
        String::from(format!("I'm a Cat named {} and I ate {} g of meat today", self.name, kg_of_meat * 1000.00))
    }
}

pub fn traits() {
    let milo = Cat::new(1, "Milo".to_string(), "fun".to_string());
    let leon = Leon {
        age: 10,
        name: "Leonel Messi".to_string(),
    };

    println!("{}", milo.eats(0.05));
    println!("{}", leon.eats(50.00));
}

impl Cat {
    //it's like a constructor
    // "Self" is used instead of the class name
    fn new(age: i32, name: String, behaviour: String) -> Self {
        Self {
            age,
            name,
            behaviour,
        }
    }

    fn speak(&self) {
        println!("Miuauu, mrrrriuau");
    }
}

pub fn associated_methods() {
    let kitty = Cat::new(2, "Kitty".to_string(), "cea mai iubitoare pisica din lume".to_string());
    kitty.speak();
}

struct Cat {
    age: i32,
    name: String,
    behaviour: String,
}

struct House {
    cat1: Cat,
    cat2: Cat,
    owner: String,
}

pub fn data_structures() {
    let milo = Cat {
        age: 1,
        name: "Milo".to_string(),
        behaviour: "torcatorul meu mic".to_string(),
    };

    let aria = Cat {
        name: "Aria".to_string(),
        behaviour: "pisalcoasa mica".to_string(),
        ..milo
    };

    let house = House {
        cat1: milo,
        cat2: aria,
        owner: "Gigi + Andreea".to_string(),
    };
}

