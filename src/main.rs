use std::fs;
use std::io::Read;
use std::str::FromStr;

const CHARGE_NOW: &'static str = "/sys/class/power_supply/BAT0/charge_now";
const ENERGY_NOW: &'static str = "/sys/class/power_supply/BAT0/energy_now";
const CHARGE_FULL: &'static str = "/sys/class/power_supply/BAT0/charge_full";
const ENERGY_FULL: &'static str = "/sys/class/power_supply/BAT0/energy_full";

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

fn read_file(charge: &str, energy: &str) -> Result<u64, Error> {
    let mut buf = [0; 8];

    let mut file = fs::File::open(charge);
    if file.is_err() {
        file = fs::File::open(energy);
    }

    let mut file = file?;
    file.read(&mut buf[..])?;
    let num = u64::from_ne_bytes(buf);
    Ok(num)
}

fn main() {
    let now = read_file(CHARGE_NOW, ENERGY_NOW).unwrap();
    let full = read_file(CHARGE_FULL, ENERGY_FULL).unwrap();

    println!("{}%", now/(full/100));
}
