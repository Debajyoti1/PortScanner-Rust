use std::io;
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::thread;
use std::time::{Duration, Instant};
fn main() {
    println!("Enter the Website/IP Address for Scanning");
    let mut input=String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_owned();
            //println!("{:?}", input.as_bytes());
            //println!("{}", input);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    input=input+":80";
    //println!("{}",input);

    let ip=input
    .to_socket_addrs()
    .expect("Please enter  }correct Website/ip address")
    .next()
    .unwrap()
    .ip();

    println!("{:?}",ip);

    /*
    for i in 1..1000{
        println!("{}",ip.to_string().clone()+":"+&i.to_string());
        if let Ok(_stream) = TcpStream::connect_timeout(&SocketAddr::new(ip, i),Duration::from_millis(150)) {
            println!("{}",i);
        }
    }
    */
    let mut threads=vec![];

    let start = Instant::now();
    for i in 1..=65535{
        let h=thread::spawn(move||{
            if let Ok(_stream) = TcpStream::connect_timeout(&SocketAddr::new(ip, i),Duration::from_millis(500)) {
                println!("{}",i);
            }
            /*
            else {
                println!("Unable to connect {}",ip.to_string().clone()+":"+&i.to_string());
            }
            */
        });
        threads.push(h);
    }
    //println!("Threads created");

    for i in threads{
        i.join().unwrap();
    }
    println!("Time taken {:?}",start.elapsed())

}
