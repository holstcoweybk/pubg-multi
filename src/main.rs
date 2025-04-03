use std::sync::{Arc, Mutex};
use tokio::task;
use reqwest::Client;
use rand::Rng;

struct PUBGMulti {
    aimbot_enabled: bool,
    esp_enabled: bool,
    speed_hack_enabled: bool,
}

impl PUBGMulti {
    fn new() -> Self {
        PUBGMulti {
            aimbot_enabled: false,
            esp_enabled: false,
            speed_hack_enabled: false,
        }
    }

    fn toggle_aimbot(&mut self) {
        self.aimbot_enabled = !self.aimbot_enabled;
    }

    fn toggle_esp(&mut self) {
        self.esp_enabled = !self.esp_enabled;
    }

    fn toggle_speed_hack(&mut self) {
        self.speed_hack_enabled = !self.speed_hack_enabled;
    }

    async fn fetch_game_data(&self) -> Result<String, reqwest::Error> {
        let client = Client::new();
        let response = client.get("http://game.api/data").send().await?;
        let data = response.text().await?;
        Ok(data)
    }

    fn randomize_settings(&mut self) {
        let mut rng = rand::thread_rng();
        self.aimbot_enabled = rng.gen_bool(0.5);
        self.esp_enabled = rng.gen_bool(0.5);
        self.speed_hack_enabled = rng.gen_bool(0.5);
    }
}

#[tokio::main]
async fn main() {
    let pubg_multi = Arc::new(Mutex::new(PUBGMulti::new()));
    let pubg_multi_clone = Arc::clone(&pubg_multi);

    task::spawn(async move {
        let data = pubg_multi_clone.lock().unwrap().fetch_game_data().await;
        match data {
            Ok(game_data) => println!("Game Data: {}", game_data),
            Err(e) => eprintln!("Error fetching game data: {}", e),
        }
    });

    loop {
        let mut instance = pubg_multi.lock().unwrap();
        instance.randomize_settings();
        println!("Aimbot: {}, ESP: {}, Speed Hack: {}", instance.aimbot_enabled, instance.esp_enabled, instance.speed_hack_enabled);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}