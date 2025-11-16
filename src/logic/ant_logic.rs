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
    mut query_ant: Query<(&mut Transform, &mut Storage, &mut Target), With<Ant>>,
    mut query_asteroid: Query<(&Transform, &mut Storage, Entity), (With<Asteroid>, Without<Ant>)>,
) {
    for (mut ant_transform, mut ant_storage, mut ant_target) in query_ant.iter_mut() {
        if ant_storage.stored_ore == 0.0 {
            if let Some(target_asteroid) = ant_target.target {
                match query_asteroid.get(target_asteroid) {
                    Ok((ast_transform, mut ast_storage, _)) => {
                        // if the asteroid got emptied out in between go somewhere else
                        println!("matched asteroid");
                        if ast_storage.stored_ore == 0.0 {
                            ant_target.target = None;
                        } else {
                            let dir =
                                (ast_transform.translation - ant_transform.translation).normalize();
                            let dist = (ast_transform
                                .translation
                                .distance(ant_transform.translation));
                            // speed down the ant when it approaches an asteroid
                            let speed = 0.1 - 0.09 / (1.0 + dist / 5.0) * heaviside(dist, 10.0);
                            println!("speed is {}", speed);
                            ant_transform.translation = ant_transform.translation + dir * speed;
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
                        println!("Added new asteroid");
                    }
                }
            }
        }
    }
}
