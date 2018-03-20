#[macro_use]
extern crate clap;
extern crate either;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;

extern crate mcu;

pub mod stm32;

use clap::{Arg, App};
use regex::Regex;

use stm32::mcu::MCU;
use stm32::gpio::GPIO;
use stm32::nvic::NVIC;
use stm32::dma::DMA;
use stm32::rcc::RCC;
use stm32::tim::TIM;

use std::fs::File;
use std::fs;
use std::path::Path;

fn open_mcu(path: &Path) {
    let file = File::open(path).unwrap();
    let mcu: MCU = serde_xml_rs::deserialize(file).unwrap();
    let mcu_pegasus = mcu.to_pegasus();

    let file_json = File::create(Path::new("pegasus.json")).unwrap();
    serde_json::to_writer(file_json, &mcu_pegasus).unwrap();
}

fn open_gpio(path: &Path) {
    let file = File::open(path).unwrap();

    let gpio: GPIO = serde_xml_rs::deserialize(file).unwrap();
    //let gpio_pegasus = gpio.to_pegasus();
    //println!("{:?}", gpio_pegasus);
}

fn open_nvic(path: &Path) {
    let file = File::open(path).unwrap();

    let nvic: NVIC = serde_xml_rs::deserialize(file).unwrap();
    //nvic.to_pegasus();
}

fn open_dma(path: &Path) {
    let file = File::open(path).unwrap();

    let dma: DMA = serde_xml_rs::deserialize(file).unwrap();

    //println!("{:?}", dma);
}

fn open_rcc(path: &Path) {
    let file = File::open(path).unwrap();

    let rcc: RCC = serde_xml_rs::deserialize(file).unwrap();

    //println!("{:?}", rcc);
}

fn open_tim(path: &Path) {
    let file = File::open(path).unwrap();

    let tim: TIM = serde_xml_rs::deserialize(file).unwrap();

    //println!("{:?}", tim);
}

fn main() {

    let matches = App::new("converter").author(crate_authors!()).get_matches();
    
    let entries = fs::read_dir("./samples/stm32").unwrap();

    lazy_static! {
        static ref RE :Regex = Regex::new(r"(families)|(^STM32[FL]\d{3})|(UART)|(TIM)|(GPIO)|(NVIC)|(RCC)").unwrap();
    }

    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() {
            let filename =  path.file_name().unwrap().to_str().unwrap();
            println!("Filename: {}", filename);
            let caps = RE.captures(filename);

            for cap in caps {
                // Families
                if let Some(c) = cap.get(1) {

                    println!("Families: {}", c.as_str());
                    }
                // MCU
                else if let Some(c) = cap.get(2) {

                    println!("Parse MCU: {}", c.as_str());
                    open_mcu(path.as_path());
                    }
                // UART
                else if let Some(c) = cap.get(3) {

                    println!("Parse UART: {}", c.as_str());
                    //open_uart(path.as_path());
                    }
                // TIM
                else if let Some(c) = cap.get(4) {

                    println!("Parse TIM: {}", c.as_str());
                    open_tim(path.as_path());
                    }
                // GPIO
                else if let Some(c) = cap.get(5) {

                    println!("Parse GPIO: {}", c.as_str());
                    open_gpio(path.as_path());
                    }
                // NVIC
                else if let Some(c) = cap.get(6) {

                    println!("Parse NVIC: {}", c.as_str());
                    open_nvic(path.as_path());
                    }
                // RCC
                else if let Some(c) = cap.get(7) {

                    println!("Parse RCC: {}", c.as_str());
                    open_rcc(path.as_path());
                    }
                else {

                    }
                }
        }
        /*
        let file = path.unwrap().path().to_str().unwrap();
        //println!("Name: {}", path.unwrap().path().display());
        
        */
    }
}
