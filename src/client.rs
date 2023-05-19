use ambient_api::{
    components::core::camera::aspect_ratio_from_window, concepts::make_orthographic_camera,
    prelude::*,
};

use components::player_camera_ref;

#[main]
pub fn main() {
    spawn_query((player(), user_id())).bind(move |players| {
        for (id, (_, user)) in players {
            let local_user_id =
                entity::get_component(entity::resources(), local_user_id()).unwrap();
            eprintln!("Player joined {user}\nlocal_user_id: {local_user_id:?}");
            // First, we check if this player is the "local" player, and only then do we attach a camera
            if user == local_user_id {
                eprintln!("Attaching camera to player {}", user);
                let camera = Entity::new()
                    .with_merge(make_orthographic_camera())
                    .with(aspect_ratio_from_window(), EntityId::resources())
                    .with_default(main_scene())
                    .with(user_id(), user)
                    .with(translation(), Vec3::new(0., 0., 0.))
                    .spawn();

                entity::add_components(id, Entity::new().with(player_camera_ref(), camera));
            }
        }
    });

    query(window_physical_size()).each_frame(move |window| {
        ambient_api::messages::Frame::subscribe(move |_| {
            let (delta, input) = input::get_delta();

            if !delta.mouse_buttons.is_empty() {
                for (_, windows) in &window {
                    let window = windows.as_vec2();

                    let window_width = window.x;
                    let window_height = window.y;

                    let click_pos = vec2(
                        input.mouse_position.x - window_width / 2.0,
                        input.mouse_position.y - window_height / 2.0,
                    );

                    messages::Input::new(click_pos).send_server_reliable();
                }
            }
        });
    });
}
