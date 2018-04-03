use mcu::memory::Memory;
use mcu::package::Package;
use mcu::pin::Position;
use mcu::pin::PinBuilder;
use mcu::mcu::{ARMCore, Core, Frequency, Platform};

use regex::Regex;

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
    Ram: Vec<i32>,
    IONb: i32,
    Die: String,
    Flash: Vec<i32>,
    #[serde(rename = "IP")] IPs: Vec<IP>,
    #[serde(rename = "Pin")] Pins: Vec<Pin>,
}

// TODO: Refactor into methods
impl MCU {
    pub fn to_pegasus(self) -> ::mcu::mcu::MCU {
        let mut memories = Vec::new();

        for flash in self.Flash {
            let flash = Memory::Flash {
                start: 0x08000000,
                size: flash as u32 * 1024,
            };
            memories.push(flash);
        }

        for ram in self.Ram {
            let ram = Memory::Ram {
                start: 0x20000000,
                size: ram as u32 * 1024,
            };

            memories.push(ram);
        }

        // TODO: Handle sign & overflow
        let frequency = Frequency::MHz(self.Frequency as u16);

        // TODO: Handle unknown core
        let core = match self.Core.as_ref() {
            "ARM Cortex-M0" => Core::ARM(ARMCore::CortexM0),
            "ARM Cortex-M3" => Core::ARM(ARMCore::CortexM3),
            "ARM Cortex-M4" => Core::ARM(ARMCore::CortexM4),
            "ARM Cortex-M7" => Core::ARM(ARMCore::CortexM7),
            _ => Core::AVR,
        };

        let package = Package::new(&self.Package);

        let mut ips: Vec<::mcu::mcu::IP> = Vec::with_capacity(self.IPs.len());

        for ip in self.IPs {
            ips.push(::mcu::mcu::IP {
                name: ip.Name,
                config_file: ip.Version,
            });
        }

        let mut pins: Vec<::mcu::pin::Pin> = Vec::with_capacity(self.Pins.len());

        for pin in self.Pins {
            let pos = if package.is_grid() {
                lazy_static! {
                    static ref RE :Regex = Regex::new(r"([[:alpha:]])(\d*)").unwrap();
                }

                let caps = RE.captures(&pin.Position).unwrap();

                let count = caps.get(2).unwrap().as_str().parse::<u16>().unwrap();

                match caps.get(1).unwrap().as_str() {
                    "A" => Position::Grid(0, count as u8),
                    "B" => Position::Grid(1, count as u8),
                    "C" => Position::Grid(2, count as u8),
                    "D" => Position::Grid(3, count as u8),
                    "E" => Position::Grid(4, count as u8),
                    "F" => Position::Grid(5, count as u8),
                    "G" => Position::Grid(6, count as u8),
                    "H" => Position::Grid(7, count as u8),
                    "J" => Position::Grid(8, count as u8),
                    "K" => Position::Grid(9, count as u8),
                    "L" => Position::Grid(10, count as u8),
                    "M" => Position::Grid(11, count as u8),
                    "N" => Position::Grid(12, count as u8),
                    "P" => Position::Grid(13, count as u8),
                    "R" => Position::Grid(14, count as u8),
                    "S" => Position::Grid(15, count as u8),
                    "T" => Position::Grid(16, count as u8),
                    _ => Position::Grid(0, 0),
                }
            } else {
                Position::Linear(pin.Position.parse::<u16>().unwrap())
            };

            let mut pin2store = PinBuilder::new(&pin.Type, pos, &pin.Name);

            if let Some(signals) = pin.Signals {
                let mut sigs: Vec<String> = Vec::new();
                for sig in signals {
                    if let Some(name) = sig.Name {
                        sigs.push(name);
                    };
                }

                pin2store = pin2store.signals(sigs);
            }

            let p = pin2store.finish();
            pins.push(p);
        }

        ::mcu::mcu::MCU {
            memory: memories,
            frequency: frequency,
            core: core,
            name: self.RefName,
            platform: Platform::STM32 {
                family: self.Family,
                line: self.Line,
            },
            package: package,
            ips: ips,
            pins: pins,
        }
    }
}
