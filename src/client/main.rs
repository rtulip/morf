extern crate ron;
extern crate serde;
extern crate shared;
use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy, prelude::*, utils::application_root_dir,
};
use shared::networking;
use std::net::{SocketAddr, TcpListener};
use std::time::Duration;
mod systems;

#[derive(Default)]
struct ClientGameModel;

impl SimpleState for ClientGameModel {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let _world = data.world;
    }
}
use log::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let listener = TcpListener::bind("127.0.0.1:0")?;
    listener.set_nonblocking(true)?;
    let server_addr: SocketAddr = "127.0.0.1:8080"
        .parse()
        .expect("Failed to parse server address");

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");

    if let Some(asset_dir_str) = assets_dir.to_str() {
        info!("asset dir: {}", asset_dir_str)
    }

    if let Some(config_dir_str) = config_dir.to_str() {
        info!("config dir: {}", config_dir_str)
    }

    let game_data = GameDataBuilder::default()
        .with_bundle(networking::TcpSystemBundle::new(
            listener,
            Some(server_addr),
        ))?
        .with_bundle(systems::PongSystemBundle)?;

    let mut game = Application::build(assets_dir, ClientGameModel::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            1,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
