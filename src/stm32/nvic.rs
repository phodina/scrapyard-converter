use mcu::irqs::{IRQ, IRQS};

#[serde(rename = "IP")]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct NVIC {
    #[serde(rename = "RefParameter")] RefParams: Vec<RefParameter>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct RefParameter {
    Name: String,
    PossibleValue: Option<Vec<PossibleValue>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct PossibleValue {
    Comment: String,
    Value: String,
}

use errors::*;
use Export;

impl Export<IRQS> for NVIC {
    fn export(self) -> Result<IRQS> {
        let mut irqs = Vec::new();

        for param in self.RefParams.into_iter() {
            if param.Name == "IRQn" {
                match param.PossibleValue {
                    Some(value) => for v in value.into_iter() {
                        println!("{} |||  {}", v.Comment, v.Value);
                        irqs.push(IRQ {
                            Desc: v.Comment,
                            Value: v.Value,
                        });
                    },
                    None => (),
                }
            }
        }

        Ok(IRQS { irqs: irqs })
    }
}
