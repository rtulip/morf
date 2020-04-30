extern crate ron;
extern crate serde;
extern crate shared;
use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy, prelude::*, utils::application_root_dir,
};
use shared::networking;
use std::time::Duration;

#[derive(Default)]
struct ClientGameModel;

impl SimpleState for ClientGameModel {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
use log::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");

    if let Some(asset_dir_str) = assets_dir.to_str() {
        info!("asset dir: {}", asset_dir_str)
    }

    if let Some(config_dir_str) = config_dir.to_str() {
        info!("config dir: {}", config_dir_str)
    }

    let game_data =
        GameDataBuilder::default().with(networking::TcpConnectorSystem, "TcpConnector", &[]);

    let mut game = Application::build(assets_dir, ClientGameModel::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            1,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
