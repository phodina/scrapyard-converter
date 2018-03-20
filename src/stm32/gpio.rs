#[serde(rename = "IP")]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct GPIO {
    #[serde(rename = "RefParameter")] RefParams: Vec<RefParameter>,
    #[serde(rename = "RefMode")] RefModes: Vec<RefMode>,
    #[serde(rename = "GPIO_Pin")] Pins: Vec<GPIO_Pin>,
    // TODO: Implement GPIO_Port
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct RefMode {
    Name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct RefParameter {
    Name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct GPIO_Pin {
    PortName: String,
    Name: String,
    #[serde(rename = "SpecificParameter")] SpecificParams: Option<Vec<SpecificParameter>>,
    #[serde(rename = "PinSignal")] PinSignals: Option<Vec<PinSignal>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct PinSignal {
    Name: String,
    #[serde(rename = "SpecificParameter")] SpecificParam: SpecificParameter,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct SpecificParameter {
    Name: String,
    PossibleValue: String,
}

impl GPIO {

    pub fn to_pegasus(self) {}
    /*
    pub fn to_pegasus(self) -> ::pegasus::gpio::GPIO {
        let mut pins: Vec<::pegasus::gpio::Pin> = Vec::new();
        for pin in self.Pins {
            let mut signals_p: Vec<::pegasus::gpio::Signal> = Vec::new();

            match pin.PinSignals {
                Some(signals) => for signal in signals {
                    let s = ::pegasus::gpio::Signal {
                        name: signal.Name,
                        value: signal.SpecificParam.PossibleValue,
                    };
                    signals_p.push(s);
                },
                None => (),
            }
            let p = ::pegasus::gpio::Pin {
                name: pin.Name,
                signals: signals_p,
            };
            pins.push(p);
        }

        ::pegasus::gpio::GPIO { pins: pins }
    }*/
}
