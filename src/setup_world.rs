use crate::components::*;
use bevy::{color::palettes::tailwind, prelude::*};
use rand::prelude::*;

pub struct YeetSetupWorldPlugin;

impl Plugin for YeetSetupWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_world, setup_lights));
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const RADIUS: f32 = 2.0;
    const OUTER_BOUND: f32 = 20.0;
    const INNER_BOUND: f32 = 4.0;
    const MIN_DISTANCE: f32 = RADIUS * 2.5;
    let mut rng = rand::rng();
    let mut valid_points = Vec::new();

    while valid_points.len() < 10 {
        let (x, y, z): (f32, f32, f32) = (
            rng.random_range(-OUTER_BOUND..=OUTER_BOUND),
            rng.random_range(-OUTER_BOUND..=OUTER_BOUND),
            rng.random_range(-OUTER_BOUND..=OUTER_BOUND),
        );

        // Check if the point is outside the inner bound
        if x.abs() > INNER_BOUND && y.abs() > INNER_BOUND && z.abs() > INNER_BOUND {
            let mut collision = false;

            // Check for collisions with existing spheres
            for (existing_x, existing_y, existing_z) in &valid_points {
                let dx: f32 = x - existing_x;
                let dy: f32 = y - existing_y;
                let dz: f32 = z - existing_z;
                let distance: f32 = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
                if distance < MIN_DISTANCE {
                    collision = true;
                    println!("Collision found !");
                    break; // No need to check other points if a collision is found.
                }
            }

            if !collision {
                valid_points.push((x, y, z));
            }
        }
    }

    // Spawn the spheres from valid points. No shuffling needed now!
    for (x, y, z) in &valid_points {
        let sphere_mesh = meshes.add(Sphere::new(RADIUS));
        let sphere_material = materials.add(Color::from(tailwind::BLUE_700));
        commands.spawn((
            Mesh3d(sphere_mesh),
            MeshMaterial3d(sphere_material.clone()),
            Transform::from_xyz(*x, *y, *z), // Dereference to get the values
        ));
    }
}

fn setup_lights(mut commands: Commands) {
    commands.spawn(AmbientLight {
        color: Color::from(tailwind::SKY_950),
        ..default()
    });

    commands.spawn((
        PointLight {
            color: Color::from(tailwind::YELLOW_100),
            intensity: 40_000_000.0, // this value seems to create a good brightness of the pointlight
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0),
    ));
}
