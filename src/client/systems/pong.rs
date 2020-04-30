use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    shrev::{EventChannel, ReaderId},
    Error,
};
use log::*;
use shared::networking::{self, NetworkEvent, NetworkResource};

pub struct PongSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PongSystemBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'_, '_>,
    ) -> Result<(), Error> {
        builder.add(
            Pong::new(world),
            "Pong",
            &[networking::TCP_NETWORK_LISTENER_SYSTEM_NAME],
        );

        Ok(())
    }
}

struct Pong {
    reader_id: ReaderId<NetworkEvent>,
}

impl Pong {
    fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world
            .fetch_mut::<EventChannel<NetworkEvent>>()
            .register_reader();
        Self { reader_id }
    }
}

impl<'a> System<'a> for Pong {
    type SystemData = (
        Read<'a, EventChannel<NetworkEvent>>,
        Write<'a, NetworkResource>,
    );
    fn run(&mut self, (channel, mut net_resource): Self::SystemData) {
        for event in channel.read(&mut self.reader_id) {
            match event {
                NetworkEvent::Message(addr, bytes) => {
                    if bytes[..].eq(b"ping") {
                        match net_resource.send(addr, "pong") {
                            Err(e) => error!("Pong Error: {}", e),
                            _ => (),
                        }
                    } else {
                        warn!("Bytes: {:?}", bytes);
                    }
                }
                _ => {}
            }
        }
    }
}
