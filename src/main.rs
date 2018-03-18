extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate mcu;

pub mod stm32;
pub mod pegasus;

use stm32::mcu::MCU;

use std::fs::File;
use std::fs;

fn main() {
    let file = File::open("samples/stm32/STM32F030C6Tx.xml").unwrap();
    let mcu: MCU = serde_xml_rs::deserialize(file).unwrap();
    let mcu_pegasus = mcu.to_pegasus();

    println!("{:?}", mcu_pegasus);
    /*
    let paths = fs::read_dir("./samples/stm32").unwrap();

    
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }*/
}
