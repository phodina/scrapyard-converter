extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;

extern crate mcu;

pub mod stm32;
pub mod pegasus;

use stm32::mcu::MCU;
use stm32::gpio::GPIO;

use std::fs::File;
use std::fs;
use std::path::Path;

fn open_mcu() {
    let file = File::open("samples/stm32/STM32F030C6Tx.xml").unwrap();
    let mcu: MCU = serde_xml_rs::deserialize(file).unwrap();
    let mcu_pegasus = mcu.to_pegasus();

    let file_json = File::create(Path::new("pegasus.json")).unwrap();
    serde_json::to_writer(file_json, &mcu_pegasus).unwrap();
}
fn main() {
    let file = File::open("samples/stm32/GPIO-STM32F031_gpio_v1_0_Modes.xml").unwrap();

    let gpio: GPIO = serde_xml_rs::deserialize(file).unwrap();
    let gpio_pegasus = gpio.to_pegasus();
    println!("{:?}", gpio_pegasus);
    /*
    let paths = fs::read_dir("./samples/stm32").unwrap();

    
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }*/
}
