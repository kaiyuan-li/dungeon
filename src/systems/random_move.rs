use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut movers = <(&mut Point, &MovingRandomly)>::query();
    movers.iter_mut(ecs).for_each(|(pos, _)| {
        let directions = vec![
            Point::new(0, 1),
            Point::new(0, -1),
            Point::new(1, 0),
            Point::new(-1, 0),
        ];
        let destination = *pos + directions[RandomNumberGenerator::new().range(0, 4)];
        if map.can_enter_tile(destination) {
            *pos = destination;
        }
    });
}
