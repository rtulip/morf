extern crate ron;
extern crate serde;
extern crate shared;
use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy, prelude::*, utils::application_root_dir,
};

use shared::networking;

use std::net::TcpListener;
use std::time::Duration;

#[derive(Default)]
struct ServerGameModel;

impl SimpleState for ServerGameModel {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let listener = TcpListener::bind("localhost:8080")?;
    listener.set_nonblocking(true)?;

    let assets_dir = app_root.join("assets");
    let _config_dir = app_root.join("config");

    let game_data = GameDataBuilder::default().with(
        networking::TcpListenerSystem::new(listener),
        "TcpListener",
        &[],
    );

    let mut game = Application::build(assets_dir, ServerGameModel::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            1,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
