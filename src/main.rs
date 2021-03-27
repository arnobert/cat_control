mod yaesu991;


fn main() {

    let mut radio = yaesu991::Y991::init();

    radio.set_band_80m();
   // let ptest = radio.get_freq();
   // println!("GOT: {}", ptest);
 
}
