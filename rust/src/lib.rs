use godot::engine::{ISprite2D, Sprite2D};
use godot::prelude::*;
use std::f64::consts::PI;

/// 拓展入口，有此项的动态库才能被 *.gdextension 识别。
/// `MyExtensition`的名称无关紧要，只要有一个类实现了`ExtensionLibrary`即可。
struct MyExtensition;
/// `#[gdextension]`会将特性实现转化为动态库开放的入口函数`gdext_rust_init`，Godot的*.gdextension需要这个。
/// 入口函数会自动为下述标有`#[derive(GodotClass)]`的类执行注册和清理工作。
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
