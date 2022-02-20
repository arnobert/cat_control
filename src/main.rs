use sat_track::yaesu991;

mod sat_track;
use std::{thread, time};

fn main() {

    let mut sat_tracker = sat_track::sat_track::init();
    let ten_millis = time::Duration::from_millis(100);

    loop{
        sat_tracker.read_and_write();
        thread::sleep(ten_millis);
    }



}
