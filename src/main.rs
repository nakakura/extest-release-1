use rosrust;
use rosrust_msg;

fn main() {
    // Initialize node
    rosrust::init("talker");

    let mut count = 0;
    // Create object that maintains 10Hz between sleep requests
    let rate = rosrust::rate(10.0);

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
        rosrust::ros_info!("hello world {}", count);
        rate.sleep();
        count += 1;
    }
}

#[test]
fn test() {
    assert_eq!(1, 1);
}

#[test]
fn test2() {
    assert_eq!(2, 2);
}