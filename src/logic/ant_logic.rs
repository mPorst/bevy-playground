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
                            let dir =
                                (ast_transform.translation - ant_transform.translation).normalize();
                            let dist = ast_transform
                                .translation
                                .distance(ant_transform.translation);
                            // approach the asteroid if it is farther away than 1 unit
                            if dist > 1.0 {
                                // speed down the ant when it approaches an asteroid
                                let speed = 0.1 - 0.09 / (1.0 + dist / 5.0) * heaviside(dist, 10.0);
                                ant_transform.translation = ant_transform.translation + dir * speed;
                            } else {
                                // start the transfer once the asteroid is in range
                                if ast_storage.stored_ore >= ant_storage.max_stored_ore {
                                    ast_storage.stored_ore -= ant_storage.max_stored_ore;
                                    ant_storage.stored_ore += ant_storage.max_stored_ore;
                                    println!(
                                        "Ant storage: {} after taking from asteroid storage: {}",
                                        ant_storage.stored_ore, ast_storage.stored_ore
                                    );
                                } else {
                                    ant_storage.stored_ore += ast_storage.stored_ore;
                                    ast_storage.stored_ore = 0.0;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("unable to get asteroid in ant_agent_logic");
                    }
                }
            } else {
                for (ast_transform, mut ast_storage, ast_entity) in query_asteroid.iter_mut() {
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
                        let dir =
                            (home_transform.translation - ant_transform.translation).normalize();
                        let dist = home_transform
                            .translation
                            .distance(ant_transform.translation);
                        // approach the homebase if it is farther away than 1 unit
                        if dist > 1.0 {
                            // speed down the ant when it approaches the base
                            let speed = 0.1 - 0.09 / (1.0 + dist / 5.0) * heaviside(dist, 10.0);
                            ant_transform.translation = ant_transform.translation + dir * speed;
                        } else {
                            // start the transfer once the homebase is in range
                            home_storage.stored_ore += ant_storage.stored_ore;
                            ant_storage.stored_ore -= ant_storage.stored_ore;
                            println!(
                                "Ant storage: {} after putting into home storage: {}",
                                ant_storage.stored_ore, home_storage.stored_ore
                            );
                        }
                    }
                    Err(e) => {
                        println!("unable to get asteroid in ant_agent_logic");
                    }
                }
            }
        }
    }
}
