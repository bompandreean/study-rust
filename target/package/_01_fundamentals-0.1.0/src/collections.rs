use std::collections::{HashMap, HashSet};
use std::collections::vec_deque::VecDeque;

pub fn collections() {
    // vectors
    let mut my_first_vector: Vec<i32> = Vec::new();
    my_first_vector.push(1);
    my_first_vector.push(3);
    my_first_vector.push(4);

    let value_at_index_panicking = my_first_vector[0];
    let value_at_index = my_first_vector.get(0);
    if let Some(value_at_missing_index) = my_first_vector.get(10) {
        println!("Index was found!")
    } else { println!("index is missing!") }

    my_first_vector.insert(1, 2); //inser at index 1, the value =2

    for i in my_first_vector.iter() {
        println!("{}", i)
    }

    let mut my_second_vector: Vec<&str> = vec!["ala", "bala", "portocala"];
    my_second_vector.push("mamaliga cu carnati");

    for i in my_second_vector.iter() {
        println!("{}", i)
    }

    //DoubleEndedQueueVector or VecDeq
    let mut vec_dec1: VecDeque<u16> = VecDeque::new();
    vec_dec1.push_back(0);
    vec_dec1.push_back(3);
    vec_dec1.push_back(1);
    vec_dec1.push_front(5);
    vec_dec1.push_back(10);
    vec_dec1.push_front(11);
    print!("VecDeq: ");
    //iterates over a collection not changing it's values
    for i in vec_dec1.iter() {
        print!("{}, ", i)
    }
    println!();

    //iterates over a colection changing the values:
    let mut limit: u16 = 10;
    for i in vec_dec1.iter_mut() {
        if i >= &mut limit {
            *i -= limit
        }

        print!("{}, ", i)
    }

    //MAPS:
    let mut my_map = HashMap::new();
    my_map.insert("Milo", (0.6, "Male"));
    my_map.insert("Aria", (0.3, "Female"));
    my_map.insert("Kitty", (1.6, "Female"));

    if !my_map.contains_key("Aria") {
        my_map.insert("Aria", (0.7, "F"));
    } else {
        println!("Aria is already in the map")
    }

    //SETS
    let mut my_set = HashSet::new();
    my_set.insert(5);
    my_set.insert(5);
    my_set.insert(15);
    my_set.insert(5);

    print!("Set: ");
    for i in my_set.iter() {
        print!("{}, ", i)
    }
}