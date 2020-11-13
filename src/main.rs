use bevy::prelude::*;

struct SnakeHead;
struct Materials { head_material: Handle<ColorMaterial>, }


/*
 * ColorMaterial .add returns a Handle<ColorMaterial>
 */

/* Bevy requires a specific ordering to the params when registering systems.
 * Commands → Resources → Components/Queries.
 * If you get a mysterious compile-time error after messing with a system,
 * check your order. */
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteComponents {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(SnakeHead);
}

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup")
        .add_startup_system_to_stage("game_setup", spawn_snake.system())
        .add_plugins(DefaultPlugins)
        .run();
}
