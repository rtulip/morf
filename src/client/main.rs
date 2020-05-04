extern crate ron;
extern crate serde;
extern crate shared;
use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use shared::networking;
use std::net::{SocketAddr, TcpListener};
use std::time::Duration;
mod systems;

mod client_game_model;
use crate::client_game_model::ClientGameModel;
mod ball;

use log::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let listener = TcpListener::bind("127.0.0.1:0")?;
    listener.set_nonblocking(true)?;

    let server_addr: SocketAddr = "127.0.0.1:8080"
        .parse()
        .expect("Failed to parse server address");

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let input_path = app_root.join("config").join("bindings.ron");

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");

    if let Some(asset_dir_str) = assets_dir.to_str() {
        info!("asset dir: {}", asset_dir_str)
    }

    if let Some(config_dir_str) = config_dir.to_str() {
        info!("config dir: {}", config_dir_str)
    }

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(input_path)?)?
        .with_bundle(networking::TcpSystemBundle::new(
            listener,
            Some(server_addr),
        ))?
        .with_bundle(systems::PongSystemBundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        );

    let mut game = Application::build(assets_dir, ClientGameModel::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            60,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
