use crate::components::*;
use bevy::{color::palettes::tailwind, prelude::*};

pub struct YeetAntLogicPlugin;

impl Plugin for YeetAntLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ant_agent_logic);
    }
}

fn heaviside(x: f32, threshold: f32) -> f32 {
    if x < threshold {
        return 1.0;
    }
    return 0.0;
}

// ant agent logic:
// required data: ant, homebase, nearest asteroid
// logic:
// primary goal: Gather resources.
// 1) If inventory empty AND hive has space => go to nearest asteroid that has ore
// 2) IF inventory contains ore and hive has space => return to hive
fn ant_agent_logic(
    mut commands: Commands,
    mut query_ant: Query<(&mut Transform, &mut Storage, &mut Target, &HomeBase), With<Ant>>,
    mut query_asteroid: Query<(&Transform, &mut Storage, Entity), (With<Asteroid>, Without<Ant>)>,
    mut query_hive: Query<
        (&Transform, &mut Storage),
        (With<AntHive>, Without<Asteroid>, Without<Ant>),
    >,
) {
    for (mut ant_transform, mut ant_storage, mut ant_target, ant_homebase) in query_ant.iter_mut() {
        if ant_storage.stored_ore == 0.0 {
            if let Some(target_asteroid) = ant_target.target {
                match query_asteroid.get_mut(target_asteroid) {
                    Ok((ast_transform, mut ast_storage, _)) => {
                        // if the asteroid got emptied out in between go somewhere else
                        if ast_storage.stored_ore == 0.0 {
                            ant_target.target = None;
                        } else {
                            move_to_target(&mut ant_transform, ast_transform);
                            let dist = ast_transform
                                .translation
                                .distance(ant_transform.translation);
                            // approach the asteroid if it is farther away than 1 unit
                            if dist < 1.1 {
                                // start the transfer once the asteroid is in range
                                transfer_storage(&mut ant_storage, &mut ast_storage);
                            }
                        }
                    }
                    Err(e) => {
                        println!("unable to get asteroid in ant_agent_logic");
                    }
                }
            } else {
                for (ast_transform, ast_storage, ast_entity) in query_asteroid.iter_mut() {
                    let dist = ast_transform
                        .translation
                        .distance(ant_transform.translation);
                    if dist < 90.0 && ast_storage.stored_ore != 0.0 {
                        ant_target.target = Some(ast_entity);
                    }
                }
            }
        } else {
            // ant_storage.stored_ore > 0.0
            if let Some(homebase) = ant_homebase.home_base {
                match query_hive.get_mut(homebase) {
                    Ok((home_transform, mut home_storage)) => {
                        move_to_target(&mut ant_transform, home_transform);
                        let dist = ant_transform
                            .translation
                            .distance(home_transform.translation);
                        if dist < 1.1 {
                            transfer_storage(&mut home_storage, &mut ant_storage);
                            //println!(
                            //    "Ant storage: {} after putting into home storage: {}",
                            //    ant_storage.stored_ore, home_storage.stored_ore
                            //);
                        }
                    }
                    Err(e) => {
                        println!("unable to get homebase in ant_agent_logic");
                    }
                }
            }
        }
    }
}

fn move_to_target(mover: &mut Transform, target: &Transform) {
    /// 1) Rotate
    let to_target = (target.translation - mover.translation).normalize();
    let mover_target_rotation = Quat::from_rotation_arc(Vec3::Y, to_target);

    // need a threshold to identify the rotations to be equal
    const ROT_THRESHOLD: f32 = 0.5;
    let angle = mover
        .rotation
        .angle_between(mover_target_rotation)
        .to_degrees()
        .abs();
    //println!("rotation angle: {}", angle);

    if angle > ROT_THRESHOLD {
        mover.rotation = mover
            .rotation
            .slerp(mover_target_rotation, 0.03)
            .normalize();
    } else {
        // snap to target rotation
        mover.rotation = mover_target_rotation;
        // then move
        let dir = (target.translation - mover.translation).normalize();
        let dist = target.translation.distance(mover.translation);
        // approach the target if it is farther away than 1 unit
        if dist > 1.0 {
            // speed down the ant when it approaches the base
            let speed = 0.1 - 0.09 / (1.0 + dist / 5.0) * heaviside(dist, 10.0);
            mover.translation = mover.translation + dir * speed;
        }
    }
}

fn transfer_storage(to: &mut Storage, from: &mut Storage) {
    let free_space = to.max_stored_ore - to.stored_ore;
    let avail_volume_for_transfer = from.stored_ore;
    if free_space < avail_volume_for_transfer {
        to.stored_ore += free_space;
        from.stored_ore -= free_space;
    }
    if free_space > avail_volume_for_transfer {
        to.stored_ore += avail_volume_for_transfer;
        from.stored_ore = 0.0;
    }
}
