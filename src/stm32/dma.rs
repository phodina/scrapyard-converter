#[serde(rename = "IP")]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DMA {
    #[serde(rename = "RefParameter")] RefParams: Vec<RefParameter>,
    #[serde(rename = "RefMode")] RefModes: Vec<RefMode>,
    ModeLogicOperator: ModeLogicOperator,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct RefParameter {
    Name: String,
    Comment: String,
    Type: String,
    #[serde(rename = "PossibleValue")] PossibleValues: Option<Vec<PossibleValue>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct PossibleValue {
    Value: String,
    Comment: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct RefMode {
    Name: String,
    Comment: String,
    BaseMode: String,
    Abstract: Option<String>,
    #[serde(rename = "Parameter")] Params: Vec<Parameter>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Parameter {
    Name: String,
    PossibleValue: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ModeLogicOperator {
    Name: String,
    Mode: Vec<Mode>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Mode {
    Name: String,
    ModeLogicOperator: Option<Box<ModeLogicOperator>>,
    Semaphore: Option<Vec<String>>,
}

impl DMA {
    pub fn to_pegasus(self) {}
}
