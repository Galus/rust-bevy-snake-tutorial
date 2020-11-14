use bevy::prelude::*;
use bevy::render::pass::ClearColor;

struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}

const ARENA_W: u32 = 10;
const ARENA_H: u32 = 10;
struct Position {
    x: i32,
    y: i32,
}
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

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
        .with(SnakeHead)
        .with(Position { x: 3, y: 3 })
        .with(Size::square(0.8));
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        println!(" sprite.size {}", sprite.size);
        println!(" sprite_size.width {}", sprite_size.width);
        println!(" sprite_size.height {}", sprite_size.height);
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_W as f32 * window.width() as f32,
            sprite_size.height / ARENA_H as f32 * window.height() as f32,
        );
        println!(
            "w {} / {} * {} ",
            sprite_size.width,
            ARENA_W,
            window.width()
        );
        println!(
            "h {} / {} * {} ",
            sprite_size.height,
            ARENA_H,
            window.height()
        );
        println!("new sprite.size {}", sprite.size);
    }
}

/*
Example:
....x.....
1234567890
x is at 5,
new position = x / 10 * window_width(200) - window_width / 2
new position = 5 / 10 * 200 - 200 / 2
new position = 0.5 * 200 - 200 / 2
new position = 100 - 200/2
new position = 100 - 100
new position = 0

translation starts from the center, but our coordinate system starts at bottom-left
so in order to get this offset correctly, we need to subtrat half the window width.
*/
fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_W as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_H as f32),
            0.0,
        );
    }
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<With<SnakeHead, &mut Position>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 2000,
            height: 2000,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup")
        .add_startup_system_to_stage("game_setup", spawn_snake.system())
        .add_system(snake_movement.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_plugins(DefaultPlugins)
        .run();
}
