
use log::info;

use std::{net::SocketAddr, time::Duration};

use naia_client::{ClientConfig, ClientEvent, NaiaClient};

use naia_qs_example_shared::{
    get_shared_config, manifest_load, AuthEvent, ExampleEntity, ExampleEvent,
};

const SERVER_PORT: u16 = 14191;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use std::net::IpAddr;
    } else {
        use naia_client::find_my_ip_address;
    }
}

extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    input::{Input, Key},
    Result, Settings, Window,
};

pub fn get_settings() -> Settings {
    let mut settings = Settings::default();
    settings.size = Vector::new(640.0, 360.0);
    settings
}

pub async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {

    // Naia

    info!("Naia Client Example Started");

    cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                // Put your Server's IP Address here!, can't easily find this automatically from the browser
                let server_ip_address: IpAddr = "192.168.1.5".parse().expect("couldn't parse input IP address");
            } else {
                let server_ip_address = find_my_ip_address().expect("can't find ip address");
            }
        }

    let server_socket_address = SocketAddr::new(server_ip_address, SERVER_PORT);

    let mut client_config = ClientConfig::default();
    client_config.heartbeat_interval = Duration::from_secs(2);
    client_config.disconnection_timeout_duration = Duration::from_secs(5);

    let auth = ExampleEvent::AuthEvent(AuthEvent::new("charlie", "12345"));

    let mut client = NaiaClient::new(
            server_socket_address,
            manifest_load(),
            Some(client_config),
            get_shared_config(),
            Some(auth),
        );

    // Quicksilver

    let square_size = Vector::new(32.0, 32.0);
    const SQUARE_SPEED: f32 = 2.0;

    loop {
        while let Some(_) = input.next_event().await {}

        // naia update
        match client.receive() {
            Ok(event) => {
                match event {
                    ClientEvent::Connection => {
                        info!("Client connected to: {}", client.server_address());
                    }
                    ClientEvent::Disconnection => {
                        info!("Client disconnected from: {}", client.server_address());
                    }
                    ClientEvent::CreateEntity(local_key) => {
                        if let Some(entity) = client.get_entity(local_key) {
                            match entity {
                                ExampleEntity::PointEntity(point_entity) => {
                                    info!("creation of point entity with key: {}, x: {}, y: {}",
                                          local_key,
                                          point_entity.as_ref().borrow().x.get(),
                                          point_entity.as_ref().borrow().y.get(),
                                    );
                                }
                            }
                        }
                    }
                    ClientEvent::UpdateEntity(local_key) => {
                        if let Some(entity) = client.get_entity(local_key) {
                            match entity {
                                ExampleEntity::PointEntity(point_entity) => {
                                    info!("update of point entity with key: {}, x:{}, y: {}",
                                          local_key,
                                          point_entity.as_ref().borrow().x.get(),
                                          point_entity.as_ref().borrow().y.get());
                                }
                            }
                        }
                    }
                    ClientEvent::DeleteEntity(local_key) => {
                        info!("deletion of point entity with key: {}", local_key);
                    }
                    ClientEvent::None => {
                        //info!("Client non-event");
                    }
                    _ => {}
                }
            }
            Err(err) => {
                info!("Client Error: {}", err);
            }
        }

        // input
//        if input.key_down(Key::A) {
//            square_position.x -= SQUARE_SPEED;
//        }
//        if input.key_down(Key::D) {
//            square_position.x += SQUARE_SPEED;
//        }
//        if input.key_down(Key::W) {
//            square_position.y -= SQUARE_SPEED;
//        }
//        if input.key_down(Key::S) {
//            square_position.y += SQUARE_SPEED;
//        }

        // drawing
        gfx.clear(Color::BLACK);

        if let Some(iter) = client.entities_iter() {
            for (key, entity) in iter {
                match entity {
                    ExampleEntity::PointEntity(point_entity) => {
                        let rect = Rectangle::new(
                            Vector::new(
                                f32::from(*(point_entity.as_ref().borrow().x.get())),
                                f32::from(*(point_entity.as_ref().borrow().y.get()))),
                            square_size);
                        gfx.fill_rect(&rect, Color::WHITE);
                    }
                }
            }
        }

        gfx.present(&window)?;
    }
}