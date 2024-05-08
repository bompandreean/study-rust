use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

pub fn concurrency_thread_communication() {
    //creating a chanel is done by calling mpsc::chanel
    let (ion_sender, ion_receiver) = mpsc::channel();
    let (ana_sender, ana_receiver) = mpsc::channel();

    let ion_handle = thread::spawn(move || {
        ion_chat(ana_sender, ion_receiver);
    });

    let ana_handle = thread::spawn(move || {
        ana_chat(ion_sender, ana_receiver);
    });

    match ana_handle.join() {
        Ok(_) => {}
        Err(_) => {}
    };

    match ion_handle.join() {
        Ok(_) => {}
        Err(_) => {}
    };
}

fn ana_chat(ion_sender: Sender<&str>, ana_receiver: Receiver<&str>) {
    let result = ana_receiver.recv(); //blocks and waits for a message
    println!("Ana received a message {}", result.unwrap());

    let send_message = ion_sender.send("Salut Ioane, Ce faci bade?");
}

fn ion_chat(ana_sender: Sender<&str>, ion_receiver: Receiver<&str>) {
    let send_message = ana_sender.send("Ce faci, Ano?");

    let result = ion_receiver.recv();
    println!("Ion received a message {}", result.unwrap());
}

pub fn concurrency_basic() {
    let outer_scope = 365;
    let my_thread = thread::spawn(move || {
        outer_scope * 2
    });

    let result = my_thread.join();
    match result {
        Ok(value) => {
            println!("The result is ok: {:?}", value);
        }
        Err(_) => {}
    };
}
