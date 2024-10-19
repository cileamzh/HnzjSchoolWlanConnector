use cileamzh_web::{meb::ToVec, HttpRequest};
use std::{
    env::current_exe,
    fs::read,
    io::{BufRead, Read, Write},
    net::TcpStream,
    process::Command,
    thread,
    time::Duration,
};
fn main() -> std::io::Result<()> {
    let stp = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
        .join("setting.config");

    let mut args: Vec<String> = Vec::new();
    let binary: Vec<u8> = read(stp).unwrap();
    for l in binary.lines() {
        args.push(l.unwrap());
    }

    let cout = Command::new("netsh")
        .args([
            "wlan",
            "connect",
            "name=\"HNZJ-Student\"",
            "ssid=\"HNZJ-Student\"",
        ])
        .output()?;
    let coutr = String::from_utf8_lossy(&cout.stdout);
    println!("{}", coutr);
    if coutr.to_string().contains("successfully") {
        thread::sleep(Duration::from_secs(1));
        let output = Command::new("ipconfig").output().unwrap();
        let cmdout = String::from_utf8_lossy(&output.stdout);

        let mut all_ipv4: Vec<String> = Vec::new();
        for line in cmdout.lines() {
            if line.contains("IPv4 Address") {
                all_ipv4.push(line.split(": ").nth(1).unwrap().to_string());
            }
        }
        let qp = format!(
            "http://172.16.1.38:801/eportal/?c=ACSetting&a=Login&loginMethod=1&protocol=http%3A&hostname=172.16.1.38&port=&iTermType=1&wlanuserip={}&wlanacip=172.20.1.1&wlanacname=&redirect=null&session=null&vlanid=0&mac=00-00-00-00-00-00&ip={}&enAdvert=0&jsVersion=2.4.3&DDDDD=%2C0%2C{}%40cmcc&upass={}&R1=0&R2=0&R3=0&R6=0&para=00&0MKKey=123456&buttonClicked=&redirect_url=&err_flag=&username=&password=&user=&cmd=&Login=&v6ip=",
            all_ipv4[0], all_ipv4[0], args[0], args[1]
        );

        let mut req = HttpRequest::new();
        req.method = "GET".to_string();
        req.protocol = "HTTP/1.1".to_string();
        req.path = qp;
        req.push_header("host: 172.16.1.38:801");
        let mut s = TcpStream::connect("172.16.1.38:801")?;
        s.write(&req.to_vec_u8()).unwrap();
        s.flush()?;
        let mut buf = Vec::new();
        s.read_to_end(&mut buf)?;
        let r = String::from_utf8_lossy(&buf);
        println!("{}", r);
        if r.contains("ErrorMsg") {
            let mut s = TcpStream::connect("172.16.1.38:801")?;
            s.write(&req.to_vec_u8()).unwrap();
            s.flush()?;
        }
    }

    Ok(())
}
