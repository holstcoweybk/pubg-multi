pub mod pubg_multi {
    pub struct PUBGMulti {
        pub aimbot_enabled: bool,
        pub esp_enabled: bool,
        pub speed_hack_enabled: bool,
    }

    impl PUBGMulti {
        pub fn new() -> Self {
            PUBGMulti {
                aimbot_enabled: false,
                esp_enabled: false,
                speed_hack_enabled: false,
            }
        }

        pub fn toggle_aimbot(&mut self) {
            self.aimbot_enabled = !self.aimbot_enabled;
        }

        pub fn toggle_esp(&mut self) {
            self.esp_enabled = !self.esp_enabled;
        }

        pub fn toggle_speed_hack(&mut self) {
            self.speed_hack_enabled = !self.speed_hack_enabled;
        }
    }
}