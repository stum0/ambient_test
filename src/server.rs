use ambient_api::{components::core::primitives::cube, concepts::make_transformable, prelude::*};
use components::player_mouse_location;

#[main]
pub fn main() {
    spawn_query(player()).bind(move |players| {
        for (id, _) in players {
            entity::add_components(
                id,
                Entity::new()
                    .with_merge(make_transformable())
                    .with_default(cube())
                    .with(color(), Vec4::ONE)
                    .with(scale(), vec3(0.1, 0.1, 0.1))
                    .with(translation(), vec3(0., 0., 0.)),
            );
        }
    });

    messages::Input::subscribe(move |source, msg| {
        let Some(player_id) = source.client_entity_id() else { return; };

        entity::add_component(player_id, player_mouse_location(), msg.mouse_delta_x);
    });

    query((player(), player_mouse_location())).each_frame(move |players| {
        for (player_id, (_, mouse_pos)) in players {
            let displace = (mouse_pos).extend(0.0);

            println!("Player {} clicked {}", player_id, displace);
            entity::set_component(player_id, translation(), displace);
        }
    });
}
