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
    // Keep in mind that the disconnect timeout duration should always be at least
    // 2x greater than the heartbeat interval, to make it so at the worst case, the
    // server would need to miss 2 heartbeat signals before disconnecting from a
    // given client
    server_config.disconnection_timeout_duration = Duration::from_secs(5);

    let mut server = NaiaServer::new(
        current_socket_address,
        manifest_load(),
        Some(server_config),
        get_shared_config(),
    )
    .await;

    // This method is called during the connection handshake process, and can be
    // used to reject a new connection if the correct credentials have not been
    // provided
    server.on_auth(Rc::new(Box::new(|_, auth_type| {
        if let ExampleEvent::AuthEvent(auth_event) = auth_type {
            let username = auth_event.username.get();
            let password = auth_event.password.get();
            return username == "charlie" && password == "12345";
        }
        return false;
    })));

    // Create a new, singular room, which will contain Users and Entities that they
    // can receive updates from
    let main_room_key = server.create_room();

    // Create 4 PointEntities, with a range of X values
    let main_entity = PointEntity::new(16,16).wrap();
    let entity_key = server.register_entity(main_entity);
    server.room_add_entity(&main_room_key, &entity_key);

    // This method will be called every step to determine whether a given Entity
    // should be in scope for a given User
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
                        // Game logic, updating of the world, should happen here

                        // VERY IMPORTANT! Calling this actually sends all Entity/Event data packets
                        // to all Clients that require it. If you don't call this method, the Server
                        // will never communicate with it's connected Clients
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
