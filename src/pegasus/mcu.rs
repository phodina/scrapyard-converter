use mcu::mcu::Memory;
use mcu::package::Package;

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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Pin {
    pub name: String,
    // TODO: Change to Position and implement parsing through regex
    pub position: String,
    pub type_t: String,
}