use godot::engine::{ISprite2D, Sprite2D};
use godot::prelude::*;
use std::f64::consts::PI;

struct MyExtensition;
/// 拓展入口，有此项的dll才能被Godot识别
#[gdextension]
unsafe impl ExtensionLibrary for MyExtensition {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    #[base]
    base: Base<Sprite2D>,
    speed: f64,
    angular_speed: f64,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, Rust!");
        return Self {
            base,
            speed: 400.0,
            angular_speed: PI,
        };
        unimplemented!();
    }
    fn physics_process(&mut self, delta: f64) {
        self.base.rotate((self.angular_speed * delta) as f32);
        // unimplemented!();
    }
}
