//use either::Either;

#[serde(rename = "IP")]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TIM {
    #[serde(rename = "RefParameter")] RefParams: Vec<RefParameter>,
    #[serde(rename = "RefMode")] RefModes: Vec<RefMode>,
    ModeLogicOperator: ModeLogicOperator,
    #[serde(rename = "RefSignal")] RefSignals: Vec<RefSignal>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct RefSignal {
    IOMode: Option<String>,
    Name: String,
    Direction: Option<String>,
    ExclusiveGroupName: Option<String>,
    ShareableGroupName: Option<String>,
    Virtual: Option<String>,
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
    BaseMode: Option<String>,
    HalMode: String,
    Group: Option<String>,
    // FIXME: Load these cumbersome values
    //#[serde(rename = "ConfigForMode")] Configs: Either<Option<String>, Option<Vec<String>>>,
    //#[serde(rename = "Parameter")] Params: Option<Vec<Parameter>>,
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
    UserName: Option<String>,
    ModeLogicOperator: Option<Box<ModeLogicOperator>>,
    SignalLogicalOp: Option<SignalLogicalOp>,
    Semaphore: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct SignalLogicalOp {
    Name: String,
    #[serde(rename = "Signal")] Signals: Option<Vec<Signal>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Signal {
    Name: String,
}

impl TIM {
    pub fn to_pegasus(self) {}
}
