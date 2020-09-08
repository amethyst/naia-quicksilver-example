
use log::info;

use std::{net::SocketAddr, time::Duration};

use naia_client::{ClientConfig, ClientEvent, NaiaClient};

use naia_qs_example_shared::{get_shared_config, manifest_load, AuthEvent, ExampleEntity, ExampleEvent, KeyCommand, shared_behavior};

const SERVER_PORT: u16 = 14191;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use std::net::IpAddr;
    } else {
        use naia_client::find_my_ip_address;
    }
}

extern crate quicksilver;

use quicksilver::{geom::{Rectangle, Vector}, graphics::{Color, Graphics}, input::{Input, Key}, Result, Settings, Window, Timer};
use quicksilver::geom::Circle;

pub fn get_settings() -> Settings {
    let mut settings = Settings::default();
    settings.size = Vector::new(1280.0, 720.0);
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

    let mut update_timer = Timer::time_per_second(30.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let mut pawn_key: u16 = 999;

    loop {
        while let Some(_) = input.next_event().await {}

        // naia update
        while update_timer.tick() {
            loop {
                match client.receive() {
                    Some(result) => match result {
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
                                ClientEvent::DeleteEntity(local_key) => {
                                    info!("deletion of point entity with key: {}", local_key);
                                }
                                ClientEvent::Tick => {
                                    let w = input.key_down(Key::W);
                                    let s = input.key_down(Key::S);
                                    let a = input.key_down(Key::A);
                                    let d = input.key_down(Key::D);
                                    if w || s || a || d {
                                        let new_command = KeyCommand::new(w, s, a, d);
                                        client.send_command(pawn_key, &new_command);
                                    }
                                }
                                ClientEvent::AssignPawn(local_key) => {
                                    pawn_key = local_key;
                                    info!("assign pawn");
                                }
                                ClientEvent::UnassignPawn(_) => {
                                    pawn_key = 999;
                                    info!("unassign pawn");
                                }
                                ClientEvent::Command(pawn_key, command_type) => {
                                    match command_type {
                                        ExampleEvent::KeyCommand(key_command) => {
                                            if let Some(typed_entity) = client.get_pawn(pawn_key) {
                                                match typed_entity {
                                                    ExampleEntity::PointEntity(entity) => {
                                                        shared_behavior::process_command(&key_command, entity);
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        Err(err) => {
                            info!("Client Error: {}", err);
                        }
                    },
                    None => { break; }
                }
            }
        }

        // drawing
        if draw_timer.exhaust().is_some() {
            gfx.clear(Color::BLACK);

            if let Some(iter) = client.entities_iter() {
                for (_, entity) in iter {
                    match entity {
                        ExampleEntity::PointEntity(point_entity) => {
                            let rect = Rectangle::new(
                                Vector::new(
                                    f32::from(*(point_entity.as_ref().borrow().x.get())),
                                    f32::from(*(point_entity.as_ref().borrow().y.get()))),
                                square_size);
                            gfx.fill_rect(&rect, Color::RED);
                        }
                    }
                }
            }

            if let Some(iter) = client.pawns_iter() {
                for (_, entity) in iter {
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

            if let Some(iter) = client.pawn_history_iter(&pawn_key) {
                for entity in iter {
                    match entity {
                        ExampleEntity::PointEntity(point_entity) => {
                            let circle = Circle::new(
                                Vector::new(
                                    f32::from(*(point_entity.as_ref().borrow().x.get())) + 16.0,
                                    f32::from(*(point_entity.as_ref().borrow().y.get())) + 16.0),
                                4.0);
                            gfx.stroke_circle(&circle, Color::YELLOW);
                        }
                    }
                }
            }

            gfx.present(&window)?;
        }
    }
}