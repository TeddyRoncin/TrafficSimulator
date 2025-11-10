mod road;
mod agent;
mod gui;

struct SimulationData {
    roads: road::Roads,
    cars: Vec<agent::car::Car>,
}


#[macroquad::main("MyGame")]
async fn main() {
    let mut sd: SimulationData = SimulationData { roads: road::Roads::new(), cars: Vec::new() };
    let mut window = gui::Window::new().await;
    sd.cars.push(agent::car::Car::new(road::RoadPoint::new(0, 0.)));
    // for _ in 0..10 {
    //     let road_idx: usize = gen_range(0, sd.roads.segments.len());
    //     let progression: f32 = gen_range(0., 1.);
    //     sd.cars.push(Car::new(roads::RoadPoint::new(road_idx, progression)));
    // }
    loop {
        let step_size = macroquad::prelude::get_frame_time();
        let roads = &sd.roads;
        for car in sd.cars.iter_mut() {
            car.step(step_size, roads);
        }
        macroquad::window::clear_background(macroquad::color::BLACK);
        sd.roads.render(&window);
        for (i, car) in sd.cars.iter().enumerate() {
            car.render(&window, &sd.roads, i == 0)
        }
        window.update();
        macroquad::time::draw_fps();
        macroquad::window::next_frame().await;
    }
}
