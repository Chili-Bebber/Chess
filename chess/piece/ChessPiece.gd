extends RigidBody

export var is_white = true
export var piece_type = ""
export var interp_curve: Curve
var is_active = false
var is_selected = false
onready var start = get_tile_position()
var dest: PoolIntArray
var start_pos: Vector3
var dest_pos: Vector3
var is_dragged = false
var is_moving = false
var emit_signals = true
var slide_speed = DEFAULT_SPEED
var slide_height = DEFAULT_HEIGHT
var is_captured = false
var is_promoted = false
onready var mouse_input = get_node("/root/MouseInput")
onready var chess_game = get_parent().get_parent()
const WHITE_MATERIAL = preload("res://chess/models/pieces/white_piece.tres")
const BLACK_MATERIAL = preload("res://chess/models/pieces/black_piece.tres")

var time = 0.0

const DEFAULT_SPEED = 1.0
const DEFAULT_HEIGHT = 4.0

signal moved
signal moved_internal

func _ready():
    connect("moved", get_parent(), "_piece_moved")
    connect("mouse_entered", self, "_mouse_entered")
    connect("mouse_exited", self, "_mouse_exited")
    mouse_input.connect("drag", self, "_drag")
    mouse_input.connect("drop", self, "_drop")
    mouse_input.connect("click", self, "_click")
    
    $trans/shadow.set_translation(get_translation())
    
func set_piece_type(type: String):
    piece_type = type
    $MeshInstance.set_mesh(load("res://chess/models/pieces/%s.tres" % type))
func set_is_white(is_white: bool):
    self.is_white = is_white
    if is_white:
        $MeshInstance.set_surface_material(0, WHITE_MATERIAL)
    else:
        $MeshInstance.set_rotation(Vector3(0, PI, 0))
        $MeshInstance.set_surface_material(0, BLACK_MATERIAL)
    
func set_fields(is_white: bool, type: String):
    set_piece_type(type)
    set_is_white(is_white)
    
# if statements in below functions make sure that if you start dragging
# a piece and move the mouse off it before the drag timer ends, the drag
# is still registered
func _mouse_entered():
#    if not Input.is_action_pressed("click"):
    is_active = true
func _mouse_exited():
    is_active = false
   
# pick up the piece 
func _drag():
    if is_active and not is_moving and is_white == chess_game.is_white_turn:
        get_parent().owner.get_node("GUI").set_actions_enabled(false)
        is_dragged = true
        is_moving = true
        start = get_tile_position()
        set_process(true)
# drop the piece
func _drop():
    if is_dragged:
        if not chess_game.animation_enabled:
            # must set these values here because they are set inside of the
            # slide function normally, but if animation is disabled, the
            # move function just sets translation and exits without calling slide
            is_moving = false
            set_process(false)
        time = 0.0
        is_dragged = false
        is_active = false
        dest = get_tile_position()
        chess_game.try_move_drag(self, start, dest)
# highlight the piece
func _click():
    if is_active and not is_moving and is_white == chess_game.is_white_turn:
        if is_selected:
            is_selected = false
        else:
            is_selected = true
# tell the piece to move from one place to another
func move(dest: Vector3, speed = DEFAULT_SPEED, height = DEFAULT_HEIGHT, emit_signal = true):
    if chess_game.animation_enabled:
        time = 0.0
        emit_signals = emit_signal
        start_pos = get_translation()
        slide_speed = speed
        slide_height = height
        dest_pos = dest
        is_moving = true
        set_process(true)
    else:
        set_translation(dest)
        if emit_signal:
            emit_signal("moved")
        $trans/RayCast.set_translation($RemoteTransform.get_translation()+get_translation())
        $trans/RayCast.force_raycast_update()
        $trans/shadow.set_translation($trans/RayCast.get_collision_point())
        $trans/shadow.modulate.a = (20.547 - get_translation().y) / 20.0
# tell the piece to send itself to the stratosphere
func capture():
    if chess_game.animation_enabled:
        time = 0.0
        start_pos = get_translation()
        emit_signals = false
        dest_pos = Vector3(0, 20, 0)
        is_moving = true
        is_captured = true
        slide_speed = 0.4
        set_process(true)
    else:
        $trans/shadow.modulate.a = 0
        set_translation(Vector3(0, 20, 0))
        
func promote(new_type: String):
    var tile_pos = get_tile_position()
    var tile_vec = Vector2(tile_pos[0], tile_pos[1])
    var tile_loc = get_tile().get_pos()
    move(get_translation()+Vector3(0.1, 9, 0), 0.4, DEFAULT_HEIGHT, false)
    yield(self, "moved_internal")
    set_piece_type(new_type)
    set_is_white(is_white)
    chess_game.get_node("ChessDirector").upgrade_pawn(tile_vec, new_type)
    move(start_pos)

# get the square below this piece
func get_tile_position() -> PoolIntArray:
    var tile = get_tile()
    if tile and tile.is_in_group("board_tile"):
        return tile.get_tile()
    return PoolIntArray([-1, -1])
# get the tile this piece is sitting on
func get_tile() -> Node:
    $trans/RayCast.force_raycast_update()
    var tile = $trans/RayCast.get_collider()
    if tile and tile.is_in_group("board_tile"):
        return tile
    return null

# _process disabled when not needed
func _process(delta):
    $trans/RayCast.force_raycast_update()
    $trans/shadow.set_translation($trans/RayCast.get_collision_point())
    $trans/shadow.modulate.a = (20.547 - get_translation().y) / 20.0
    if is_dragged:
        # slide the piece to the mouse position
        if chess_game.animation_enabled:
            slide(
                delta, 
                get_translation(), 
                chess_game.drag_point, 
                chess_game.drag_point.y)
        else:
            set_translation(chess_game.drag_point)
    elif is_moving:
        slide(
            delta,
            start_pos,
            dest_pos,
            slide_height,
            false,
            true)
        if is_captured and get_translation().y > 8:
            queue_free()
            emit_signal("moved")
    # if the piece is processing physics, keep its shadow updating
    elif get_mode() != MODE_RIGID:
        set_process(false)

# slide the piece from one location to another          
func slide(
    delta: float, 
    from: Vector3, 
    to: Vector3, 
    height: float, 
    is_dragging = true, 
    drop_at_dest = false):
    if height < to.y:
        height = to.y
    var pos = get_translation()
    var dist = (pos-to).length()
    if dist < 0.01 and not is_dragged:
        is_moving = false
        time = 0.0
        slide_speed = DEFAULT_SPEED
        if emit_signals:
            emit_signal("moved")
        else:
            emit_signals = true
        emit_signal("moved_internal")
        return
    if pos.x != to.x or pos.z != to.z:
        if pos.y < height-0.1:
            time += delta
            set_translation(
                from.linear_interpolate(
                    Vector3(from.x, height, from.z), interp(time*2.0*slide_speed)))
            if get_translation().y >= height:
                time = 0.0
        else:
            time += delta
            var interp_time = 0.0
            # if this is not done, the piece will interpolate its position
            # at values greater than 1 and will begin shaking/teleporting
            if is_dragging:
                interp_time = delta*9.0
            else:
                interp_time = time*0.3
            var dest_pos = Vector3(to.x, height, to.z)
            if (pos-dest_pos).length() > 0.01:
                set_translation(
                    get_translation().linear_interpolate(
                        dest_pos, interp_time*slide_speed))
            else:
                set_translation(dest_pos)
                time = 0.0
    # set the piece back down at its destination
    elif drop_at_dest:
        time += delta
        set_translation(
            Vector3(to.x, height, to.z).linear_interpolate(
                to, interp(time*2.0*slide_speed)))
        
func interp(time: float) -> float:
    return interp_curve.interpolate(time)
