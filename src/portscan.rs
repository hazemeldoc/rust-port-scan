use std::net::{SocketAddr};
use std::time::Duration;
use tokio::time::sleep;
use tokio::net::TcpStream;

pub async fn start_scan_1000 (l:u16,oct1:u8,oct2:u8,oct3:u8,oct4:u8)-> bool
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