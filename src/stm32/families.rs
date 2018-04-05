#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Families {
    #[serde(rename = "Family")] families: Vec<Family>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Family {
    #[serde(rename = "SubFamily")] subfamilies: Vec<SubFamily>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct SubFamily {
    #[serde(rename = "MCU")] mcus: Option<Vec<MCU>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct MCU {
    Name: String,
    PackageName: String,
    RefName: String,
    RPN: String,
    Core: Option<String>,
    Frequency: Option<String>,
    Ram: Option<String>,
    IONb: Option<String>,
    Flash: Option<String>,
    Current: Option<Current>,
    Voltage: Option<Voltage>,
    Temperature: Option<Temperature>,
    Peripheral: Option<Peripheral>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Voltage {
    Max: String,
    Min: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Current {
    Lowest: String,
    Run: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Temperature {
    Max: String,
    Min: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Peripheral {
    Type: String,
    MaxOccurs: String,
}

use errors::*;
use Export;

impl Export<()> for Families {
    fn export(self) -> Result<()> {
        Ok(())
    }
}
