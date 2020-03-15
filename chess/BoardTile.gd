extends Area

export var tile = PoolIntArray([0, 0])

func get_tile() -> PoolIntArray:
    return tile

func get_pos() -> Vector3:
    return get_translation()+$CollisionShape.get_translation()
    
func get_piece() -> Node:
    var overlapping_bodies = get_overlapping_bodies()
    if overlapping_bodies.size() > 0:
        return overlapping_bodies[0]
    return null
