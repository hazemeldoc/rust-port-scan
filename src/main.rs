use std::net::IpAddr;
use std::{env, vec};
use std::time::Duration;
use std::vec::Vec;
use tokio::time::sleep;
use dns_lookup::lookup_host;
use std::str::FromStr;
use clap::{App, Arg};

mod file_read;
mod portscan;
#[tokio::main]
async fn main() {
    let param = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("file")
                .help("file to read from")
                .short("f")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("sleep")
                .help("make the tool sleep between request")
                .long("sleep")
                .short("s")
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();


    let path=param.value_of("file").unwrap();
    let mut Q=vec![];
    file_read::read_file_line_by_line(path, &mut Q);
    for _i in 0..Q.len()
    {
        let mut target=Q.pop().unwrap();
        println!("---------------{}-------------",target);
        let mut ips: Vec<std::net::IpAddr> = lookup_host(&target).unwrap_or(vec![IpAddr::from_str("99.99.99.99").unwrap()]);
        target=ips.pop().unwrap().to_string();
        if target=="99.99.99.99".to_string()
        {
            //i know it's such a bad way to handle the error ;) ,but it's not stupid if it works
            continue;
        }
        let split = target.split(".");
        let vec = split.collect::<Vec<&str>>();
	    let octet1=vec[0].parse::<u8>().unwrap();
        let octet2=vec[1].parse::<u8>().unwrap();
        let octet3=vec[2].parse::<u8>().unwrap();
        let octet4=vec[3].parse::<u8>().unwrap();
        start_scan(octet1,octet2,octet3,octet4,param.is_present("sleep")).await;
    }
}

async fn start_scan(octet1:u8,octet2:u8,octet3:u8,octet4:u8,sleep_:bool)
{
    let mut handles_parent=vec![];
    for l in 0..66
    {
        let handle_parent = tokio::spawn(async move{
        if portscan::start_scan_1000(l,octet1,octet2,octet3,octet4,sleep_).await==true
        {
           //still workin on it
        }
        });
        if sleep_
        {
            sleep(Duration::from_millis(300)).await;
        }
        else
        {
            sleep(Duration::from_millis(10)).await;
        }
        handles_parent.push(handle_parent);
    }
    for handle_parent in handles_parent
    {
        if sleep_
        {
            sleep(Duration::from_millis(200)).await;
        }
        handle_parent.await.unwrap();
    }
}
