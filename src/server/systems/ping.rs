use amethyst::ecs::{System, Write};
use shared::networking::NetworkResource;

pub struct Ping;

impl<'a> System<'a> for Ping {
    type SystemData = Write<'a, NetworkResource>;
    fn run(&mut self, mut net_resource: Self::SystemData) {
        net_resource.send_all("ping");
    }
}
