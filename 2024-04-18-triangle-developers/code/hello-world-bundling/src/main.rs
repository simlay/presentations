use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn main() {
    for i in 1..100 {
        // How long has it been since unix epoc?
        let since_unix_epoc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("Hello, world for the {i}'th time, {since_unix_epoc:?} seconds from unix epoc");
        // Let's time out
        sleep(Duration::from_millis(500));
    }
}
#[test]
fn add() {
    assert_eq!(1 + 1, 2);
}
