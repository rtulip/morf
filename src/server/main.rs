extern crate serde;
extern crate ron;
extern crate shared;
use amethyst::{
    core::{
        frame_limiter::FrameRateLimitStrategy,
    },
    prelude::*,
    utils::application_root_dir,
};
use std::time::Duration;
use std::net::TcpListener;

#[derive(Default)]
struct ServerGameModel;

impl SimpleState for ServerGameModel {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        
    }
}
use log::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let listener = TcpListener::bind("127.0.0.1:3457")?;
    listener.set_nonblocking(true)?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");

    if let Some(asset_dir_str) = assets_dir.to_str() {
        info!("asset dir: {}", asset_dir_str)
    }

    if let Some(config_dir_str) = config_dir.to_str() {
        info!("config dir: {}", config_dir_str)
    }

    let game_data = GameDataBuilder::default();

    let mut game = Application::build(assets_dir, ServerGameModel::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            1,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
