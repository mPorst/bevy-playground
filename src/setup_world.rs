use crate::components::*;
use bevy::{color::palettes::tailwind, log::*, prelude::*};
use rand::prelude::*;

pub struct YeetSetupWorldPlugin;

impl Plugin for YeetSetupWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_hives,
                setup_asteroids.after(setup_hives),
                setup_lights,
            ),
        );
    }
}

fn setup_hives(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const RADIUS: f32 = 2.0;
    const OUTER_BOUND: f32 = 50.0;
    const INNER_BOUND: f32 = 4.0;
    const MIN_DISTANCE: f32 = RADIUS * 2.5;
    let mut rng = rand::rng();
    let mut valid_hives: Vec<(f32, f32, f32)> = Vec::new();

    while valid_hives.len() < 3 {
        let (x, y, z): (f32, f32, f32) = (
            rng.random_range(-OUTER_BOUND..=OUTER_BOUND),
            rng.random_range(-OUTER_BOUND..=OUTER_BOUND),
            rng.random_range(-OUTER_BOUND..=OUTER_BOUND),
        );

        // Check if the point is outside the inner bound
        if x.abs() > INNER_BOUND && y.abs() > INNER_BOUND && z.abs() > INNER_BOUND {
            let mut collision = false;

            // Check for enough distance between hives
            for (existing_x, existing_y, existing_z) in &valid_hives {
                let dx: f32 = x - existing_x;
                let dy: f32 = y - existing_y;
                let dz: f32 = z - existing_z;
                let distance: f32 = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
                if distance < MIN_DISTANCE {
                    collision = true;
                    break; // No need to check other points if a collision is found.
                }
            }

            if !collision {
                valid_hives.push((x, y, z));
            }
        }
    }

    // Spawn the hives from valid points.
    for (x, y, z) in &valid_hives {
        let sphere_mesh = meshes.add(Sphere::new(RADIUS));
        let sphere_material = materials.add(Color::from(tailwind::BLUE_700));
        commands.spawn((
            Mesh3d(sphere_mesh),
            MeshMaterial3d(sphere_material.clone()),
            Transform::from_xyz(*x, *y, *z),
            AntHive,
            Collidable,
        ));
    }
}

fn setup_asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&Transform, With<Collidable>>,
) {
    let my_span = info_span!("setup_asteroids", name = "setup_asteroids").entered();

    const RADIUS: f32 = 2.0;
    const OUTER_BOUND: f32 = 200.0;
    const INNER_BOUND: f32 = 4.0;
    const MIN_DISTANCE: f32 = RADIUS * 2.5;
    let mut rng = rand::rng();
    let mut valid_asteroids: Vec<(f32, f32, f32)> = Vec::new();

    while valid_asteroids.len() < 50 {
        // generate asteroid coordinates
        let (x, y, z): (f32, f32, f32) = (
            rng.random_range(-OUTER_BOUND..=OUTER_BOUND),
            rng.random_range(-OUTER_BOUND..=OUTER_BOUND),
            rng.random_range(-OUTER_BOUND..=OUTER_BOUND),
        );
        // first perform bounds-check
        if x.abs() > INNER_BOUND && y.abs() > INNER_BOUND && z.abs() > INNER_BOUND {
            let mut collision = false;
            // loop over all collidables
            for component in query.iter() {
                let c_x = component.translation.x;
                let c_y = component.translation.y;
                let c_z = component.translation.z;
                let dx: f32 = x - c_x;
                let dy: f32 = y - c_y;
                let dz: f32 = z - c_z;
                let distance: f32 = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
                if distance < MIN_DISTANCE {
                    collision = true;
                    break; // No need to check other points if a collision is found.
                }
                // check no collision with collidable
            }

            // now check that asteroids don't intersect
            if !collision {
                for (existing_x, existing_y, existing_z) in &valid_asteroids {
                    let dx: f32 = x - existing_x;
                    let dy: f32 = y - existing_y;
                    let dz: f32 = z - existing_z;
                    let distance: f32 = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
                    if distance < MIN_DISTANCE {
                        collision = true;
                        //println!("Collision with asteroid found !");
                        break; // No need to check other points if a collision is found.
                    }
                }

                if !collision {
                    valid_asteroids.push((x, y, z));
                }
            }
        }
    }

    // Spawn the asteroids from valid points.
    for (x, y, z) in &valid_asteroids {
        let sphere_mesh = meshes.add(Sphere::new(RADIUS));
        let sphere_material = materials.add(Color::from(tailwind::EMERALD_500));
        commands.spawn((
            Mesh3d(sphere_mesh),
            MeshMaterial3d(sphere_material.clone()),
            Transform::from_xyz(*x, *y, *z),
            Asteroid,
            Collidable,
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
            range: 30.0,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0),
    ));
}
