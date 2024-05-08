pub fn fn_with_closure() {
    let company = "Wizz Air";

    let closure_method = |msg: String| -> String {
        String::from(format!("{}, {}", company, msg))
    };

    println!("{}", closure_method(String::from("We hit the ground every time!")));
}


pub fn function_ownership() {
    let mut original = String::from("original");

    print(&original);
    change_var(&mut original);
    print(&original)
}

fn print(var: &String) {
    println!("printing value {} ", var);
}

fn change_var(var: &mut String) {
    //borrow reference
    let next = var;
    //change value
    *next = String::from("next var");
    println!("changing value {} ", next);
}

pub fn ownership() {
    let original = String::from("original value");
    println!("original {} ", original);
    //error:
    // let unnoriginal = original;
    // println!("original {} ", original); //error is thrown

    //barrowing
    let next = &original;
    println!("original {} ", original);

    //lifetime
    // let outer_scope;
    // {
    //     let inner_scope = 5;
    //     outer_scope = &inner_scope;
    // }
    //
    // println!("outer scope {}", outer_scope); //dangling reference

    let value1 = 63;
    let value2 = 77;
    let bigger = find_bigger(&value1, &value2);
}

// 'a is the lifetime
fn find_bigger<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
    if p1 > p2 {
        p1
    } else {
        p2
    }
}