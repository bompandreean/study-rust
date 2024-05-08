#![allow(unused_variables)]

// use std::thread::scope;
mod fundamentals;
use fundamentals::{introduction, variables, operators, control_flow, control_flow2};

mod projects;
use projects::{project1, project2};

mod fundamentals_match;
use fundamentals_match::{match_1, match_2, match_with_enum};

mod memory_management;
use memory_management::{ownership, function_ownership, fn_with_closure};

mod error_handling;
use error_handling::{error_handling, error_propagation};

mod data_structures;
use data_structures::{data_structures, associated_methods, traits};

mod collections;
use collections::collections;

mod concurrency;
use concurrency::{concurrency_basic, concurrency_thread_communication};

fn main() {
    introduction();
    variables();
    operators();
    project1();
    control_flow();
    control_flow2();
    match_1();
    match_2();
    match_with_enum();
    project2();

    /* Memory management in rust
    !!!  OWNERSHIP AND BORROWING only apply to the data ON THE HEAP !!! doesn't apply to data on the stack

    it can be ONE AND ONLY ONE owner of a data at any give memory location at a time: two objects can't point to the same location
    Borrowing = allows another variable to take temporary ownership of data without deallocating the original variable

    CLOSURE " || " = are anonymous functions that can access variables from the method that they are created in
    */
    ownership();
    function_ownership();
    fn_with_closure();

    //ERROR handling :  errors can be recoverable and non-recoverable;
    error_handling();
    error_propagation();

    /*Data structures = like classes
        - are created using the word "struct"
        - the name of a struct should start with a big letter and is formated like in java: NameClass

      Associated methods
        - in a java project, a class an have both fields and behaviours(methods), in Rust the methods are moved to another place
        - the behaviour for a given "structure" is moved in "implementation" or impl

       Traits = like interfaces
         - define a shared behaviour between multiple structures
         - for implementing a trait we use the method association:
         ex: impl TraitName for StructName
     */
    data_structures();
    associated_methods();
    traits();

    /* COLLECTIONS
    Array and tuple are not collections, are on the stack memory and their size cannot change
        -collections stay on the heap memory, their size can change
        - can be: sequences or maps

        Sequences:
            = lists that let you add, remove, update, search for values within the list at runtime
            - values are not unique
            - store values sequentially : the values are ordered in the order you add them
            - vectors "Vec" or double ended queue vector "VecDeq", LinkedList

                Vector:
                    - simple list of values
                    - declared as "let mut my_vector:Vec<GenericType> = Vec::new()" or by setting the initial values of the vector "=vec![1,2,3]"
                Double ended queue vector: VecDeq:
                    - we can add both at the end and at the start of a list
                    - elements cannot be sorted
                    - has the properties of a vector
        Maps:
            = key - value pairs of generic types
            - if a key is already used, if inserting another key-value, the initial value will be overwriten
            - can be HashMaps and BTreeMaps
            - the difference between Hash and BTree is that BTree has the keys ordered
            "let my_map=HashMap::new"

        Sets:
            - are maps that have the "key" hidden, uses a hashmap/btreeMap internally
            - has uniq elements
     */
    collections();

    /*
        CONCURRENCY
        - creating threads: "thread::spawn" that returns a JoinHandle
        - to wait after a thread we call .join() method on the JoinHandle

        Thread communicate by passing messages between them or by sharing memory that is more risk
            Messages:
            - creating a chanel is done by calling mpsc::channel()
     */
    concurrency_basic();
    concurrency_thread_communication();
}











