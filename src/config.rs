use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct CommandOption {
    pub number: usize,
    pub display_name: String,
    pub command: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub commands: Vec<CommandOption>,
}

pub fn load_config(path: &str) -> Config {
    let config_data = std::fs::read_to_string(path).expect("Unable to read config file");
    serde_json::from_str(&config_data).expect("Unable to parse config file")
}

pub fn save_config(path: &str, config: &Config) {
    let config_data = serde_json::to_string_pretty(config).expect("Failed to serialize config");
    let mut file = std::fs::File::create(path).expect("Unable to create config file");
    std::io::Write::write_all(&mut file, config_data.as_bytes()).expect("Unable to write to config file");
    println!("Config saved.");
}

pub fn create_default_config(path: &str) {
    let default_commands = vec![
        CommandOption {
            number: 1,
            display_name: "Clear 📥 Inbox".to_string(),
            command: "tod list process -f '@Morn & (due today | overdue | no due date)'".to_string(),
        },
        CommandOption {
            number: 2,
            display_name: "📆 Daily & Weekly Review".to_string(),
            command: "tod list process -f '#GTD Weekly Review 📆 & (overdue | due today | no due date) & !@EOD or #Daily Review 📆 & (overdue | due today | no due date)  & !@EOD'".to_string(),
        },
        // Add more commands as needed...
    ];

    let default_config = Config {
        commands: default_commands,
    };

    save_config(path, &default_config);
}