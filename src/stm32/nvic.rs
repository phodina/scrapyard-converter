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

impl NVIC {
    pub fn to_pegasus(self) {
        for param in self.RefParams {
            if param.Name == "IRQn" {
                match param.PossibleValue {
                    Some(ref value) => for v in value {
                        println!("{} |||  {}", v.Comment, v.Value);
                    },
                    None => (),
                }
            }
        }
    }
}
