use std::collections::HashMap;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

#[derive(Component, Clone)]
struct Cell {
    x: i32,
    y: i32,
    is_alive: bool,
}

#[derive(Component)]
struct CellHashMap(HashMap<(i32, i32), bool>);

impl Cell {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            is_alive: rand::random(),
        }
    }
}

fn setup(mut commands: Commands) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());

    let mut cells = Vec::new();
    for i in 0..160 {
        for j in 0..144 {
            cells.push(Cell::new(i, j));
        }
    }

    for cell in cells.iter() {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    cell.x as f32 * 1.0 - 80.0,
                    cell.y as f32 * 1.0 - 72.0,
                    0.0,
                ),
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                ..default()
            },
            Cell {
                x: cell.x,
                y: cell.y,
                is_alive: cell.is_alive,
            },
        ));
    }

    let cell_hashmap: HashMap<(i32, i32), bool> = cells
        .iter()
        .map(|cell| ((cell.x, cell.y), cell.is_alive))
        .collect();

    commands.spawn(CellHashMap(cell_hashmap));
}

fn update_cells(
    mut query: Query<(&mut Cell, &mut Sprite)>,
    mut cell_hashmap_query: Query<&mut CellHashMap>,
) {
    let cells = &cell_hashmap_query.single().0;
    let mut new_cells: HashMap<(i32, i32), bool> = HashMap::new();

    for (mut cell, mut sprite) in query.iter_mut() {
        let mut alive_neighbors = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                if let Some(is_neighbor_alive) = cells.get(&(cell.x + i, cell.y + j)) {
                    if *is_neighbor_alive {
                        alive_neighbors += 1;
                    }
                }
            }
        }

        let mut is_alive = false;
        if cell.is_alive {
            if !(2..=3).contains(&alive_neighbors) {
                is_alive = false;
            } else {
                is_alive = true;
            }
        } else if alive_neighbors == 3 {
            is_alive = true;
        }

        if is_alive {
            cell.is_alive = true;
            sprite.color = Color::rgb(1.0, 1.0, 1.0);
            new_cells.insert((cell.x, cell.y), true);
        } else {
            cell.is_alive = false;
            sprite.color = Color::rgb(0.0, 0.0, 0.0);
            new_cells.insert((cell.x, cell.y), false);
        }
    }

    std::mem::swap(&mut cell_hashmap_query.single_mut().0, &mut new_cells);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: {
                WindowDescriptor {
                    title: "Interesting Title".to_string(),
                    width: 600.0,
                    height: 400.0,
                    ..default()
                }
            },
            ..default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(update_cells)
        .run();
}
