use crate::RenderedText;

pub struct InfoBox{
    id: u32,
    text: Vec<RenderedText>,
    screen_pos: (f32,f32),
    width: f32,
    height: f32,
    texture_id: u16,
    vertex_start: u32,
    vertex_end: u32,
}   