use webdriver_client::firefox::GeckoDriver;
use webdriver_client::messages::NewSessionCmd;
use webdriver_client::Driver;
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let driver = GeckoDriver::spawn().unwrap();
    let session = driver.session(& NewSessionCmd::default()).unwrap();
    session.go("https://example.com").unwrap();
    thread::sleep(time::Duration::from_secs(10));
}
