use serenity::{
    all::{EventHandler, GatewayIntents},
    async_trait, Client,
};

struct Application {
    database_manager: DatabaseManager,
    bot_manager: BotManager,
}

struct DatabaseManager;

struct BotManager {
    client: Client,
}

#[async_trait]
impl EventHandler for BotManager {}

impl Application {
    pub async fn run(token: String) -> Self {
        println!("Running application");

        use GatewayIntents as GI;
        let intents = GI::all();
        let client 
        Self {
            database_manager: DatabaseManager,
            bot_manager: BotManager,
        }
    }
}
