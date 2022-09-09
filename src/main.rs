use std::env;
use std::net::{SocketAddr};
use std::process::exit;
use std::time::Duration;
use std::vec::Vec;
use tokio::time::sleep;
use tokio::net::TcpStream;
use clap::{App, Arg};

mod file_read;
mod portscan;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut Q=vec![];
    file_read::read_file_line_by_line(path, &mut Q);
    for i in 0..Q.len()
    {
        
        let target=Q.pop().unwrap();
        println!("---------------{}-------------",target);
        let split = target.split(".");
        let vec = split.collect::<Vec<&str>>();
	    let octet1=vec[0].parse::<u8>().unwrap();
        let octet2=vec[1].parse::<u8>().unwrap();
        let octet3=vec[2].parse::<u8>().unwrap();
        let octet4=vec[3].parse::<u8>().unwrap();
        start_scan(octet1,octet2,octet3,octet4).await;
    }
}

async fn start_scan(octet1:u8,octet2:u8,octet3:u8,octet4:u8)
{
    let mut handles_parent=vec![];
    for l in 0..66
    {
        let handle_parent = tokio::spawn(async move{
        if portscan::start_scan_1000(l,octet1,octet2,octet3,octet4).await==true
        {
           //still workin on it
        }
        });
        sleep(Duration::from_millis(200)).await;
        handles_parent.push(handle_parent);
    }
    for handle_parent in handles_parent
    {
        sleep(Duration::from_millis(200)).await;
        handle_parent.await.unwrap();
    }
}
