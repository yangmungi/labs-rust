use std::io::prelude::*;

use std::collections::hash_map::{HashMap,Entry};
use std::hash::Hash;
use std::net::{TcpListener,Shutdown};
use std::thread;

struct ChangeRequest <Key, Value> {
    key: Key,
    old: Option<Value>,
    new: Option<Value>,
}

fn logical_handle<KeyType: Eq + Hash, ValueType: Eq>(mut old_values: HashMap<KeyType, ValueType>, change_request: ChangeRequest<KeyType, ValueType>) {
    let key = change_request.key;
    let maybe_old_expected_value = change_request.old;
    let maybe_new_value = change_request.new;

    match old_values.entry(key) {
        Entry::Occupied(mut occupied) => match maybe_old_expected_value {
            Some(old_value) => if old_value == *occupied.get() { 
                match maybe_new_value {
                    Some(new_value) => {
                        occupied.insert(new_value);
                        ()
                    },
                    None => {
                        occupied.remove();
                        ()
                    }
                }
            },
            None => (),
        },
        Entry::Vacant(v) => match maybe_old_expected_value {
            Some(_) => (),
            None => { 
                match maybe_new_value {
                    Some(new_value) => {
                        v.insert(new_value); 
                    },
                    None => { 
                        ()
                    }
                }
            }
        }
    }
}

fn stream_handle<StreamType: Read>(mut stream: StreamType) {
    // transfer protocol
    // u8 coded length for key
    // first 2 bits
    //   00 -> next 6 bits = u8 -> 0 - 64 bytes
    //   01 -> num is 6 + 8 bits = u16 -> 0 - 16384 bytes
    // the next two seem impractical...
    //   10 -> num is 6 + 24 bits = u32 -> 0 - 2^30 bytes
    //   11 -> num is 6 + 56 bits = u64 -> 0 - 2^62 bytes

    //let mut buffer = [0; 10];
    //try!(stream.read(&mut buffer[..]));

    let mut buffer = String::new();
    match stream.read_to_string(&mut buffer) {
        Ok(read) => println!("Read {} bytes", read),
        Err(_) => println!("Could not read!"),
    }

    println!("Read: \"{}\"", buffer);
}

fn main() {
    let listen_addr = "0.0.0.0:4000";
    let listener = TcpListener::bind(&listen_addr)
        .unwrap();

    println!("Listening on {}", listen_addr);

    //let old_values = &mut HashMap::new();

    for stream_listener in listener.incoming() {
        match stream_listener {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let peer_addr = stream.peer_addr()
                        .expect("Undetermined Peer Address");

                    println!("Connected {}", peer_addr);

                    stream_handle(&mut stream);

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
