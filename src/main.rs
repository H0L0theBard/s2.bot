use serenity::async_trait;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Activity;
use serenity::model::gateway::Ready;
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;
use std::env;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
        let guilds = ctx.cache.guilds().len();

        println!("The bot is in {} guilds", guilds);
        set_activity(ctx).await;
    }
    async fn guild_member_addition(
        &self,
        ctx: Context,
        new_member: serenity::model::guild::Member,
    ) {
        new_member
            .user
            .direct_message(ctx, |m| {
                m.content("https://northstar.thunderstore.io/package/S2Mods/GlitchOverhaul/")
            })
            .await
            .unwrap();
    }
}

async fn set_activity(ctx: Context) {
    let activity = Activity::playing("on mp_glitch");
    let status = OnlineStatus::Online;
    ctx.set_presence(Some(activity), status).await;
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new();
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::default() | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
