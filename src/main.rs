mod yaesu991;


fn main() {

    let path = String::from("/dev/ttyUSB0");
    let baud = 9600;
    let mut radio = yaesu991::Y991::init(&path, baud);

    radio.set_band_80m();
   // let ptest = radio.get_freq();
   // println!("GOT: {}", ptest);

}
