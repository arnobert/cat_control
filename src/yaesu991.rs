use std::io::*;
use std::time::Duration;

use serialport::SerialPort;

use std::borrow::Cow;

pub struct Y991 {
    port: serialport::TTYPort,
}

impl Y991 {
    pub fn init(path: &String, baud: u32) -> Y991 {
        let p1 = serialport::new(path, baud);
        let mut prt = p1.open_native().expect("No tty found");

        let tout = Duration::from_millis(500);

        prt.set_timeout(tout).expect("Setting timeout failed");

        Y991 { port: prt }
    }


    fn set(&mut self, cmd: &[u8]) {
        &self.port.write(cmd).expect("write failed");
    }

    fn get(&mut self) -> String{
        const eol_sign: u8 = 59; //;
        const CMDLEN: usize = 1;

        let mut rdbuf: [u8; CMDLEN] = [0; CMDLEN];
        let mut ipbuf: Vec<u8> = Vec::new();

        loop {
            self.port.read_exact(&mut rdbuf).expect("Read error");
            ipbuf.push(rdbuf[0]);

            if eol_sign == rdbuf[0] {
                break;
            }
        }

        let strout = String::from_utf8_lossy(&ipbuf);
        Cow::from(strout).to_string()
    }

//--------- PUBLIC FUNCTIONS FOLLOW HERE ---------------------

//--------- AB: VFO-A TO VFO-B -------------------------------
    pub fn set_a2b(&mut self){
        self.set("AB;".as_bytes());
    }

//--------- AC: ANTENNA TUNER CTRL ---------------------------
    pub fn set_tuner_on(&mut self){
        self.set("AC001;".as_bytes());
    }

    pub fn set_tuner_off(&mut self){
        self.set("AC000;".as_bytes());
    }

    pub fn set_start_tuning(&mut self){
        self.set("AC002;".as_bytes());
    }

    pub fn get_tuner(&mut self) -> u8{
        self.set("AC;".as_bytes());

        let strout = self.get();
        let dout = &strout[4..5];
        dout.parse().unwrap()
    }

//--------- AG: AF GAIN --------------------------------------
    pub fn set_af_gain(&mut self, gain: u8){

    }
    pub fn get_af_gain(&mut self) -> u8 {0}

//--------- AI: AUTO INFORMATION -----------------------------
    pub fn set_ai_on(&mut self){
        self.set("AI1;".as_bytes());
    }

    pub fn set_ai_off(&mut self){
        self.set("AI0;".as_bytes());
    }

    pub fn get_ai(&mut self) -> bool {false}

//--------- AM: VFO-A TO MEMORY CHANNEL ----------------------
    pub fn set_a2m(&mut self){
        self.set("AM;".as_bytes());
    }

//--------- BA: VFO-B TO VFO-A -------------------------------
    pub fn set_b2a(&mut self){
        self.set("BA;".as_bytes());
    }

//--------- BC: AUTO NOTCH -----------------------------------
    pub fn set_auto_notch_on(&mut self){
        self.set("BC01;".as_bytes());
    }
    pub fn set_auto_notch_off(&mut self){
        self.set("BC00;".as_bytes());
    }

    pub fn get_auto_notch(&mut self) -> bool {false}

//--------- BD: BAND DOWN ------------------------------------
    pub fn set_band_down(&mut self){
        self.set("BD0;".as_bytes());
    }

//--------- BI: BREAK IN -------------------------------------
    pub fn set_break_in_on(&mut self) {
        self.set("BI1;".as_bytes());
    }
    pub fn set_break_in_off(&mut self) {
        self.set("BI0;".as_bytes());
    }

    pub fn get_break_in(&mut self) -> bool {false}


//--------- BP: MANUAL NOTCH ---------------------------------
    pub fn set_manual_notch_on(&mut self) {

    }
    pub fn set_manual_notch_off(&mut self) {

    }

    pub fn get_manual_notch(&mut self) -> bool {false}

    pub fn set_manual_notch_level(&mut self, level: u8) {}

    pub fn get_manual_notch_level(&mut self) -> u8 {0}

//--------- BS: BAND SELECT ----------------------------------
    pub fn set_band_160m(&mut self) {
        self.set("BS00;".as_bytes());
    }

    pub fn set_band_80m(&mut self) {
        self.set("BS01;".as_bytes());
    }

    pub fn set_band_60m(&mut self) {
        self.set("BS02;".as_bytes());
    }

    pub fn set_band_40m(&mut self) {
        self.set("BS03;".as_bytes());
    }

    pub fn set_band_30m(&mut self) {
        self.set("BS04;".as_bytes());
    }

    pub fn set_band_20m(&mut self) {
        self.set("BS05;".as_bytes());
    }

    pub fn set_band_17m(&mut self) {
        self.set("BS06;".as_bytes());
    }

    pub fn set_band_15m(&mut self) {
        self.set("BS07;".as_bytes());
    }

    pub fn set_band_12m(&mut self) {
        self.set("BS08;".as_bytes());
    }

    pub fn set_band_10m(&mut self) {
        self.set("BS09;".as_bytes());
    }

    pub fn set_band_6m(&mut self) {
        self.set("BS10;".as_bytes());
    }

    pub fn set_band_GEN(&mut self) {
        self.set("BS11;".as_bytes());
    }

    pub fn set_band_MW(&mut self) {
        self.set("BS12;".as_bytes());
    }

    pub fn set_band_13(&mut self) {
        self.set("BS13;".as_bytes());
    }

    pub fn set_band_AIR(&mut self) {
        self.set("BS14;".as_bytes());
    }

    pub fn set_band_2m(&mut self) {
        self.set("BS15;".as_bytes());
    }

    pub fn set_band_70cm(&mut self) {
        self.set("BS16;".as_bytes());
    }

//--------- BU: BAND UP --------------------------------------
    pub fn set_band_up(&mut self) {}

//--------- BY: BUSY -----------------------------------------

//--------- CH: CHANNEL UP / DOWN ----------------------------

//--------- CN: CTCSS/DCS NUMBER -----------------------------

//--------- CO: CONTOUR --------------------------------------

//--------- CS: CW SPOT --------------------------------------

//--------- CT: CTCSS ----------------------------------------

//--------- DA: DIMMER ---------------------------------------

//--------- DN: DOWN -----------------------------------------

//--------- DT: DATE AND TIME --------------------------------

//--------- ED: ENCODER DOWN ---------------------------------

//--------- EK: ENT KEY --------------------------------------

//--------- EU: ENCODER UP -----------------------------------

//--------- EX: MENU -----------------------------------------

//--------- FA: FREQUENCY VFO A-------------------------------


    pub fn set_freq(&mut self, freq: f64) {
        // Command
        let mut cmd = String::new();
        &cmd.push_str("FA");

        // Calculate frequency into Hertz and format
        let hzfreq = freq * 1000000.0;
        let mut prtfreq = format!("{:09}", hzfreq);
        prtfreq.push_str(";");

        &cmd.push_str(&prtfreq);

        println!("{}", hzfreq);
        let data_2_write: &[u8] = &cmd.as_bytes();

        self.set(data_2_write);
    }

    pub fn get_freq(&mut self) -> f64 {
        // Command
        let mut cmd = String::new();
        &cmd.push_str("FA;");


        let data_2_write: &[u8] = &cmd.as_bytes();
        self.set(data_2_write);


        let strout = self.get();


        let dout = &strout[2..10];
        let f_ret : f64 = dout.parse().unwrap();

        f_ret / 100000.0
    }

//--------- FB: FREQUENCY VFO-B ------------------------------

//--------- FS: FAST STEP ------------------------------------

//--------- FT: FUNCTION TX ----------------------------------

//--------- GT: AGC FUNCTION ---------------------------------

//--------- ID: IDENTIFICATION -------------------------------

//--------- IF: INFORMATION ----------------------------------

//--------- IS: IF-SHIFT -------------------------------------

//--------- KM: KEYER MEMORY ---------------------------------

//--------- KP: KEY PITCH ------------------------------------

//--------- KR: KEYER ----------------------------------------

//--------- KS: KEY SPEED ------------------------------------

//--------- KY: CW KEYING ------------------------------------

//--------- LK: LOCK -----------------------------------------

//--------- LM: LOAD MESSAGE ---------------------------------

//--------- MA: MEMORY CHANNEL TO VFO-A ----------------------

//--------- MC: MEMORY CHANNEL -------------------------------

//--------- MD: MODE -----------------------------------------

//--------- MG: MIC GAIN -------------------------------------

//--------- ML: MONITOR LEVEL --------------------------------

//--------- MR: MEMORY READ ----------------------------------

//--------- MS: METER SW--------------------------------------

//--------- MT: MEMORY CHANNEL WRITE/TAG ---------------------

//--------- MW: MEMORY WRITE ---------------------------------

//--------- MX: MOX SET --------------------------------------

//--------- NA: NARROW ---------------------------------------

//--------- NB: NOISE BLANKER --------------------------------

//--------- NL: NOISE BLANKER LEVEL --------------------------

//--------- NR: NOISE REDUCTION ------------------------------

//--------- OI: OPPOSITE BAND INFORMATION --------------------

//--------- OS: OFFSET (Repeater shift) ----------------------

//--------- PA: PRE-AMP (IPO) --------------------------------

//--------- PB: PLAY BACK ------------------------------------

//--------- PC: POWER CONTROL --------------------------------

//--------- PL: SPEECH PROCESSOR LEVEL -----------------------

//--------- PR: SPEECH PROCESSOR -----------------------------

//--------- PS: POWER SWITCH ---------------------------------

//--------- QI: QMB STORE ------------------------------------

//--------- QR: QMB RECALL -----------------------------------

//--------- QS: QUICK SPLIT ----------------------------------

//--------- RA: RF ATTENUATOR --------------------------------

//--------- RC: CLAR CLEAR -----------------------------------

//--------- RD: CLAR DOWN ------------------------------------

//--------- RG: RF GAIN --------------------------------------

//--------- RI: RADIO INFORMATION ----------------------------

//--------- RL: NOISE REDUCTION LEVEL ------------------------

//--------- RM: READ METER -----------------------------------

//--------- RS: RADIO STATUS ---------------------------------

//--------- RT: CLAR -----------------------------------------

//--------- RU: CLAR UP --------------------------------------

//--------- SC: SCAN -----------------------------------------

//--------- SD: SEMI BREAK-IN DELAY TIME ---------------------

//--------- SH: WIDTH ----------------------------------------

//--------- SM: S METER --------------------------------------

//--------- SQ: SQUELCH LEVEL --------------------------------

//--------- SV: SWAP VFO -------------------------------------

//--------- TS: TXW ------------------------------------------

//--------- TX: TX SET ---------------------------------------

//--------- UL: UNLOCK ---------------------------------------

//--------- UP: UP -------------------------------------------

//--------- VD: VOX DELAY TIME -------------------------------

//--------- VG: VOX GAIN -------------------------------------

//--------- VM: V/M KEY FUNCTION -----------------------------

//--------- VX: VOX ------------------------------------------

//--------- XT: TX CLAR --------------------------------------

//--------- ZI: ZERO IN --------------------------------------

}
