
use log::info;

use std::{net::SocketAddr, time::Duration};

use naia_client::{ClientConfig, ClientEvent, NaiaClient};

use naia_qs_example_shared::{get_shared_config, manifest_load, AuthEvent, ExampleEntity, ExampleEvent, KeyCommand, shared_behavior, PointEntityColor};

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
                let server_ip_address: IpAddr = "192.168.1.2".parse().expect("couldn't parse input IP address");
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

    let mut update_timer = Timer::time_per_second(60.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let mut pawn_key: Option<u16> = None;
    let mut queued_command: Option<KeyCommand> = None;

    loop {
        while let Some(_) = input.next_event().await {}

        // Queue up key commands
        let w = input.key_down(Key::W);
        let s = input.key_down(Key::S);
        let a = input.key_down(Key::A);
        let d = input.key_down(Key::D);

        if let Some(command) = &mut queued_command {
            if w { command.w.set(true); }
            if s { command.s.set(true); }
            if a { command.a.set(true); }
            if d { command.d.set(true); }
        } else {
            queued_command = Some(KeyCommand::new(w, s, a, d));
        }

        // naia update
        if update_timer.exhaust().is_some() {
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
                                ClientEvent::Tick => {
                                    if let Some(pawn_key) = pawn_key {
                                        if let Some(command) = queued_command.take() {
                                            client.send_command(pawn_key, &command);
                                        }
                                    }
                                }
                                ClientEvent::AssignPawn(local_key) => {
                                    pawn_key = Some(local_key);
                                    info!("assign pawn");
                                }
                                ClientEvent::UnassignPawn(_) => {
                                    pawn_key = None;
                                    info!("unassign pawn");
                                }
                                ClientEvent::Command(pawn_key, command_type) => {
                                    match command_type {
                                        ExampleEvent::KeyCommand(key_command) => {
                                            if let Some(typed_entity) = client.get_pawn_mut(&pawn_key) {
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
                    None => {
                        break;
                    }
                }
            }
        }

        if !client.has_connection() {
            continue;
        }

        // drawing
        if draw_timer.exhaust().is_some() {

            client.frame_begin();

            gfx.clear(Color::BLACK);

            for entity_key in client.entity_keys().unwrap() {
                if let Some(entity) = client.get_entity(&entity_key) {
                    match entity {
                        ExampleEntity::PointEntity(point_entity) => {
                            let rect = Rectangle::new(
                                Vector::new(
                                    f32::from(*(point_entity.as_ref().borrow().x.get())),
                                    f32::from(*(point_entity.as_ref().borrow().y.get()))),
                                square_size);
                            match point_entity.as_ref().borrow().color.get() {
                                PointEntityColor::Red => gfx.fill_rect(&rect, Color::RED),
                                PointEntityColor::Blue => gfx.fill_rect(&rect, Color::BLUE),
                                PointEntityColor::Yellow => gfx.fill_rect(&rect, Color::YELLOW),
                            }
                        }
                    }
                }
            }

            for pawn_key in client.pawn_keys().unwrap() {
                if let Some(entity) = client.get_pawn(&pawn_key) {
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
}