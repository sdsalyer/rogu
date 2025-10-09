use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &mut SubWorld, #[resource] camera: &Camera) {
    const DRAW_LAYER: usize = 1;
    const SORT_ORDER: usize = 5000;

    let offset = Point::new(camera.left_x, camera.top_y);
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(DRAW_LAYER);

    // add each map tile to the draw batch for rendering
    let _ = <(&Point, &Render)>::query() // find all Point components
        .iter(ecs) // loop over SubWorld results
        .for_each(|(pt, r)| {
            //if map.in_bounds(*pt) { // not necessary to check in bounds?
            draw_batch.set(*pt - offset, r.color, r.glyph);
            //}
        });

    draw_batch.submit(SORT_ORDER).expect("Batch error");
}
