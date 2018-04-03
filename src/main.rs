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

use clap::{App, Arg};
use regex::Regex;

use std::error;
use std::fmt;

use serde::Deserialize;

use stm32::mcu::MCU;
use stm32::gpio::GPIO;
use stm32::nvic::NVIC;
use stm32::dma::DMA;
use stm32::rcc::RCC;
use stm32::tim::TIM;

use std::io;
use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum ConverterError {
    Io(io::Error),
    SerdeXML(serde_xml_rs::Error),
    SerdeJSON(serde_json::Error)
}

impl fmt::Display for ConverterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            ConverterError::Io(ref err) => write!(f, "IO error: {}", err),
            ConverterError::SerdeXML(ref err) => write!(f, "Serde error: {}", err),
            ConverterError::SerdeJSON(ref err) => write!(f, "Serde error: {}", err),
        }
    }
}

impl error::Error for ConverterError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            ConverterError::Io(ref err) => err.description(),
            ConverterError::SerdeXML(ref err) => err.description(),
            ConverterError::SerdeJSON(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            ConverterError::Io(ref err) => Some(err),
            ConverterError::SerdeXML(ref err) => Some(err),
            ConverterError::SerdeJSON(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for ConverterError {
    fn from(err: io::Error) -> ConverterError {
        ConverterError::Io(err)
    }
}

impl From<serde_xml_rs::Error> for ConverterError {
    fn from(err: serde_xml_rs::Error) -> ConverterError {
        ConverterError::SerdeXML(err)
    }
}

impl From<serde_json::Error> for ConverterError {
    fn from(err: serde_json::Error) -> ConverterError {
        ConverterError::SerdeJSON(err)
    }
}

fn open_mcu(path: &Path, output_dir: &str) -> Result<(), ConverterError>{
    let file = File::open(path)?;
    let mcu: MCU = serde_xml_rs::deserialize(file)?;
    let mcu_pegasus = mcu.to_pegasus();

    let mut filename = PathBuf::new();
    filename.push(".");
    filename.push(output_dir);
    filename.set_file_name(path.file_stem().unwrap());
    filename.set_extension("json");

    let file_json = File::create(Path::new(&filename))?;
    serde_json::to_writer(file_json, &mcu_pegasus)?;

    Ok(())
}

fn open_cfg<'a, T: Deserialize<'a>>(path: &Path) -> Result<(), ConverterError> {
    let file = File::open(path)?;

    let cfg: T = serde_xml_rs::deserialize(file)?;

    //println!("{:?}", tim);
    Ok(())
}

fn run(input_dir: &str, output_dir: &str) -> Result<(), ConverterError> {

    let entries = fs::read_dir(input_dir).unwrap();

    lazy_static! {
        static ref RE :Regex = Regex::new(r"(families)|(^STM32[FL]\d{3})|(UART)|(TIM)|(GPIO)|(NVIC)|(RCC)").unwrap();
    }

    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            println!("Filename: {}[{}]", filename, path.to_str().unwrap());
            let caps = RE.captures(filename);

            for cap in caps {
                // Families
                if let Some(c) = cap.get(1) {
                    println!("Families: {}", c.as_str());
                }
                // MCU
                else if let Some(c) = cap.get(2) {
                    println!("Parse MCU: {}", c.as_str());
                    open_mcu(path.as_path(), output_dir)?;
                }
                // UART
                else if let Some(c) = cap.get(3) {
                    println!("Parse UART: {}", c.as_str());
                    //open_cfg::<UART>(path.as_path());
                }
                // TIM
                else if let Some(c) = cap.get(4) {
                    println!("Parse TIM: {}", c.as_str());
                    open_cfg::<TIM>(path.as_path())?;
                }
                // GPIO
                else if let Some(c) = cap.get(5) {
                    println!("Parse GPIO: {}", c.as_str());
                    open_cfg::<GPIO>(path.as_path())?;
                }
                // NVIC
                else if let Some(c) = cap.get(6) {
                    println!("Parse NVIC: {}", c.as_str());
                    open_cfg::<NVIC>(path.as_path())?;
                }
                // RCC
                else if let Some(c) = cap.get(7) {
                    println!("Parse RCC: {}", c.as_str());
                    open_cfg::<RCC>(path.as_path())?;
                } else {

                }
            }
        }
    }

    Ok(())
}

fn main() {
    let matches = App::new("converter")
        .author(crate_authors!())
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("IDir")
                .help("Input directory with configuration to parse")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("ODir")
                .help("Output directory with parsed configuration ")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let input_dir = matches.value_of("input").unwrap();
    let output_dir = matches.value_of("output").unwrap();

    if let Err(e) = run(input_dir, output_dir) {
        println!("Error: {}", e);
    }
}
