use sonic_client::search::SearchChan;
use std::thread;
use std::time;

fn main() {
    let mut s = SearchChan::new("127.0.0.1", 1491, "SecretPassword").expect("Connection error");
    let handle = s.read();
    assert_eq!("CONNECTED <sonic-server v1.3.0>\r\n", s.connect().unwrap());
    thread::sleep(time::Duration::from_secs(4));
    let r1 = s
        .query("helpdesk", "user:0dcde3a6", "gdpr", Some(50), None)
        .unwrap();
    let r2 = s.ping().unwrap();
    let r3 = s.quit().unwrap();
    assert_eq!("EVENT", r1[0]);
    assert_eq!("PONG\r\n", r2.recv().unwrap());
    assert_eq!("ENDED quit\r\n", r3.recv().unwrap());
    handle.join().expect("Failed to wait process");
    println!("{}","All working. Done.");
}
