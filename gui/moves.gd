extends Tabs

const MOVE_ENTRY = preload("res://gui/MoveEntry.tscn")

onready var chess_director = owner.get_node("ChessDirector")
onready var move_container = $MarginContainer/ScrollContainer/VBoxContainer

func _ready():
    chess_director.connect("update_moves", self, "update_moves")
# wackyness to get around deadlock here
func update_moves(turns = -1):
    for child in move_container.get_children():
        child.queue_free()
    var num_turns
    if turns == -1:
        num_turns = chess_director.get_num_turns()
    else:
        num_turns = turns
    for num in range(num_turns):
        var move_entry = MOVE_ENTRY.instance()
        move_entry.turn_num = num
        move_container.add_child(move_entry)
