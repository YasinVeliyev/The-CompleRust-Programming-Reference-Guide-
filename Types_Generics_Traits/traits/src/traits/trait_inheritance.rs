#![allow(dead_code)]
trait Vehicle {
    fn get_price(&self) -> u64;
}

trait Car: Vehicle {
    fn model(&self) -> String;
}

struct TeslaRoadster {
    model: String,
    release_date: u16,
}

impl TeslaRoadster {
    fn new(model: &str, release_date: u16) -> Self {
        Self {
            model: model.to_string(),
            release_date,
        }
    }
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        self.model.to_string()
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        200_000
    }
}

pub fn trait_inheritance() {
    let my_roadster = TeslaRoadster::new("Tesla Roadster II", 2020);
    println!(
        "{} is priced at ${}",
        my_roadster.model,
        my_roadster.get_price()
    )
}
