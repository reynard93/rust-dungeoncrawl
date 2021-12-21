use crate::prelude::*;

#[system]
#[write_component(Point)] // requests writable access to Point component
#[read_component(Player)] // requests read-only access to Player component
pub fn player_input(
    ecs: &mut SubWorld, // like a World but can only see requested components
    #[resource] map: &Map, // #[resource] requests access to types you stored in Legion's rsc handler
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query()
                .filter(component::<Player>());
            // why is iter_mut passed ecs?
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}
