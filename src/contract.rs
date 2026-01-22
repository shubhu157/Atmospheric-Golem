use rialo_std::{contract, http, Collections, Address};
use serde::{Deserialize, Serialize};

// Define the Golem structure
struct Golem {
    id: u64,
    owner: Address,
    city: String,
    current_form: String,
}

// Data structure for the Weather API response
#[derive(Deserialize)]
struct WeatherData {
    temp: f32,
    condition: String,
}

#[contract]
impl AtmosphericGolems {

    // Main function to update the Golem based on weather
    pub fn update_golem_form(&mut self, golem_id: u64) {
        
        let golem = self.golems.get(golem_id);
        
        // 1. Fetch weather data using Rialo Native HTTP (No Oracle needed!)
        let url = format!("https://api.weather.com/v1/location/{}", golem.city);
        let response = http::get(url).await;
        let weather: WeatherData = serde_json::from_str(&response.body).unwrap();

        // 2. Determine new form based on logic
        let new_form = if weather.temp < 0.0 {
            "ICE_GOLEM".to_string()
        } else if weather.condition == "Rain" {
            "MOSS_GOLEM".to_string()
        } else {
            "STONE_GOLEM".to_string()
        };

        // 3. Save the change on-chain
        self.golems.update(golem_id, |g| {
            g.current_form = new_form;
        });
        
        print!("Golem {} transformed into {}", golem_id, new_form);
    }
}
