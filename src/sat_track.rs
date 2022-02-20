pub mod yaesu991;

use math::round;

pub struct sat_track{
    radio: yaesu991::Y991,
    VfoA_old: f64,
    VfoB_old: f64
}

impl sat_track {
    pub fn init() -> sat_track {
        let path = String::from("/dev/ttyUSB0");
        let baud = 9600;

        sat_track{
            radio:  yaesu991::Y991::init(&path, baud),
            VfoA_old: 0.0,
            VfoB_old: 0.0

        }
    }


    pub fn read_and_write(&mut self) {

        if self.VfoA_old == 0.0 {
            self.VfoA_old = self.radio.get_freq(false);
        }
        let VfoA = self.radio.get_freq(false);
        let VfoB = self.radio.get_freq(true);



        let dif_A = VfoA - self.VfoA_old;



        self.radio.set_freq(round::floor(VfoB + dif_A, 5), true);
        self.VfoA_old = VfoA;
    }

}
