extends Control

func _ready():
	var a = RustRefCounted.new()
	print(a.a())
	print(a.a)
	print(get_position)
	print(get_position())
	print(a.b())
	print(RustRefCounted.b())
