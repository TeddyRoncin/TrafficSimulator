mod road;
mod agent;
mod gui;

use macroquad::{prelude::*};

struct SimulationData {
    roads: road::Roads,
    cars: Vec<agent::car::Car>,
}


#[macroquad::main("MyGame")]
async fn main() {
    let mut sd: SimulationData = SimulationData { roads: road::Roads::new(), cars: Vec::new() };
    let window = gui::Window::new();
    sd.cars.push(agent::car::Car::new(road::RoadPoint::new(0, 0.)));
    // for _ in 0..10 {
    //     let road_idx: usize = gen_range(0, sd.roads.segments.len());
    //     let progression: f32 = gen_range(0., 1.);
    //     sd.cars.push(Car::new(roads::RoadPoint::new(road_idx, progression)));
    // }
    loop {
        let step_size = get_frame_time();
        let roads = &sd.roads;
        for car in sd.cars.iter_mut() {
            car.step(step_size, roads);
        }
        clear_background(BLACK);
        sd.roads.render(&window);
        for (i, car) in sd.cars.iter().enumerate() {
            car.render(&window, &sd.roads, i == 0)
        }
        draw_fps();
        next_frame().await;
    }
}
