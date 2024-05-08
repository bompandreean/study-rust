enum NavigationAid2 {
    NDB(u16),
    VOR(String, f32),
    VORDME(String, f32),
    FIX { name: String, latitude: f32, longitude: f32 },
}

pub fn match_with_enum() {
    let ndb = NavigationAid2::NDB(325);
    let vor = NavigationAid2::VOR(String::from("EsvhsFRhs"), 259.15);
    let vordme = NavigationAid2::VORDME(String::from("Lala"), -518.365);
    let fix = NavigationAid2::FIX {
        name: String::from("Nothh"),
        latitude: -123.65,
        longitude: 87.23,
    };

    print_nav_aid(&ndb);
    print_nav_aid(&vor);
    print_nav_aid(&vordme);
    print_nav_aid(&fix);
}

fn print_nav_aid(navaid: &NavigationAid2) {
    match navaid {
        NavigationAid2::NDB(khz) => println!("NBD frequency is {} kilohertz", khz),
        NavigationAid2::VOR(name, khz) => println!("VOR with name {} is {} kilohertz", name, khz),
        NavigationAid2::VORDME(name, khz) => println!("VORDME with name {} is {} kilohertz", name, khz),
        NavigationAid2::FIX { name, latitude, longitude } => println!("FIX with name {} and {} {} ", name, latitude, longitude),
    }
}

pub fn match_2() {
    // like switch
    let animal = "duck";
    match animal {
        "duck" => println!("quack"),
        "dog" => println!("woff"),
        _ => println!("all quiet")
    };

    let freq = 386;
    //range freq>300 && freq <=500
    //inclusive range
    match freq {
        300..=500 => println!("valid NDB frequency"),
        _ => println!("not valid")
    }

    //exclusive range can be made only with if
    match freq {
        freq if freq < 200 || freq >= 500 => { println!("valid NDB frequency") }
        _ => println!("not valid")
    }
}

pub fn match_1() {
    let prop = String::from("hello there");
    let letter1 = prop.chars().nth(3);
    let letter2 = prop.chars().nth(99);

    let check_letter1 = match letter1 {
        Some(character) => character.to_string(),
        None => String::from("no char found")
    };
    let check_letter2 = match letter2 {
        Some(character) => character.to_string(),
        None => String::from("no char found")
    };

    println!("for l1 {}", check_letter1);
    println!("for l2 {}", check_letter2);
}