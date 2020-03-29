extends Node

func set_visible(state: bool):
    for child in get_children():
        child.set_visible(state)
