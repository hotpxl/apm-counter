extern crate influent;

use std::process::Command;
use std::process::Stdio;
use std::process::Child;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::BufRead;
use std::thread;
use std::time::Duration;

use influent::create_client;
use influent::client::Client;
use influent::client::Credentials;
use influent::measurement::Measurement;
use influent::measurement::Value;

fn listen_for_output(program: &mut Child) {

    match program.stdout.as_mut() {
        Some(out) => {
            let buf_reader = BufReader::new(out);
            let cnt = Arc::new(Mutex::new(0));
            let cnt2 = cnt.clone();
            thread::spawn(move || {
                let credentials = Credentials {
                    username: "",
                    password: "",
                    database: "apm",
                };
                let hosts = vec!["http://localhost:8086"];
                let client = create_client(credentials, hosts);
                loop {
                    {
                        let mut c = cnt2.lock().unwrap();
                        let mut measurement = Measurement::new("key");
                        measurement.add_field("value", Value::Integer(*c / 2));
                        *c = 0;
                        client.write_one(measurement, None).unwrap();
                    }
                    thread::sleep(Duration::new(1, 0))
                }
            });

            for line in buf_reader.lines() {
                match line {
                    Ok(_) => {
                        let mut c = cnt.lock().unwrap();
                        *c += 1
                    }
                    Err(_) => return,
                };
            }
        }
        None => return,
    }
}

fn main() {
    let mut output = Command::new("xinput")
                         .arg("test")
                         .arg("11")
                         .stdin(Stdio::piped())
                         .stdout(Stdio::piped())
                         .stderr(Stdio::piped())
                         .spawn()
                         .unwrap();
    println!("Initialized.");
    listen_for_output(&mut output);
    println!("Done.");
}
