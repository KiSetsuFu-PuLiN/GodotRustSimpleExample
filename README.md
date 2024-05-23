# 简单的 Godot - Rust 范例和注释

- 看`Cargo.toml`，导入依赖库并设置rust的编译方式。

- 看`hello_world.gdextension`，将rust编译产生的类添加到Godot中。

- 看`lib.rs`，其中包含了使用rust编写Godot类的说明注释。

- 看`launch.json`，其中包含调试配置。

## Android 构建方法

```

Ok, I can at least give a rough description on how I made it work locally (Linux/Arch):
Windows下也可使用

1. 安装系统依赖

按照教程指示安装 android-sdk(Android Studio)、jdk等普通导出所需的依赖。

jdk 安装时需设置 JAVA_HOME 环境变量

安装 Rust 的 cargo ndk:

cargo install cargo-ndk

2. 安装 rust 构建目标

rustup target add aarch64-linux-android

3. 为 Godot 构建动态链接库

(每次改完 Rust 代码后都需要重复这一步)

- Debug 构建
- - env ANDROID_NDK_HOME=/path/to/android-ndk
- - cargo ndk -t arm64-v8a build
- Release 构建
- - env ANDROID_NDK_HOME=/path/to/android-ndk
- - cargo ndk -t arm64-v8a build --release

NOTE: If you are on a different distro, or you installed the ndk through Android Studio, you'll need to change the path to point to your correct ndk!

NOTE：记得改 NDK 的路径。

4. 为 apk 包生成一个签名密钥

keytool -keyalg RSA -genkeypair -alias androiddebugkey -keypass android -keystore debug.keystore -storepass android -dname "CN=Android Debug,O=Android,C=US" -validity 9999 -deststoretype pkcs12

Note: I'm not quite sure which package keytool is from, it may not necessarily be installed by only installing the dependencies mentioned above.

5. 为 Godot 编辑器配置 Android 环境

照着官方安卓导出教程就行

- Top Bar
- Editor
- Editor Settings
- Search for "Android" in the search bar
- Click on Export->Android in the left pane
- Set SDK path to /home/USER/Android/Sdk
- - You need Android Studio for this! - Don't forget to install the correct SDK
- Set the debug keystore to the file you created in the previous step with keytool (debug.keystore)
- Set user to "androiddebugkey"
- Set password to "android"
- Close the Settings

6. 为项目安装 Android 构建模板

- Top Bar
- Project
- Create Android-Build-Template
- follow the Wizard

7. 将 Android 构建目标添加到 .gdextensions 文件。

In my case it looks like this:

[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1

[libraries]
linux.debug.x86_64     = "res://../rust/target/debug/librust_godot.so"
linux.release.x86_64   = "res://../rust/target/release/librust_godot.so"
                            
windows.debug.x86_64   = "res://../rust/target/debug/rust_godot.dll"
windows.release.x86_64 = "res://../rust/target/release/rust_godot.dll"
                            
macos.debug            = "res://../rust/target/debug/librust_godot.dylib"
macos.release          = "res://../rust/target/release/librust_godot.dylib"

macos.debug.arm64      = "res://../rust/target/debug/librust_godot.dylib"
macos.release.arm64    = "res://../rust/target/release/librust_godot.dylib"

android.release.arm64  = "res://../rust/target/aarch64-linux-android/debug/librust_godot.so"
android.debug.arm64    = "res://../rust/target/aarch64-linux-android/release/librust_godot.so"

8. 重新加载 Godot 项目

You are either prompted to reload the project,
or just use TopBar->Project->Reload, or just close Godot and re-open it.

9. 导出 apk

-> Top Bar
-> Project
-> Export

- Select Android
- Rename your project acc. to android rules (otherwise you'll get an error)
- Click "Export Project"

You should now be left with an apk file in the godot/build folder.
You can copy this apk to an end device and it should work.

Note that Android Emulators which use x86/64-Android will not work for executing this. - We didn't compile in the necessary targets. - Though I'm sure you could add them somehow.

注意：构建出来的库仅能供 arm64 架构的 Android 手机使用，使用 x86/64 架构的安卓模拟器将无法运行这个。

```