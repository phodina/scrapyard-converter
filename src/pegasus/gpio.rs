#[derive(Serialize, Deserialize, Debug)]
pub struct GPIO {
    pub pins: Vec<Pin>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Pin {
    pub name: String,
    pub signals: Vec<Signal>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signal {
    pub name: String,
    pub value: String,
}
