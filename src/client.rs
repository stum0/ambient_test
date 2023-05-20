use ambient_api::{
    camera::screen_to_world_direction, components::core::camera::aspect_ratio_from_window,
    concepts::make_perspective_infinite_reverse_camera, prelude::*,
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
                    .with_merge(make_perspective_infinite_reverse_camera())
                    .with(aspect_ratio_from_window(), EntityId::resources())
                    .with_default(main_scene())
                    .with(user_id(), user)
                    .with(translation(), vec3(0., 10., 10.))
                    .with(lookat_target(), vec3(0., 0., 0.))
                    .spawn();

                entity::add_components(id, Entity::new().with(player_camera_ref(), camera));
            }
        }
    });

    ambient_api::messages::Frame::subscribe(move |_| {
        query(player_camera_ref()).each_frame(move |camera| {
            let (delta, input) = input::get_delta();
            for (_, camera) in camera {
                if !delta.mouse_buttons.is_empty() {
                    let click_pos = screen_to_world_direction(camera, input.mouse_position);

                    messages::Input::new(click_pos.dir, click_pos.origin).send_server_reliable();
                }
            }
        });
    });
}
