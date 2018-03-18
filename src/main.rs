extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate mcu;

use mcu::mcu::Memory;
use mcu::MCUBuilder;
use mcu::package::Package;
use mcu::pin::Position;
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

// TODO: Check for Eeprom
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

impl Mcu {
    pub fn to_json(self) -> json::MCU {
        let flash = Memory::Flash {
            start: 0x08000000,
            size: self.Flash as u32,
        };
        let ram = Memory::Ram {
            start: 0x20000000,
            size: self.Ram as u32,
        };

        let mut memories = Vec::new();
        memories.push(flash);
        memories.push(ram);

        let package = Package::new(&self.Package);

        let mut ips: Vec<json::IP> = Vec::new();

        for ip in self.IPs {
            ips.push(json::IP::from(ip));
        }

        let mut pins: Vec<json::Pin> = Vec::new();

        for pin in self.Pin {
            pins.push(json::Pin::from(pin));
        }

        json::MCU {
            memory: memories,
            frequency: self.Frequency as u32,
            core: self.Core,
            name: self.RefName,
            package: package,
            ips: ips,
            pins: pins,
        }
    }
}

mod json {

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MCU {
        pub memory: Vec<Memory>,
        pub frequency: u32,
        // TODO: Change to enum
        pub core: String,
        pub name: String,
        pub package: Package,
        pub ips: Vec<IP>,
        pub pins: Vec<Pin>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    struct MCURoot {
        Mcu: MCU,
    }

    pub struct MCUBuilder {
        mcu: MCURoot,
        //pins: Option<PinsBuilder>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct IP {
        pub config_file: String,
        pub name: String,
    }

    impl From<super::IP> for IP {
        fn from(ip: super::IP) -> Self {
            IP {
                config_file: ip.Version,
                name: ip.Name,
            }
        }
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Pin {
        pub name: String,
        // TODO: Change to Position and implement parsing through regex
        pub position: String,
        pub type_t: String,
    }

    impl From<super::Pin> for Pin {
        fn from(pin: super::Pin) -> Self {
            Pin {
                name: pin.Name,
                position: pin.Position,
                type_t: pin.Type,
            }
        }
    }
}

fn main() {
    let file = File::open("samples/stm32/STM32F030C6Tx.xml").unwrap();
    let mcu: Mcu = serde_xml_rs::deserialize(file).unwrap();
    let mcu_json = mcu.to_json();

    println!("{:?}", mcu_json);
    /*
    let paths = fs::read_dir("./samples/stm32").unwrap();

    
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }*/
}
