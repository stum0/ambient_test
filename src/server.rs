use ambient_api::{
    components::core::{
        physics::{
            character_controller_height, character_controller_radius, physics_controlled,
            plane_collider,
        },
        primitives::{cube, quad},
    },
    concepts::make_transformable,
    physics::raycast_first,
    prelude::*,
};
use components::player_mouse_location;

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 100.)
        .with(color(), vec4(1., 0., 0., 1.))
        .with_default(plane_collider())
        .with(translation(), vec3(0., 0., 0.))
        .spawn();

    spawn_query(player()).bind(move |players| {
        for (id, _) in players {
            entity::add_components(
                id,
                Entity::new()
                    .with_merge(make_transformable())
                    .with_default(cube())
                    .with(color(), Vec4::ONE * 10.)
                    .with(scale(), vec3(0.5, 0.5, 0.5))
                    .with(translation(), vec3(0., 0., 0.)),
            );
        }
    });

    messages::Input::subscribe(move |source, msg| {
        let Some(player_id) = source.client_entity_id() else { return; };

        if let Some(pos) =
            raycast_first(msg.screen_to_world_ori, msg.screen_to_world_dir.normalize())
        {
            entity::add_component(player_id, player_mouse_location(), pos.position);
        }
    });

    query((player(), player_mouse_location())).each_frame(move |players| {
        for (player_id, (_, mouse_pos)) in players {
            let speed = 0.1;
            let current_pos = entity::get_component::<Vec3>(player_id, translation()).unwrap();
            let direction = mouse_pos - current_pos;
            let new_pos = current_pos + direction * speed;
            entity::set_component(player_id, translation(), new_pos);
        }
    });
}
