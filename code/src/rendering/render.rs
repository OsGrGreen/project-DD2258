/* use glium::glutin::api::egl::display;
use glium::Display;
use glium::{glutin::surface::WindowSurface};
use winit::event_loop::EventLoop;
use winit::window::Window; */


use glam::Mat4;
use glium::{glutin::surface::WindowSurface, program::TransformFeedbackMode, uniforms::{AsUniformValue, Uniforms, UniformsStorage}, Display, DrawParameters, Frame, Program, Surface, VertexBuffer};

use super::{text::RenderedText};

#[derive(Copy, Clone,Debug)]
pub struct VertexSimple {
    pub w_position: [f32; 3],
}
implement_vertex!(VertexSimple, w_position);


#[derive(Copy, Clone,Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2], 
}
implement_vertex!(Vertex, position,normal,tex_coords);

pub struct Renderer<'b>
    {
        // What to be rendered (Verts)
        // Make these be able to render any Vertex type again since saving normal and tex_coords for line is kina overkill...
        // pub vbo: VertexBuffer<T>
        pub vbo: VertexBuffer<Vertex>,
        // How to be rendered (Indicies) (Kan behöva öka från u16 till u32)
        pub indicies: glium::IndexBuffer<u16>,
        // The program to render it
        pub program: Program,
        pub draw_params: DrawParameters<'b>,
        pub is_dynamic: bool,
        used_vbo: usize,
        used_inds: usize,
        // Specific Uniforms (Känns kanske lite svårt att spara på ett bra sätt här?)
        // Texture
}

impl <'b>Renderer<'b>{    
        pub fn new<'a>(shape: &Vec<Vertex>, inds: &Vec<u16>, prim_type: Option<glium::index::PrimitiveType> ,vert_shader: &'a str, frag_shader: &'a str, geo_shader: Option<&'a str>, tess_ctrl: Option<&'a str>, tess_eval: Option<&'a str>, disp: &Display<WindowSurface>, params: Option<DrawParameters<'b>>, transformFeedback:Option<(Vec<std::string::String>, TransformFeedbackMode)> ) -> Result<Renderer<'b>, &'a str>{
            let shape_len = shape.len();

            let vbo = glium::VertexBuffer::new(disp, &shape).unwrap();
            let program = glium::Program::new(disp, glium::program::ProgramCreationInput::SourceCode{
                vertex_shader: vert_shader,
                fragment_shader: frag_shader,
                geometry_shader: geo_shader,
                tessellation_control_shader: tess_ctrl,
                tessellation_evaluation_shader: tess_eval,
                transform_feedback_varyings: transformFeedback,
                outputs_srgb: true,
                uses_point_size: false,
            }).unwrap();

            

            if inds.len() < 1{
                let mut inds = vec![];
                for n in (0..shape_len).step_by(3){
                    inds.push(n as u16);
                    inds.push(((n+1)%shape_len) as u16);
                    inds.push(((n+2)%shape_len) as u16);
                }
                let indicies = glium::IndexBuffer::new(disp,prim_type.unwrap_or(glium::index::PrimitiveType::TrianglesList),
                &inds).unwrap();

                Ok(Renderer{
                    vbo: vbo.into(),
                    indicies: indicies,
                    program: program,
                    is_dynamic: false,
                    draw_params: params.unwrap_or(Default::default()),
                    used_vbo: 0,
                    used_inds: 0,
                })
            }else{
                let indicies = glium::IndexBuffer::new(disp,prim_type.unwrap_or(glium::index::PrimitiveType::TrianglesList),
                &inds).unwrap();

                Ok(Renderer{
                    vbo: vbo.into(),
                    indicies: indicies,
                    program: program,
                    is_dynamic: false,
                    draw_params: params.unwrap_or(Default::default()),
                    used_vbo: 0,
                    used_inds: 0,
                })
            }
        }

        pub fn new_dynamic<'a>(shape: Vec<Vertex>, inds: Vec<u16>, prim_type: Option<glium::index::PrimitiveType> ,vert_shader: &'a str, frag_shader: &'a str, geo_shader: Option<&'a str>, disp: &Display<WindowSurface>, params: Option<DrawParameters<'b>>) -> Result<Renderer<'b>, &'a str>{
            let shape_len = shape.len();

            let vbo = glium::VertexBuffer::dynamic(disp, &shape).unwrap();

            let program = glium::Program::from_source(disp, vert_shader, frag_shader, geo_shader).unwrap();

            if inds.len() < 1{
                //println!("Found no indecies");
                let mut inds = vec![];
                for n in (0..shape_len).step_by(3){
                    //println!("Pushing: {}, {}, {}", n, (n+1)%shape_len,(n+2)%shape_len);
                    inds.push(n as u16);
                    inds.push(((n+1)%shape_len) as u16);
                    inds.push(((n+2)%shape_len) as u16);
                }
                let indicies = glium::IndexBuffer::dynamic(disp,prim_type.unwrap_or(glium::index::PrimitiveType::TrianglesList),
                &inds).unwrap();

                Ok(Renderer{
                    vbo: vbo.into(),
                    indicies: indicies,
                    program: program,
                    is_dynamic: true,
                    draw_params: params.unwrap_or(Default::default()),
                    used_vbo: 0,
                    used_inds: 0,
                })
            }else{
                let indicies = glium::IndexBuffer::dynamic(disp,prim_type.unwrap_or(glium::index::PrimitiveType::TrianglesList),
                &inds).unwrap();

                Ok(Renderer{
                    vbo: vbo.into(),
                    indicies: indicies,
                    program: program,
                    is_dynamic: true,
                    draw_params: params.unwrap_or(Default::default()),
                    used_vbo: 0,
                    used_inds: 0,
                })
            }
        }

        pub fn new_empty_dynamic<'a>(max_elements: usize, prim_type: Option<glium::index::PrimitiveType> ,vert_shader: &'a str, frag_shader: &'a str, geo_shader: Option<&'a str>, disp: &Display<WindowSurface>, params: Option<DrawParameters<'b>>) -> Result<Renderer<'b>, &'a str>{

            let vbo:VertexBuffer<Vertex> = glium::VertexBuffer::empty_dynamic(disp, max_elements).unwrap();

            let program = glium::Program::from_source(disp, vert_shader, frag_shader, geo_shader).unwrap();
            let mut empty_vec:Vec<u16> = Vec::with_capacity(max_elements*3);
            for i in 0..max_elements*3{
                empty_vec.push(0);
            }
            let indicies = glium::IndexBuffer::dynamic(disp, prim_type.unwrap_or(glium::index::PrimitiveType::TrianglesList), &empty_vec).unwrap();
            indicies.invalidate();
            Ok(Renderer{
                    vbo: vbo.into(),
                    indicies: indicies,
                    program: program,
                    is_dynamic: true,
                    draw_params: params.unwrap_or(Default::default()),
                    used_vbo: 0,
                    used_inds: 0,
            })
        }

        pub fn draw<T, R>(&self, frame: &mut Frame, draw_parameters: Option<&DrawParameters>, uniforms: Option<&UniformsStorage<T, R>>)
        where
        T: AsUniformValue,
        R: Uniforms,
        {
            if uniforms.is_some(){
                frame.draw(&self.vbo, &self.indicies, &self.program, uniforms.unwrap(),
                draw_parameters.unwrap_or(&Default::default())).unwrap();
            }else{
                frame.draw(&self.vbo, &self.indicies, &self.program, &glium::uniforms::EmptyUniforms,
                    draw_parameters.unwrap_or(&Default::default())).unwrap();
            }
                
        }

        pub fn add_part_vao(&mut self, new_vertices: Vec<Vertex>, new_indicies: Vec<u16>){

            //Only allow vao to be modified if VAO is dynamic
        
            if self.is_dynamic{

                // Can probably be done in a nicer way but works for now

                let end_new_vbo = new_vertices.len()+self.used_vbo;
                let end_new_indicies = new_indicies.len()+self.used_inds;
                if end_new_vbo > self.vbo.len() || end_new_indicies > self.indicies.len(){
                    return;
                }
                let update_slice_vbo = self.vbo.slice_mut(self.used_vbo..end_new_vbo).unwrap();
                update_slice_vbo.write(&new_vertices);
                let update_slice_indicies = self.indicies.slice_mut(self.used_inds..end_new_indicies).unwrap();
                let mut fixed_inds = vec![];
                for inds in new_indicies{
                    fixed_inds.push(inds+self.used_vbo as u16);
                }
                update_slice_indicies.write(&fixed_inds);
                self.used_vbo = end_new_vbo;
                self.used_inds = end_new_indicies;
            }else{
                return
            }
        }

        pub fn draw_line(&mut self, start_ndc: (f32,f32), end_ndc: (f32,f32), color: Option<[f32;3]>){
    
            let new_vertices: Vec<Vertex> = vec![Vertex{position: [start_ndc.0, start_ndc.1, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: [0.0, 0.0]}, Vertex{position: [end_ndc.0, end_ndc.1, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: [0.0, 0.0]}];
            let new_indicies = vec![0, 1];
        
            self.add_part_vao(new_vertices, new_indicies);
        
        }
        
        pub fn draw_rectangle(&mut self, start_ndc: (f32,f32), width:f32,height:f32, color: Option<[f32;3]>){
        
            let new_vertices: Vec<Vertex> = vec![
                    Vertex{position: [start_ndc.0, start_ndc.1, 0.0], normal: color.unwrap_or([1.0,0.0,0.0]), tex_coords: [0.0, 0.0]}, 
                    Vertex{position: [start_ndc.0+width, start_ndc.1, 0.0], normal: color.unwrap_or([0.0,1.0,0.0]), tex_coords: [0.0, 0.0]},
                    Vertex{position: [start_ndc.0+width, start_ndc.1+height, 0.0], normal: color.unwrap_or([0.0,0.0,1.0]), tex_coords: [0.0, 0.0]},
                    Vertex{position: [start_ndc.0, start_ndc.1+height, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: [0.0, 0.0]}];
            let new_indicies = vec![0, 2, 1, 0, 2, 3];
        
            self.add_part_vao(new_vertices, new_indicies);
        }

        pub fn draw_rectangle_with_texture(&mut self, start_ndc: (f32,f32), width:f32,height:f32, color: Option<[f32;3]>, texture_id: u32){
            
            let texture_coords = rectangle_texture_id_to_texture_coords(texture_id);

            let new_vertices: Vec<Vertex> = vec![
                    Vertex{position: [start_ndc.0, start_ndc.1, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: texture_coords[0]}, 
                    Vertex{position: [start_ndc.0+width, start_ndc.1, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: texture_coords[1]},
                    Vertex{position: [start_ndc.0+width, start_ndc.1+height, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: texture_coords[2]},
                    Vertex{position: [start_ndc.0, start_ndc.1+height, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: texture_coords[3]}];
            let new_indicies = vec![0, 2, 1, 0, 2, 3];
        
            self.add_part_vao(new_vertices, new_indicies);
        }

        pub fn draw_rectangle_with_specific_texture(&mut self, start_ndc: (f32,f32), width:f32,height:f32, color: Option<[f32;3]>, texture_coords: [[f32;2];4]){
        
            let new_vertices: Vec<Vertex> = vec![
                    Vertex{position: [start_ndc.0, start_ndc.1, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: texture_coords[0]}, 
                    Vertex{position: [start_ndc.0+width, start_ndc.1, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: texture_coords[1]},
                    Vertex{position: [start_ndc.0+width, start_ndc.1+height, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: texture_coords[2]},
                    Vertex{position: [start_ndc.0, start_ndc.1+height, 0.0], normal: color.unwrap_or([1.0,1.0,1.0]), tex_coords: texture_coords[3]}];
            let new_indicies = vec![0, 2, 1, 0, 2, 3];
        
            self.add_part_vao(new_vertices, new_indicies);
        }

        pub fn remove_subset(&mut self, vertex_start: u32, index_start:u32){

        }
        
        //Implement this...
        pub fn remove_text(&mut self, text: RenderedText){

        }
}  

fn rectangle_texture_id_to_texture_coords(id:u32) -> [[f32;2];4]{
    return [[0.0,1.0],[1.0,0.0],[1.0,1.0],[0.0,0.1]]
}

pub fn calculate_perspective(dim: (f32, f32)) -> Mat4{
    let perspective = {
        let (width, height) = dim;
        let aspect_ratio = height as f32 / width as f32;
        
        let fov: f32 = std::f32::consts::PI / 4.0;
        let zfar = 100.0;
        let znear = 0.1;
    
        let f = 1.0 / (fov / 2.0).tan();
        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    };

    return Mat4::from_cols_array_2d(&perspective)
}


const UV_HEX: [(f32,f32);6] =  [(1.0,0.366),(0.866,0.866),(0.366,1.0),(0.0,0.634),(0.134,0.134),(0.634,0.0)];


