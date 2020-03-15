extends PopupPanel

onready var queen_button = $PawnOptionContainer/Queen
onready var rook_button = $PawnOptionContainer/Rook
onready var bishop_button = $PawnOptionContainer/Bishop
onready var knight_button = $PawnOptionContainer/Knight

signal piece_type_selected

func _ready():
    queen_button.connect("button_up", self, "_queen_selected")
    rook_button.connect("button_up", self, "_rook_selected")
    bishop_button.connect("button_up", self, "_bishop_selected")
    knight_button.connect("button_up", self, "_knight_selected")
    
func _queen_selected():
    emit_signal("piece_type_selected", "queen")
    
func _rook_selected():
    emit_signal("piece_type_selected", "rook")
    
func _bishop_selected():
    emit_signal("piece_type_selected", "bishop")
    
func _knight_selected():
    emit_signal("piece_type_selected", "knight")
