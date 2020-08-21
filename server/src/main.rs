#[macro_use]
extern crate log;

use simple_logger;

use naia_server::{find_my_ip_address, NaiaServer, ServerConfig, ServerEvent, UserKey};

use naia_qs_example_shared::{
    get_shared_config, manifest_load, PointEntity, KeyEvent, AuthEvent, ExampleEvent, ExampleEntity,
};

use std::{cell::RefCell, net::SocketAddr, rc::Rc, time::Duration};

const SERVER_PORT: u16 = 14191;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).expect("A logger was already initialized");

    info!("Naia Quicksilver Server Example Started");

    let current_ip_address = find_my_ip_address().expect("can't find ip address");
    let current_socket_address = SocketAddr::new(current_ip_address, SERVER_PORT);

    let mut server_config = ServerConfig::default();
    server_config.heartbeat_interval = Duration::from_secs(2);
    server_config.disconnection_timeout_duration = Duration::from_secs(5);

    let mut server = NaiaServer::new(
        current_socket_address,
        manifest_load(),
        Some(server_config),
        get_shared_config(),
    )
    .await;

    server.on_auth(Rc::new(Box::new(|_, auth_type| {
        if let ExampleEvent::AuthEvent(auth_event) = auth_type {
            let username = auth_event.username.get();
            let password = auth_event.password.get();
            return username == "charlie" && password == "12345";
        }
        return false;
    })));

    let main_room_key = server.create_room();

    let main_entity = PointEntity::new(16,16).wrap();
    let entity_key = server.register_entity(main_entity);
    server.room_add_entity(&main_room_key, &entity_key);

    server.on_scope_entity(Rc::new(Box::new(|_, _, _, entity| match entity {
        ExampleEntity::PointEntity(point_entity) => {
            return true;
        }
    })));

    let mut tick_count: u32 = 0;

    loop {
        match server.receive().await {
            Ok(event) => {
                match event {
                    ServerEvent::Connection(user_key) => {
                        server.room_add_user(&main_room_key, &user_key);
                        if let Some(user) = server.get_user(&user_key) {
                            info!("Naia Server connected to: {}", user.address);
                        }
                    }
                    ServerEvent::Disconnection(_, user) => {
                        info!("Naia Server disconnected from: {:?}", user.address);
                    }
                    ServerEvent::Event(user_key, event_type) => {
                        if let Some(user) = server.get_user(&user_key) {
                            match event_type {
                                ExampleEvent::KeyEvent(key_event) => {
                                    info!("Naia Server recv <- {}", user.address);
                                }
                                _ => {}
                            }
                        }
                    }
                    ServerEvent::Tick => {
                        server.send_all_updates().await;
                    }
                }
            }
            Err(error) => {
                info!("Naia Server Error: {}", error);
            }
        }
    }
}
