#[macro_use]
extern crate clap;
extern crate either;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate mcu;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;

use errors::*;

mod errors;
pub mod stm32;

use clap::{App, Arg};
use regex::Regex;

use serde::{Deserialize, Serialize};

use stm32::families::Families;
use stm32::mcu::MCU;
use stm32::gpio::GPIO;
use stm32::nvic::NVIC;
use stm32::rcc::RCC;
use stm32::tim::TIM;

use mcu::irqs::IRQS;

use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};

pub trait Export<T> {
    fn export(self) -> Result<T>;
}

fn open_cfg<'a, T: Deserialize<'a> + Export<E>, E: Serialize>(
    path: &Path,
    output_dir: &str,
) -> Result<()> {
    let file = File::open(path)?;

    let cfg: T = serde_xml_rs::deserialize(file)?;

    let cfg_export = cfg.export()?;

    let mut filename = PathBuf::new();
    filename.push(".");
    filename.push(output_dir);
    let filename_str = path.file_stem().ok_or("")?;
    filename.set_file_name(filename_str);
    filename.set_extension("json");

    let file_json = File::create(Path::new(&filename))?;
    serde_json::to_writer(file_json, &cfg_export).unwrap();

    Ok(())
}

fn run(input_dir: &str, output_dir: &str) -> Result<()> {
    let entries = fs::read_dir(input_dir)?;

    lazy_static! {
        static ref RE :Regex = Regex::new(r"(families)|(^STM32[FL]\d{3})|(UART)|(TIM)|(GPIO)|(NVIC)|(RCC)").unwrap();
    }

    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().ok_or("")?;
            let path_str = path.to_str().ok_or("")?;
            println!("Filename: {}[{}]", filename, path_str);
            let caps = RE.captures(filename);

            for cap in caps {
                // Families
                if let Some(c) = cap.get(1) {
                    println!("Families: {}", c.as_str());
                    open_cfg::<Families, ()>(path.as_path(), output_dir)?;
                }
                // MCU
                else if let Some(c) = cap.get(2) {
                    println!("Parse MCU: {}", c.as_str());
                    open_cfg::<MCU, ::mcu::mcu::MCU>(path.as_path(), output_dir)?;
                }
                // UART
                else if let Some(c) = cap.get(3) {
                    println!("Parse UART: {}", c.as_str());
                //open_cfg::<UART>(path.as_path());
                }
                // TIM
                else if let Some(c) = cap.get(4) {
                    println!("Parse TIM: {}", c.as_str());
                    open_cfg::<TIM, ()>(path.as_path(), output_dir)?;
                }
                // GPIO
                else if let Some(c) = cap.get(5) {
                    println!("Parse GPIO: {}", c.as_str());
                    open_cfg::<GPIO, ()>(path.as_path(), output_dir)?;
                }
                // NVIC
                else if let Some(c) = cap.get(6) {
                    println!("Parse NVIC: {}", c.as_str());
                    open_cfg::<NVIC, IRQS>(path.as_path(), output_dir)?;
                }
                // RCC
                else if let Some(c) = cap.get(7) {
                    println!("Parse RCC: {}", c.as_str());
                    open_cfg::<RCC, ()>(path.as_path(), output_dir)?;
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
        match e.kind() {
            &ErrorKind::Msg(ref s) => println!("Msg: {}", s),
            &ErrorKind::Io(ref s) => println!("Io: {}", s),
            &ErrorKind::SerdeXML(ref s) => println!("Serde XML: {}", s),
            &ErrorKind::SerdeJSON(ref s) => println!("Serde JSON: {}", s),
            &ErrorKind::Parse(ref s) => println!("Parse: {}", s),
            _ => panic!("Unknown error"),
        }

        std::process::exit(1);
    }
}
