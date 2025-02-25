use glium::{glutin::surface::WindowSurface, Display};


use super::{char_font_pos::CHAR_TO_TEX};

#[derive(Copy, Clone, Debug)]
pub struct TextAttr {
    world_position: [f32; 3],
    colour: [f32; 3], // Changed to array
    tex_offsets: [f32;4], //x offset, y offset, scaling factor          For reading in texture atlas
}
implement_vertex!(TextAttr, world_position, colour, tex_offsets);

#[derive(Clone, Debug)]
pub struct RenderedText{
    pub text: String,
    pub vertex_start: u32,
    pub vertex_end: u32,
}

impl RenderedText{

    pub fn new(text: String) -> RenderedText{
        RenderedText{
            text:text,
            vertex_start: 0,
            vertex_end: 0,
        }
    }

    pub fn change_text(&mut self, new_text: String){
        self.text = new_text;
    }

}

pub fn format_to_exact_length(number: u32, length: usize) -> String {
    let mut num_str = number.to_string();

    // Truncate if necessary
    if num_str.len() > length {
        num_str.truncate(length);
    }

    // Pad with the specified character if necessary
    if num_str.len() < length {
        num_str = format!("{:0width$}", number, width = length)
    }

    num_str
}

pub struct TextVbo{
    pub vbo: glium::VertexBuffer<TextAttr>,
    start: u32,
    end: u32,
}



impl TextVbo{
    pub fn new(max_chars: usize, display: &Display<WindowSurface>) -> TextVbo{
        
        //Make this be truly empty, by not using empty_dynamic...
        let mut empty_vec:Vec<TextAttr> = Vec::with_capacity(max_chars);
        for i in 0..max_chars{
            empty_vec.push(TextAttr { world_position: [0.0,0.0,0.0], colour: [0.0,0.0,0.0], tex_offsets: [0.0,0.0,0.0, 0.0] });
        }
        
        return TextVbo { 
            vbo: glium::vertex::VertexBuffer::dynamic(display, &empty_vec).unwrap(),
            start: 0,
            end: 0,
        }
    }

    pub fn replace_text(&mut self, text: &RenderedText){
        let mut vert_start = text.vertex_start as usize;
        let slice_for_char = self.vbo.slice_mut(vert_start..vert_start+text.text.len()).unwrap();
        let mut read_slice = slice_for_char.read().unwrap();
        for (i, char) in text.text.chars().enumerate(){
            if vert_start > text.vertex_end as usize{
                return;
            }
            let char_as_num = char as u8;
            let tex_coords = CHAR_TO_TEX[char_as_num as usize];
            //Maybe make this into a loop
            read_slice[i].tex_offsets = [tex_coords[0], tex_coords[1], 0.0625,0.125];
            vert_start += 1;
        }
        slice_for_char.write(&read_slice);
    }


    pub fn add_text(&mut self, start_ndc: (f32,f32), font_size: f32, color: Option<[f32;3]>,text: &mut RenderedText){
        let mut current_pos = start_ndc;
        let mut text_list: Vec<TextAttr> = vec![];
        let colour = color.unwrap_or([1.0,1.0,1.0]);
        text.vertex_start = self.start;
        for char in text.text.chars(){
            let char_as_num = char as u8;
            if char_as_num == 10{
                current_pos = (start_ndc.0, current_pos.1 - font_size);
                continue; 
            }
            let tex_coords = CHAR_TO_TEX[char_as_num as usize];

            let attr = TextAttr{
                world_position: [current_pos.0, current_pos.1 , font_size],
                colour: colour,
                tex_offsets: [tex_coords[0], tex_coords[1], 0.0625,0.125],
            };

            text_list.push(attr);

            current_pos = (current_pos.0 + font_size/3.0, current_pos.1);
        }

        self.vbo.slice_mut(self.start as usize..(self.start as usize + text.text.len())).unwrap().write(&text_list);
        self.start += text.text.len() as u32;
        self.end = self.start;
        text.vertex_end = self.end;

    }

    
    fn char_to_uv(char_as_num: u8) -> [[f32;2];4]{
        let bottom_left_tex = CHAR_TO_TEX[char_as_num as usize];
        //First one is bottom left, second one is bottom right, top right, top left
        let tex_coords: [[f32;2];4] = [bottom_left_tex, [bottom_left_tex[0]+0.0625, bottom_left_tex[1]], [bottom_left_tex[0]+0.0625, bottom_left_tex[1]+0.125], [bottom_left_tex[0], bottom_left_tex[1]+0.125]];
        return tex_coords;
    }
}