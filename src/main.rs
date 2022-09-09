use std::net::{SocketAddr};
use std::time::Duration;
use std::vec::Vec;
use tokio::time::sleep;
use tokio::net::TcpStream;
use clap::{App, Arg};
#[tokio::main]
async fn main() {

    let param = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("target")
                .help("The target to scan")
                .required(true)
                .index(1),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    let target=param.value_of("target").unwrap();
    let split = target.split(".");
    let vec = split.collect::<Vec<&str>>();
	let octet1=vec[0].parse::<u8>().unwrap();
    let octet2=vec[1].parse::<u8>().unwrap();
    let octet3=vec[2].parse::<u8>().unwrap();
    let octet4=vec[3].parse::<u8>().unwrap();

    let mut handles_parent=vec![];
    for l in 0..66
    {
        let handle_parent = tokio::spawn(async move{
        if start_scan_1000(l,octet1,octet2,octet3,octet4).await==true
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
async fn start_scan_1000 (l:u16,oct1:u8,oct2:u8,oct3:u8,oct4:u8)-> bool
{
    let mut handles=vec![];
        
    for i in 0..1001
    {
        let handle = tokio::spawn(async move{
        if test_port(oct1,oct2,oct3,oct4,i+(l*1000)).await == true
        {
            println!("port {} is open",i+(l*1000));
        }
        });
        if i+(l*1000) >= 65535{
            return true;
        }
        handles.push(handle);
    }
    
    for handle in handles
    {
        handle.await.unwrap();
    }

    sleep(Duration::from_millis(300)).await;
    return false;
}
async fn test_port(oct1:u8,oct2:u8,oct3:u8,oct4:u8,port:u16) -> bool
{       
    let addr=SocketAddr::from(([oct1,oct2,oct3,oct4],port));
    let time_out=Duration::new(3    ,0);
    match tokio::time::timeout(time_out, TcpStream::connect(addr)).await {
        Ok(Ok(_)) => return true,
        _ => {}
    }
    return false;
 }
