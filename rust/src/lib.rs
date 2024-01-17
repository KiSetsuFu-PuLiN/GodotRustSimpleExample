use godot::engine::{ISprite2D, Sprite2D};
use godot::init::EditorRunBehavior;
use godot::prelude::*;
use std::f64::consts::PI;

/// 拓展入口，有此项的动态库才能被 *.gdextension 识别。
/// `MyExtensition`的名称无关紧要，只要有一个类实现了`ExtensionLibrary`即可。
struct MyExtensition;
/// `#[gdextension]`会将 特性的实现 转化为动态库开放的入口函数`gdext_rust_init`，Godot的 *.gdextension 需要这个。
/// 入口函数会自动为下述标有`#[derive(GodotClass)]`的类执行注册和清理工作。
#[gdextension]
unsafe impl ExtensionLibrary for MyExtensition {
    /// 决定此拓展将如何在Godot编辑器中运行。
    fn editor_run_behavior() -> EditorRunBehavior {
        // 忽略所有的 `#[class(tool)]` 标记。
        EditorRunBehavior::AllClasses;
        // 仅运行有 `#[class(tool)]` 标记的类。
        // 所有的类都会被注册，并且允许从 GDScript 进行调用。
        // 然而，虚函数生命周期函数(`_ready`, `_process`, `_physics_process`, ...) 并不会被调用，除非有 `#[class(tool)]` 标记。
        // 注意：_init 函数不在此列，因为 Godot 中的 _init 本质上就是构造函数。在编辑器中也是需要
        EditorRunBehavior::ToolClassesOnly
    }
    /// 决定此扩展的初始化等级（初始化时机）。
    /// 如果初始化时机早于 [`InitLevel::Scene`] 则需要重启引擎才能使拓展生效。
    fn min_level() -> InitLevel {
        // 最早的初始化时机，仅可以使用Godot的内置的基本值类型数据类型。
        InitLevel::Core;
        // 第二等级的初始化时机，仅可以使用基本值类型数据类型和 Godot内置的 服务器 类。
        InitLevel::Servers;
        // 第三等级的初始化时机，绝大多数类都可用。
        InitLevel::Scene;
        // 第四等级的初始化时机，仅在编辑器中才会加载。所有的类都可用。
        InitLevel::Editor
    }
    /// Custom logic when a certain init-level of Godot is loaded.
    ///
    /// This will only be invoked for levels >= [`Self::min_level()`], in ascending order. Use `if` or `match` to hook to specific levels.
    fn on_level_init(_level: InitLevel) {
        // Nothing by default.
    }
    /// Custom logic when a certain init-level of Godot is unloaded.
    ///
    /// This will only be invoked for levels >= [`Self::min_level()`], in descending order. Use `if` or `match` to hook to specific levels.
    fn on_level_deinit(_level: InitLevel) {
        // Nothing by default.
    }
}

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
    /// 需要注意 init 函数是不可避免在编辑器中被调用的，因为在 Godot 里，init 是 new 的别称
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
