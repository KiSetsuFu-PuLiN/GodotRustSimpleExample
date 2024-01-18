use godot::engine::{ISprite2D, Sprite2D};
use godot::init::EditorRunBehavior;
use godot::prelude::*;

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
        // 注意：_init 函数不在此列，因为 Godot 中的 _init 本质上就是构造函数。在编辑器中也是需要先 构造出来 然后才能显示在节点树上。
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
        // 第四等级的初始化时机。所有的类都可用，但需要注意 Godot 的有些类仅在编辑器下可用。
        InitLevel::Editor
    }
    /// 自定义扩展初始化时的行为。
    /// 在引擎启动、扩展被初始化时，此函数可能根据引擎的初始化等级而调用多次（ 4 次）， _level 正是当前引擎的初始化等级，扩展的初始化时机比引擎稍晚一些。
    fn on_level_init(_level: InitLevel) {
        // 默认啥都不干
    }
    /// 自定义扩展解初始化（析构）时的行为。
    /// 在引擎启动、扩展被初始化时，此函数可能根据引擎的初始化等级而调用多次（ 4 次）， _level 正是当前引擎的初始化等级，扩展的析构时机比引擎稍早一些。
    fn on_level_deinit(_level: InitLevel) {
        // 默认啥都不干
    }
}

/// `#[derive(GodotClass)]`是连接 Rust 字段和 Godot 的主要桥梁。
/// 请看 GodotClass 的宏定义：
/// ```no_run
/// #[proc_macro_derive(GodotClass, attributes(class, base, var, export, init, signal))]
/// pub fn derive_godot_class(input: TokenStream) -> TokenStream {
///     translate(input, class::derive_godot_class)
/// }
/// ```
/// 此处`proc_macro_derive`声明了使用`#[derive(GodotClass)]`类标记时，被标记的类声明会被丢给`derive_godot_class`进行处理转换。
/// `attributes`为被 derive 标记的类提供了一些额外可用的辅助处理标记，如 class, base, var, export, init, signal 等等。
#[derive(GodotClass)]
/// 使用`#[class(base=SomeGodotClass)]`标志使类继承于`godot::engine::*`中的任意类`SomeGodotClass`，否则会自动继承`RefCounted`。
/// 然后可以手动实现`ISomeGodotClass`的特性来编写其作为 Godot 对象的各项回调。
#[class(base=Sprite2D)]
struct RustObject {

    /// `#[base]`标志表示该字段是结构体继承自`Sprite2D`的基类型对象。
    /// 打上此标记来让帮助宏将基类型的数据写入到该字段中，这在使用`#[class(init)]`标志时是必要的。(对于现在这种手写`init`函数的情况是不必要的)。
    /// rust 没有继承的概念，所以对基类型方法和字段的使用只能通过这种方式，这里的`base`字段中包含了所有的基类型字段和方法。
    //#[base]
    base: Base<Sprite2D>,
    
    /// 没有任何标志的字段仅可在 Rust 内部使用。
    temp:f64,
    
    /// `#[var]`标志表示该字段可以暴露给GDScript进行操纵。
    #[var]
    speed: f64,
    
    /// `#[export]`标志表示该字段可以在编辑器的检视器中显示并进行值的设置，设置会在 init 之后且与默认值有差异时进行。
    /// 此标志也会默认实现`#[var]`标志。
    #[export]
    angular_speed: f64,

}

/// `#[godot_api]`是连接 Rust 函数和 Godot 的主要桥梁。
/// `ISprite2D`仅仅只是提示了可以写哪些函数，虽然`ISprite2D`内部有一些默认的看起来会报错的`unimplemented!()`实现，
/// 但实际上这些默认实现全都会被`#[godot_api]`优化掉，从而根本不会被 Godot 调用。
/// 在此处覆盖实现的函数则会被`#[godot_api]`暴露给引擎，像 GDScript 中虚函数那样正常工作。
#[godot_api]
impl ISprite2D for RustObject {

    /// 需要注意 init 函数是不可避免在编辑器中被调用的，因为在 Godot 里，init 就是 new 的别称。添加场景节点以及文档等均会用到 init 来创建实例。
    /// 如果不实现这个 init 函数的话，这个类会被认为是抽象类，不可构造，只能继承。
    /// 实现此函数后，函数返回的本结构体的各个字段的值即该字段在 Godot 中的默认值。
    /// 并且别忘了，`#[export]`可能（仅当检视器存储的值与默认值有差异时）会在 init 之后对字段进行修改。
    fn init(base: Base<Sprite2D>) -> Self {
        return Self {
            base,
            temp:12.,
            speed: 400.0,
            angular_speed: 3.14,
        };
    }

    /// 这里就实现了一个被 Godot 调用的虚函数。
    fn physics_process(&mut self, delta:f64){
        self.base.rotate((delta*self.angular_speed) as f32);
    }

}

#[derive(GodotClass)]
/// 使用`#[class(init)]`标志使类不需要手动实现对应的接口及其`init`函数也可以在 Godot 中`new`出来，相当于默认`init`实现。
#[class(init, base = Resource)]
struct RustRefCounted{

    #[var]
    /// 可以使用`#[init(default = xxxx)]`来跳过`init`函数为字段设置构造初始值，没有该标记的字段会使用 Godot 对应类型的默认值。
    /// 但需要注意的是，如果同时存在`init`函数实现的话，则#[init]标志的初始值会被`init`函数实现所覆盖。
    #[init(default = 12)]
    a:i64,

    /// Godot 中的 export 变种可以通过此种方法实现。
    #[export(range=(0.0,100.1))]
    b:i64,

    /// 使用`#[var(get = get_pro,set = set_pro)]`来将字段改造为访问器，绑定了用于访问和设置字段的方法。
    /// 需要注意绑定的方法需要同样有`#[func]`标志，不然绑定会失败、本字段无法在 Godot 中被看到。
    #[var(get = get_pro,set = set_pro)]
    pro:i64,

}

/// `#[godot_api]`还提供`#[func]`辅助标志用于将方法暴露出来。
#[godot_api]
impl RustRefCounted {

    /// `#[func]`标志用于将此函数暴露给 GDScript 进行操作。
    #[func]
    /// 请注意这里出现了函数名和字段同名的现象而且没报错，尽管Rust有独特的鱼涡轮和闭包处理方法，但 Godot 只是会单纯会让方法名的访问被变量覆盖，这一点需要避免。
    fn a(&mut self) -> i64{
        1234
    }
    
    #[func]
    /// 如果第一个参数不是self的话，函数也会被 Godot 视为静态函数。
    /// 尽管大多数语言都不允许通过对象来调用静态函数，但是不巧 Godot 可以同时通过类和对象调用静态函数，这一点请多加注意。
    fn b() -> i32{
        2345
    }

    /// 由于这两个函数已经被绑定为`pro`字段的访问器，因此即便有`#[func]`声明，也不会在类的文档中作为正式的方法被提名。
    #[func]
    fn get_pro(&self)->i64{1234}
    #[func]
    fn set_pro(&mut self, value:i64)->(){self.pro = value}

    /// 将此函数实现为一个信号并暴露给 Godot 。
    #[signal]
    fn custon_signal();

    /// 设置一个常数并暴露给 Godot 。(rust的常数只能写在特性或函数里)
    #[constant]
    const CUSTOM_CONST:i32 = 999;

    /// rust 与 Godot 进行非值类型的自定义类对象的交互的时候，只能通过`Gd<..>`指针进行。
    /// 被`Gd<..>`指针包裹的对象实际上还仍然由 Godot 引擎进行托管， rust 并不持有其所有权。
    /// 在尝试向 Godot 传递对象类型或接受来自 Godot 的对象时需要考虑这点。
    #[func]
    fn new()->Gd<RustRefCounted>{Gd::<RustRefCounted>::from_object(RustRefCounted { a:0, b: 0, pro: 0 })}

    //tool
    //return self, GetOther.

}
