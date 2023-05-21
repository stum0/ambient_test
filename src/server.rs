use std::collections::HashMap;

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
        .with(scale(), Vec3::ONE * 1000.)
        .with(color(), Vec4::ZERO)
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
                    .with(color(), Vec4::ONE)
                    .with(scale(), vec3(1., 1., 1.))
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
        let mut player_positions: HashMap<EntityId, Vec3> = HashMap::new();
        for (player_id, (_, _mouse_pos)) in &players {
            let current_pos = entity::get_component::<Vec3>(*player_id, translation()).unwrap();
            player_positions.insert(*player_id, current_pos);
        }

        for (player_id, (_, mouse_pos)) in players {
            let speed = 1.0;
            let current_pos = player_positions.get(&player_id).unwrap();
            let direction = mouse_pos - *current_pos;
            let distance = direction.length();
            if distance > speed {
                let new_pos = *current_pos + direction.normalize() * speed;
                entity::set_component(player_id, translation(), new_pos);
            }

            for (other_id, other_pos) in &player_positions {
                if player_id != *other_id {
                    let distance = (*other_pos - mouse_pos).length();
                    if distance < 1. {
                        println!("Player {} clicked on player {}", player_id, other_id);
                    }
                }
            }
        }
    });
}
