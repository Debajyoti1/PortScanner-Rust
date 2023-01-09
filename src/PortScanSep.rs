use std::cmp;
use std::convert::TryInto;
use std::io;
use std::net::{TcpStream, SocketAddr, ToSocketAddrs, IpAddr};
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
    .expect("Please enter correct Website/ip address")
    .next()
    .unwrap()
    .ip();

    println!("{:?}",ip);

    let start = Instant::now();
    create_threads(ip,1);
    create_threads(ip,20000);
    create_threads(ip,40000);
    create_threads(ip,60000);
    println!("Time taken {:?}",start.elapsed())

}

fn create_threads(ip: IpAddr,start:u32){
    let mut threads=vec![];
    //println!("{:?}",ip);
    let end=cmp::min(start+20000,65535);
    let s: u16=start.try_into().unwrap();
    let e: u16=end.try_into().unwrap();
    for i in s..=e{
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
    for i in threads{
        i.join().unwrap();
    }
    threads=Vec::new();
}