enum ButtonType{
    Close,
    Open,
}

pub struct Button{
    pos: (f32, f32),
    texture_od: u16,
    button_id: u16,
    button_type: ButtonType
}