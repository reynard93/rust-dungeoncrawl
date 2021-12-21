use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
   let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
           draw_batch.set(
               *pos - offset,
               render.color,
               render.glyph
           );
        });
    draw_batch.submit(5000).expect("Batch error"); /* 5,000 is used as a sort order because the map
    may include 4,000 elements. It’s a good idea to leave some room in case
    that changes or you add */
}
