extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use std::fs::File;
use std::fs;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct IP {
    ClockEnableMode: Option<String>,
    InstanceName: String,
    Name: String,
    Version: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Pin {
    Name: String,
    Position: String,
    Type: String,
    #[serde(rename = "Signal")] Signals: Option<Vec<Signal>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Signal {
    Name: Option<String>,
    IOModes: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Mcu {
    ClockTree: String,
    Family: String,
    Line: String,
    Package: String,
    RefName: String,
    Core: String,
    Frequency: i32,
    Ram: i32,
    IONb: i32,
    Die: String,
    Flash: i32,
    #[serde(rename = "IP")] IPs: Vec<IP>,
    Pin: Vec<Pin>,
}

fn main() {
    let file = File::open("samples/stm32/STM32F030C6Tx.xml").unwrap();
    let mcu: Mcu = serde_xml_rs::deserialize(file).unwrap();

    println!("MCU: {} Package: {}", mcu.RefName, mcu.Package);
    /*
    let paths = fs::read_dir("./samples/stm32").unwrap();

    
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }*/
}
