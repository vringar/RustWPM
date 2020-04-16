use webdriver_client::firefox::GeckoDriver;
use webdriver_client::messages::NewSessionCmd;
use webdriver_client::Driver;
use std::{thread, time};
use log::debug;
use simple_logger;
use crate::CommandSequenceBuilder;


fn main() {
    simple_logger::init().unwrap();
    debug!("First log");
    test();
    let driver = GeckoDriver::spawn().unwrap();
    let session = driver.session(& NewSessionCmd::default()).unwrap();
    session.go("https://example.com").unwrap();
    thread::sleep(time::Duration::from_secs(10));
}

fn test() {
    let builder = CommandSequenceBuilder{};
}