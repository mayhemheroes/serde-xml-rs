#![no_main]
use libfuzzer_sys::fuzz_target;
use std::str;

use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

// From one of the examples

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PlateAppearance {
    #[serde(rename = "$value")]
    events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Event {
    Pitch(Pitch),
    Runner(Runner),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Pitch {
    speed: u32,
    r#type: PitchType,
    outcome: PitchOutcome,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum PitchType {
    FourSeam,
    TwoSeam,
    Changeup,
    Cutter,
    Curve,
    Slider,
    Knuckle,
    Pitchout,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum PitchOutcome {
    Ball,
    Strike,
    Hit,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Runner {
    from: Base,
    to: Option<Base>,
    outcome: RunnerOutcome,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Base {
    First,
    Second,
    Third,
    Home,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum RunnerOutcome {
    Steal,
    Caught,
    PickOff,
}

fuzz_target!(|data: &[u8]| {
    match str::from_utf8(data) {
        Ok(in_string) => match from_str::<PlateAppearance>(in_string) {
            Ok(itm) => {
                to_string(&itm);
            }
            Err(..) => (),
        },
        Err(..) => (),
    }
});
