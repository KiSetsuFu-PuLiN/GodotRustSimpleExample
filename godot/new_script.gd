extends Control

func _ready():
	var a = RustResource.new()
	print(a.a())
	print(a.a)
	print(get_position)
	print(get_position())
	print(a.b())
	print(RustResource.b())
	RustResource.func(a)
	
	# 有两种方法访问由 Rust 在 Scene 初始化阶段创建的单例。
	# 其中一种是直呼其名，如下：
	RustObjectSingleton
	# 仅在Scene初始化时创建的单例可以这么获取，因此 GDScript 是做不到创建这种单例的。
	# 但是这样调用的变量会被视为是常量，无法对其字段和访问器进行赋值。如下：
	#RustObjectSingleton.texture = preload("res://icon.svg")
	# rust 和 GDScript 均可以使用 Engine.get_singleton 获取单例。并且这种单例是可变的。
	var singleton = Engine.get_singleton("RustObjectSingleton") as RustObject
	singleton.texture = preload("res://icon.svg")
	add_child(singleton)
