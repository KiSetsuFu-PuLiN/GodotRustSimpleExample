extends Control

func _ready():
	var a = RustResource.new()
	print(a.a())
	print(a.a)
	print(get_position)
	print(get_position())
	print(a.b())
	print(RustResource.b())
	RustResource.func(a);
