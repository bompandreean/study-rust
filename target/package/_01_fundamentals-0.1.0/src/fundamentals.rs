pub fn introduction() {
    // println!("Hello, world!");
    // println!("Show me ");
    // let unused_variable: u32 = 0;

    //an array of 2 floats
    let location: [f32; 2] = [0.0, 0.0];

    //tuple
    let location2 = ("kcle", 41.85226, -81.350);
    println!("Location name: {}, latitude: {}, longitude: {}", location2.0, location2.1, location2.2);

    let (name, latitude, longitude) = location2;
    println!("Location name: {}, latitude: {}, longitude: {}", name, longitude, latitude);

    //string and slice
    let person_name_slice = "Donald Mall";
    let person_name_string = person_name_slice.to_string();
    let name_string = String::from("donaldd mincj");
    let name_slice = &name_string; //gets the location of the string
    let name_slice2 = name_string.as_str();

    //concat on string
    let duck = "Duck";
    let airlines = "Airlenes";
    let airline_name = format!("{} {}", duck, airlines);
    println!("{}", airline_name);
    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "every time";
    println!("{}", slogan);
}

pub fn operators() {
    let modulus = 18 % 7;
    println!("{}", modulus);

    let square = i32::pow(8, 2);
    let float_integer = f32::powi(6.5, 3);
    let float_float = f32::powf(6.5, 3.14);
    println!("{}, {}, {}", square, float_integer, float_float);

    let are_equal_true = 1 == 1;
    let are_equal_false = 1 == 2;
    let not_eq = 1 != 2;

    let is_true = true;
    let is_false = !is_true;
    println!("{}, {}", is_true, is_false);

    let check_false = is_true && is_false;
    let check_true = is_true || is_false;
    println!("{}, {}", check_true, check_false);

    //bitwise
    let bitwise_and = 86 & 27;
    println!("bitwise and: {}", bitwise_and);

    let bitwise_or = 86 & 27;
    println!("bitwise or: {}", bitwise_or);

    let bitwise_xor = 86 ^ 27;
    println!("bitwise xor: {}", bitwise_xor);

    let left_shift = 86 << 1;
    println!("shift: {}", left_shift);
}

pub fn variables() {
    //  variables
    //Primitive data types
    let my_var_name: f32 = 1.0;
    let _warning_var = 0;

    //casting var
    // strongly typed language
    let float_thirty_two: f32 = 17.2;
    let unsignet_eight: u8 = 5;
    let cast_unsigned_eight = unsignet_eight as f32;
    println!("{}", float_thirty_two + cast_unsigned_eight);

    //imutability
    // let mut change_it = 5000;

    //scope and shadowing
    //shadowing = defining a variable with the same name as another
    let scope_test = "outer scope";
    println!("{}", scope_test);
    {
        let scope_test = "inner scope";
        println!("{}", scope_test);
    }
    println!("{}", scope_test);

    {
        println!("{}", scope_test);
    }
}


enum NavigationAid {
    NDB = 3,
    VOR = 2,
    VORDME = 5,
    // FIX { name: String, latitude: f32, longitude: f32 }
}

pub fn control_flow2() {
    //enums
    println!("NBD:\t{}", NavigationAid::NDB as u8);
    println!("VOR:\t{}", NavigationAid::VOR as u8);
    println!("VORDME:\t{}", NavigationAid::VORDME as u8);
}

pub fn control_flow() {
    //if
    let word = "duck";
    if word == "duck" {
        println!("quack");
    } else if word == "dog" {
        println!("woff")
    } else {
        println!("All quiet out here!")
    }
    println!("made it");

    let available_aircraft = "boeing";
    let minimum_crew = 7;
    let available_crew = 4;

    //and is verified first
    if available_aircraft == "boeing" || available_aircraft == "airbus"
        && minimum_crew <= available_crew {
        println!("ok");
    } else {
        println!("not ok");
    }

    //we can use () to change order
    if (available_aircraft == "boeing" || available_aircraft == "airbus")
        && minimum_crew <= available_crew {
        println!("ok");
    } else {
        println!("not ok");
    }
}
