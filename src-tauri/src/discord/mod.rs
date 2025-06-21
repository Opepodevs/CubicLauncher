use communicator::{Activity, Assets, DiscordRpcClient, Timestamps};
use once_cell::sync::Lazy;
use std::time::SystemTime;

static DISCORD_RPC: Lazy<DiscordRpcClient> =
    Lazy::new(|| DiscordRpcClient::new("1305247641252397059"));

pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
    let activity = Activity {
        details: Some("no se que poner xd".to_string()),
        state: Some("Coding".to_string()),
        timestamps: Some(Timestamps {
            start: Some(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs(),
            ),
            end: None,
        }),
        assets: Some(Assets {
            large_image: Some("logo".to_string()),
            large_text: Some("CubicLauncher".to_string()),
            small_image: Some("rust".to_string()),
            small_text: Some("Rust Programming Language".to_string()),
        }),
        party: None,
        secrets: None,
        instance: Some(false),
    };

    // Establecer actividad
    DISCORD_RPC.set_activity(activity).await?;
    Ok(())
}
