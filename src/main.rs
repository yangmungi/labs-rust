use std::io::prelude::*;

use std::collections::hash_map::{HashMap,Entry};
use std::net::{TcpListener,Shutdown};
use std::thread;

struct ChangeRequest <Key, Value> {
    key: Key,
    old: Option<Value>,
    new: Value,
}

fn main() {
    let test_request = ChangeRequest { key: "test", old: None, new: &"to" };

    let key = &test_request.key;
    let maybe_old_expected_value = test_request.old;
    let new_value = test_request.new;

    let mut old_values: HashMap<&str, &str> = HashMap::new();
    match old_values.entry(key) {
        Entry::Occupied(mut o) => match maybe_old_expected_value {
            Some(old_value) => if old_value == o.get() { Some(o.insert(new_value)); },
            None => ()
        },
        Entry::Vacant(v) => match maybe_old_expected_value {
            Some(_) => (),
            None => { v.insert(new_value); () }
        }
    }

    let listen_addr = "0.0.0.0:4000";
    let listener = TcpListener::bind(&listen_addr)
        .unwrap();

    println!("Listening on {}", listen_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // TODO handle any thread spawning with a controller
                thread::spawn(move || {
                    let peer_addr = stream.peer_addr()
                        .expect("Undetermined Peer Address");

                    println!("Connected {}", peer_addr);
                    let mut buffer = String::new();

                    match stream.read_to_string(&mut buffer) {
                        Ok(v) => println!("Read Bytes: {}", v),
                        Err(e) => println!("Error: {}", e),
                    }

                    println!("Read: \"{}\"", buffer);

                    stream.shutdown(Shutdown::Both)
                        .expect("Could not shut down");

                    println!("Closed {}", peer_addr);
                });
                
            }
            Err(e) => {
                println!("Error:{}", e);
            }
        }
    }
}
