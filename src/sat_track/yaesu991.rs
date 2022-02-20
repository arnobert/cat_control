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

//--------- SET FREQUENCY VFO A / B ------------------------
    pub fn set_freq(&mut self, freq: f64, vfoB: bool) {
        // Command
        let mut cmd = String::new();

        if vfoB {
            &cmd.push_str("FB");
        }
        else {
            &cmd.push_str("FA");
        }
    // Calculate frequency into Hertz and format
        let hzfreq = freq * 1000000.0;
        let mut prtfreq = format!("{:09}", hzfreq);
        prtfreq.push_str(";");

        &cmd.push_str(&prtfreq);

        println!("{}", hzfreq);
        let data_2_write: &[u8] = &cmd.as_bytes();

        self.set(data_2_write);
    }

    pub fn get_freq(&mut self, vfoB: bool) -> f64 {
        // Command
        let mut cmd = String::new();

        if vfoB {
            &cmd.push_str("FB;");
        }
        else {
            &cmd.push_str("FA;");
        }
        let data_2_write: &[u8] = &cmd.as_bytes();
        self.set(data_2_write);

        let strout = self.get();
        let dout = &strout[2..10];
        let f_ret : f64 = dout.parse().unwrap();

        f_ret / 100000.0
    }
}
