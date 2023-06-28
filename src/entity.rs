use glam::{Vec2, Vec3};
use wasm_bindgen::prelude::*;

#[path = "utils.rs"]
mod utils;

#[wasm_bindgen(module = "/js-side/src/Renderer.ts")]
extern "C" {
    type Renderer;
    // Import the constructor for the Renderer class.
    #[wasm_bindgen(constructor)]
    fn new() -> Renderer;
    #[wasm_bindgen(method)]
    fn circle(this: &Renderer, x: f32, y: f32, radius: f32, color: &str);
    #[wasm_bindgen(method)]
    fn rect(this: &Renderer, x: f32, y: f32, width: f32, height: f32, color: &str);
    #[wasm_bindgen(method)]
    fn clear(this: &Renderer);
    #[wasm_bindgen(method, js_name = moveCamera)]
    fn move_camera(this: &Renderer, x: f32, y: f32);
    #[wasm_bindgen(method, js_name = zoomCamera)]
    fn zoom(this: &Renderer, zoom: f32, x: f32, y: f32);
    #[wasm_bindgen(method, js_name = getScale)]
    fn get_scale(this: &Renderer) -> f32;
    #[wasm_bindgen(method, js_name = getCameraX)]
    fn get_camera_x(this: &Renderer) -> f32;
    #[wasm_bindgen(method, js_name = getCameraY)]
    fn get_camera_y(this: &Renderer) -> f32;
}

#[wasm_bindgen(module = "/js-side/src/Input.ts")]
extern "C" {
    type Input;
    #[wasm_bindgen(static_method_of = Input)]
    fn init();
    #[wasm_bindgen(static_method_of = Input, js_name = isDown)]
    fn is_down(key: &str) -> bool;
    #[wasm_bindgen(static_method_of = Input, js_name = isMouseDown)]
    fn is_mouse_down() -> bool;
    #[wasm_bindgen(static_method_of = Input, js_name = getWheel)]
    fn get_wheel() -> f32;
    #[wasm_bindgen(static_method_of = Input, js_name = getMouseX)]
    fn get_mouse_x() -> f32;
    #[wasm_bindgen(static_method_of = Input, js_name = getMouseY)]
    fn get_mouse_y() -> f32;
}

struct TerrainBlock {
    number_of_plants: u32,
    max_carrying_capacity: u32,
    fertility: f32,
}

// Create a color RGB(200, 181, 170) this is for not fertile terrain
static INFERTILE_TERRAIN_COLOR: Vec3 = Vec3::new(200.0, 181.0, 170.0);
static FERTILE_TERRAIN_COLOR: Vec3 = Vec3::new(58.0, 46.0, 39.0);

struct Terrain {
    blocks: Vec<Vec<TerrainBlock>>,
}

static GRID_SIZE: i32 = 50;
static HALF_GRID_SIZE: i32 = GRID_SIZE / 2;
static BLOCK_SIZE: f32 = 200.0;

impl Terrain {
    fn new() -> Self {
        let mut blocks: Vec<Vec<TerrainBlock>> = vec![];

        let max_distance_from_center =
            Vec2::new((HALF_GRID_SIZE) as f32, (HALF_GRID_SIZE) as f32).length();

        for x in (-HALF_GRID_SIZE)..HALF_GRID_SIZE {
            let mut row: Vec<TerrainBlock> = vec![];

            for y in (-HALF_GRID_SIZE)..HALF_GRID_SIZE {
                // Places closer to the center are more fertile

                let distance_from_center = Vec2::new(x as f32, y as f32).length();
                let fertility = 1.0 - (distance_from_center / max_distance_from_center);

                row.push(TerrainBlock {
                    number_of_plants: 0,
                    fertility,
                    max_carrying_capacity: 50 * 50,
                });
            }

            blocks.push(row);
        }

        Self { blocks }
    }
}

#[wasm_bindgen]
pub struct App {
    entities: Vec<Entity>,
    renderer: Renderer,
    plants: Vec<Plant>,
    terrain: Terrain,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
impl App {
    pub fn new() -> Self {
        utils::set_panic_hook();
        let renderer = Renderer::new();
        Input::init();

        let mut entities: Vec<Entity> = vec![];

        // for _ in 0..100 {
        //     entities.push(Entity {
        //         position: Vec2::new(
        //             (rand::random::<f32>() * 2.0 - 1.0) * 100.0,
        //             (rand::random::<f32>() * 2.0 - 1.0) * 100.0,
        //         ),
        //         size: rand::random::<f32>() * 0.9 + 0.1,
        //         sense_distance: rand::random::<f32>() * 0.9 + 0.1,
        //         velocity_mag: rand::random::<f32>() * 0.9 + 0.1,
        //         random_direction: Vec2::new(
        //             rand::random::<f32>() * 2.0 - 1.0,
        //             rand::random::<f32>() * 2.0 - 1.0,
        //         )
        //         .normalize(),
        //         random_direction_uses: 0,
        //         energy: 10000.0,
        //     });
        // }

        let mut plants: Vec<Plant> = vec![];

        // for _ in 0..1000 {
        //     plants.push(Plant {
        //         position: Vec2::new(
        //             (rand::random::<f32>() * 2.0 - 1.0) * 100.0,
        //             (rand::random::<f32>() * 2.0 - 1.0) * 100.0,
        //         ),
        //         size: rand::random::<f32>() * 0.1 + 0.1,
        //     });
        // }

        let terrain = Terrain::new();

        Self {
            entities,
            renderer,
            plants,
            terrain,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Clear the screen
        self.renderer.clear();

        // Draw terrain
        for (x, row) in self.terrain.blocks.iter().enumerate() {
            for (y, block) in row.iter().enumerate() {
                let fertility = block.fertility;

                let color = linear_interpolate_vec(
                    INFERTILE_TERRAIN_COLOR,
                    FERTILE_TERRAIN_COLOR,
                    fertility,
                );
                let x = x as f32 - HALF_GRID_SIZE as f32;
                let y = y as f32 - HALF_GRID_SIZE as f32;

                self.renderer.rect(
                    x * BLOCK_SIZE,
                    y * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    &format!(
                        "rgb({}, {}, {})",
                        color.x as u8, color.y as u8, color.z as u8
                    ),
                );
            }
        }

        update(&mut self.entities, &mut self.plants, dt);

        // Remove dead entities and increase the fertility of the land where they died.
        let mut dead_entities: Vec<usize> = Vec::new();
        for i in 0..self.entities.len() {
            if self.entities[i].energy <= 0.0 {
                dead_entities.push(i);
            }
        }

        // Increase the fertility of the land where they died.
        for i in 0..dead_entities.len() {
            let entity = &self.entities[dead_entities[i]];

            if entity.position.x.is_nan() || entity.position.y.is_nan() {
                continue;
            }

            // If the plant is in a fertile area the probability of it growing is higher
            let block_x = ((entity.position.x / 4.0) + HALF_GRID_SIZE as f32).round() as usize;
            let block_y = ((entity.position.y / 4.0) + HALF_GRID_SIZE as f32).round() as usize;

            if block_x >= (GRID_SIZE as usize) || block_y >= (GRID_SIZE as usize) {
                continue;
            }

            self.terrain.blocks[block_x][block_y].fertility += 0.3;
        }

        self.plants.retain(|plant| plant.size > 0.0);
        self.entities.retain(|entity| entity.energy > 0.0);

        // Draw entities
        for entity in self.entities.iter() {
            self.renderer.circle(
                entity.position.x * 50.0,
                entity.position.y * 50.0,
                entity.size * 50.0,
                &format!(
                    "rgb({}, {}, {})",
                    (entity.size * 255.0) as u8,
                    (entity.sense_distance * 255.0) as u8,
                    (entity.velocity_mag * 255.0) as u8
                ),
            );
        }

        // Generate more plants
        // Plants are more likely to grow in areas near the (0, 0) point

        let mut new_plants: Vec<Plant> = vec![];

        for plant in self.plants.iter() {
            // If the plant is in a fertile area the probability of it growing is higher
            let block_x = ((plant.position.x / 4.0) + HALF_GRID_SIZE as f32).round() as usize;
            let block_y = ((plant.position.y / 4.0) + HALF_GRID_SIZE as f32).round() as usize;

            if block_x >= (GRID_SIZE as usize) || block_y >= (GRID_SIZE as usize) {
                continue;
            }

            let block = &self.terrain.blocks[block_x][block_y];

            let probability_of_growth = block.fertility * 0.01;

            if rand::random::<f32>() < probability_of_growth {
                let new_plant_size = rand::random::<f32>() * 0.1 + 0.1;
                let random_vec = Vec2::new(
                    rand::random::<f32>() * 2.0 - 1.0,
                    rand::random::<f32>() * 2.0 - 1.0,
                )
                .normalize();

                let new_plant_position =
                    plant.position + random_vec * (plant.size + new_plant_size) * 10.0;

                new_plants.push(Plant {
                    position: new_plant_position,
                    size: new_plant_size,
                });

                // Reduce fertility of the block
                self.terrain.blocks[block_x][block_y].fertility -= 0.03;

                if self.terrain.blocks[block_x][block_y].fertility < 0.0 {
                    self.terrain.blocks[block_x][block_y].fertility = 0.0;
                }
            }
        }

        self.plants.append(&mut new_plants);

        // Draw plants
        for plant in self.plants.iter() {
            self.renderer.rect(
                plant.position.x * 50.0,
                plant.position.y * 50.0,
                plant.size * 50.0,
                plant.size * 50.0,
                "#004e00",
            );
        }

        // Add WASD movement
        let mut movement = Vec2::new(0.0, 0.0);
        if Input::is_down("w") {
            movement.y += 15.0;
        }
        if Input::is_down("s") {
            movement.y -= 15.0;
        }
        if Input::is_down("a") {
            movement.x += 15.0;
        }
        if Input::is_down("d") {
            movement.x -= 15.0;
        }
        self.renderer.move_camera(movement.x, movement.y);

        if Input::is_mouse_down() {
            // Add a new plant
            let mouse_x = Input::get_mouse_x();
            let mouse_y = Input::get_mouse_y();

            let camera_x = self.renderer.get_camera_x();
            let camera_y = self.renderer.get_camera_y();

            let scale = self.renderer.get_scale();

            let new_plant = Plant {
                position: Vec2::new(
                    ((mouse_x - camera_x) / 50.0) / scale,
                    ((mouse_y - camera_y) / 50.0) / scale,
                ),
                size: 0.1,
            };

            self.plants.push(new_plant);
        }

        // Zoom
        let wheel = Input::get_wheel();

        if wheel > 0.0 {
            self.renderer
                .zoom(0.9, Input::get_mouse_x(), Input::get_mouse_y());
        } else if wheel < 0.0 {
            self.renderer
                .zoom(1.1, Input::get_mouse_x(), Input::get_mouse_y());
        }

        // for _ in 0..10 {
        //     self.plants.push(Plant {
        //         position: Vec2::new(
        //             (rand::random::<f32>() * 2.0 - 1.0) * 100.0,
        //             (rand::random::<f32>() * 2.0 - 1.0) * 100.0,
        //         ),
        //         size: rand::random::<f32>() * 0.1 + 0.1,
        //     });
        // }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Plant {
    position: Vec2,
    size: f32,
}

impl Plant {
    pub fn energy(&self) -> f32 {
        self.size * self.size
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    // These properties are passed from the parent to the child when the child is born.
    // The child will slightly mutate these properties.
    // We use these three properties to create a color for the entity.
    // Each one of these properties is a number between 0 and 1.
    size: f32,
    sense_distance: f32,
    velocity_mag: f32,

    random_direction: Vec2,
    random_direction_uses: u32,

    position: Vec2,
    pub energy: f32,
}

impl Entity {
    pub fn new(
        size: f32,
        sense_distance: f32,
        velocity_mag: f32,
        position: Vec2,
        energy: f32,
    ) -> Self {
        Self {
            size,
            sense_distance,
            position,
            velocity_mag,
            energy,
            random_direction: Vec2::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
            )
            .normalize(),
            random_direction_uses: 0,
        }
    }

    fn mass(&self) -> f32 {
        self.size * self.size
    }

    fn velocity(&self) -> f32 {
        (self.velocity_mag) / 10.0
    }

    fn eat(&mut self, other: &mut Entity) {
        self.energy = self.energy + other.energy * self.size;
        other.energy = 0.0;
    }

    fn eat_plant(&mut self, plant: &mut Plant) {
        self.energy = self.energy + plant.energy() * 5000.0;
        plant.size = 0.0;
    }

    fn energy_cost(&self) -> f32 {
        // Energy cost to be alive = mass * velocity^2 + sense_distancew
        self.size * self.velocity_mag + self.sense_distance
    }

    fn move_randomly(&mut self, dt: f32) {
        if self.random_direction_uses > 100 {
            self.random_direction = Vec2::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
            )
            .normalize();
            self.random_direction_uses = 0;
        }

        self.position = self.position + self.random_direction * self.velocity();
        self.random_direction_uses += 1;
        self.consume_energy(dt);
    }

    fn move_towards(&mut self, target: Vec2, distance: f32, dt: f32) {
        // If the distance is less than the velocity then we should just move to the target.
        if distance + self.size < self.velocity() {
            self.position = target;
        } else {
            let direction = (target - self.position).normalize();
            self.position = self.position + direction * self.velocity();
        }

        self.consume_energy(dt);
    }

    fn move_away(&mut self, target: Vec2, dt: f32) {
        let direction = (target - self.position).normalize();
        self.position = self.position - direction * self.velocity();
        self.consume_energy(dt);
    }

    fn consume_energy(&mut self, dt: f32) {
        self.energy = self.energy - self.energy_cost() * dt;
    }
}

pub fn add_children(entities: &mut Vec<Entity>) {
    let mut new_entities: Vec<Entity> = Vec::new();
    for entity in entities.iter_mut() {
        // If we have enough energy to have children then we should have children.
        if entity.energy >= entity.energy_cost() * 10.0 * 1000.0 {
            let reproduction_cost = entity.energy_cost() * 1000.0; // Equivalent to 1 second of energy.
                                                                   // Remove half of the energy from the parent and remove the energy cost of reproduction.
            entity.energy = (entity.energy - reproduction_cost) / 2.0;
            let child_energy = entity.energy;

            // Move the child next to the parent.
            let random_vec = Vec2::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
            )
            .normalize();

            let child_size = randomize_trait(entity.size);
            let child_position = entity.position + random_vec * (entity.size + child_size);

            new_entities.push(Entity::new(
                child_size,
                randomize_trait(entity.sense_distance),
                randomize_trait(entity.velocity_mag),
                child_position,
                child_energy,
            ));
        }
    }

    entities.append(&mut new_entities);
}

fn randomize_trait(mut trait_value: f32) -> f32 {
    trait_value = trait_value * (1.0 + rand::random::<f32>() * 0.2 - 0.1); // +/- 10%

    if trait_value < 0.1 {
        0.1
    } else if trait_value > 1.0 {
        1.0
    } else {
        trait_value
    }
}

pub fn update(entities: &mut Vec<Entity>, plants: &mut Vec<Plant>, dt: f32) {
    // An entity does not interact with entities that are roughly the same size as it
    // because they are not a threat and they are not food.

    // We should check the closest entitie that is 20% bigger than us or the closest entity
    // that is 20% smaller than us and move towards it or away from it respectively if it is
    // within our sense distance.

    let ent_len = entities.len();
    for i in 0..ent_len {
        let entity = &entities[i];

        let mut closest_entity: Option<usize> = None;
        let mut closest_entity_distance: f32 = std::f32::INFINITY;

        // Get the closest entity
        for j in 0..ent_len {
            if i == j {
                continue;
            }

            let other_entity = &entities[j];
            let distance =
                other_entity.position.distance(entity.position) - (other_entity.size + entity.size);

            if distance < entity.sense_distance * 100.0 {
                if distance < closest_entity_distance {
                    closest_entity = Some(j);
                    closest_entity_distance = distance;
                }
            }
        }

        // Get the closest plant
        let mut closest_plant: Option<usize> = None;
        let mut closest_plant_distance: f32 = std::f32::INFINITY;

        for j in 0..plants.len() {
            let plant = &plants[j];
            let distance = plant.position.distance(entity.position) - (entity.size + plant.size);

            if distance < entity.sense_distance * 100.0 {
                if distance < closest_plant_distance {
                    closest_plant = Some(j);
                    closest_plant_distance = distance;
                }
            }
        }

        let mut moved = false;

        if let Some(other_entity_index) = closest_entity {
            let mut entity = entities[i];
            let mut other_entity = entities[other_entity_index];

            // Entities 20% bigger than us can eat us and we can eat entities 20% smaller than us
            // so we should move towards entities 20% smaller than us and move away from entities 20% bigger than us

            // If the distance is negative or zero then we are overlapping with the entity
            // that means we have been eaten.
            if closest_entity_distance <= 0.0 {
                if other_entity.size > entity.size * 1.2 {
                    // We are overlapping with an entity that is 20% bigger than us
                    // so we have been eaten.
                } else if other_entity.size <= entity.size * 0.8 {
                    // We are overlapping with an entity that is 20% smaller than us
                    // so we should eat it.
                    entity.eat(&mut other_entity);
                } else {
                    // We are overlapping with an entity that is roughly the same size as us
                    // so we should move away from it.
                    entity.move_away(other_entity.position, dt);
                    moved = true;
                }
            } else {
                if other_entity.size > entity.size * 1.2 {
                    // Move away from bigger entities
                    entity.move_away(other_entity.position, dt);
                    moved = true;
                } else if other_entity.size < entity.size * 0.8 {
                    // Move towards smaller entities
                    entity.move_towards(other_entity.position, closest_entity_distance, dt);
                    moved = true;
                }
            }

            entities[i] = entity;
            entities[other_entity_index] = other_entity;
        }

        if let Some(plant_index) = closest_plant {
            let mut entity = entities[i];
            let mut plant = plants[plant_index];

            // If the distance is negative or zero then we are overlapping with the plant
            // that means we have eaten it.
            if closest_plant_distance <= 0.0 {
                entity.eat_plant(&mut plant);
                moved = true;
            } else {
                entity.move_towards(plant.position, closest_plant_distance, dt);
                moved = true;
            }

            entities[i] = entity;
            plants[plant_index] = plant;
        }

        if !moved {
            // If there are no plants within our sense distance then we should move randomly
            // in a random direction.
            entities[i].move_randomly(dt);
        }
    }

    // Add children
    add_children(entities);
}

fn linear_interpolate_vec(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}

fn linear_interpolate_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
