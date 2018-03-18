use mcu::mcu::Memory;
use mcu::package::Package;

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
#[serde(rename = "Mcu")]
#[derive(Debug, Deserialize)]
pub struct MCU {
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

impl MCU {
    pub fn to_pegasus(self) -> ::pegasus::mcu::MCU {
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

        let mut ips: Vec<::pegasus::mcu::IP> = Vec::new();

        for ip in self.IPs {
            ips.push(::pegasus::mcu::IP::from(ip));
        }

        let mut pins: Vec<::pegasus::mcu::Pin> = Vec::new();

        for pin in self.Pin {
            pins.push(::pegasus::mcu::Pin::from(pin));
        }

        ::pegasus::mcu::MCU {
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

impl From<IP> for ::pegasus::mcu::IP {
    fn from(ip: IP) -> Self {
        ::pegasus::mcu::IP {
            config_file: ip.Version,
            name: ip.Name,
        }
    }
}

impl From<Pin> for ::pegasus::mcu::Pin {
    fn from(pin: Pin) -> Self {
        ::pegasus::mcu::Pin {
            name: pin.Name,
            position: pin.Position,
            type_t: pin.Type,
        }
    }
}
