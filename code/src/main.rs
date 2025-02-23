#[macro_use]
extern crate glium;
extern crate winit;
use object::{point::WorldPoint, WorldObject};
use rand::distr::{Distribution, Uniform};
use glam::{Mat4, Vec2, Vec3};
use util::{input_handler::InputHandler, ray_library::{distance_ray_point, ndc_to_direction, ndc_to_intersection, ndc_to_point, world_to_pixel}};
use winit::{event::{MouseButton, MouseScrollDelta}, event_loop::{ControlFlow, EventLoop}, keyboard, raw_window_handle::HasWindowHandle, window::{Fullscreen, Window}};
use glium::{glutin::surface::WindowSurface, implement_vertex, index::PrimitiveType, uniforms::{MagnifySamplerFilter, MinifySamplerFilter}, Display, Surface, VertexBuffer};
use world::{hex::Hex, layout::{HexLayout, Point, EVEN}, offset_coords::{qoffset_from_cube_offsets, qoffset_to_cube, qoffset_to_cube_offsets}, tile::Tile, world_camera::WorldCamera, OffsetTile, NUM_COLMS, NUM_ROWS};
use std::{io::stdout, time::Instant};


mod object;
mod rendering;
mod bezier_surface;
use bezier_surface::{create_surface};
use rendering::{render::{array_to_vbo, Vertex, VertexSimple}, render_camera::RenderCamera, text::{format_to_exact_length, RenderedText, TextVbo}};


mod util;
mod world;
mod UI;


#[derive(Copy, Clone, Debug)]
pub struct Attr {
    world_position: [f32; 3],
    colour: [f32; 3], // Changed to array
    tex_offsets: [f32;3], //x offset, y offset, scaling factor          For reading in texture atlas
}
implement_vertex!(Attr, world_position, colour, tex_offsets);

impl Attr{
    pub fn is_zero(&self) -> bool{
        if self.colour == [0.0,0.0,0.0]{
            return true
        }else{
            return false
        }
    }
}

fn init_window()-> (EventLoop<()>, Window, Display<WindowSurface>) {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building"); 
    
    event_loop.set_control_flow(ControlFlow::Poll);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_title("4X4D-WIP").build(&event_loop);
    
    (event_loop, window, display)
}

//Camera constants

const CAMERA_SPEED:f32 = 2.0;

const CONSTANT_FACTOR:f32 = 1.0;
fn main() {


    let mut camera = RenderCamera::new(Vec3::new(0.0,0.0,4.5), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0,0.0,-1.0));

    // Input handler

    let mut input_handler = InputHandler::new();

    camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
    //println!("camera matrix glm is {:#?}", RenderCamera::look_at_glm(Vec3::new(2.0,-1.0,1.0), Vec3::new(-2.0,1.0,1.0),Vec3::new(0.0,1.0,0.0)));
    //println!("camera matrix is: {:#?}", camera.camera_matrix);
    // 1. The **winit::EventLoop** for handling events.
    let (event_loop, window, display) = init_window();
    // Check if windows then: 
    //window.set_window_icon(window_icon);
    let monitor_handle = window.primary_monitor();
    let std_width = 800.0;
    let std_height = 480.0;
    window.set_fullscreen(Some(Fullscreen::Borderless(monitor_handle)));
    let mut width_scale:f64 = window.inner_size().width as f64 / std_width;
    let mut height_scale:f64 = window.inner_size().height as f64 / std_height;
    println!("Inner size is: {:#?}", window.inner_size());
    println!("widht_scale is: {}", width_scale);
    println!("hejgut scale is: {}", height_scale);
    
    let mut world_camera = WorldCamera::new((NUM_ROWS, NUM_COLMS));
    

    let mut cube_object: WorldObject = WorldObject::new();
    let tea_positions = vec![
        // FRONT
        Vertex { position: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },  // [00]
        Vertex { position: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },  // [01]
        Vertex { position: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },  // [02]
        Vertex { position: [1.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },  // [03]
    
        // BACK
        Vertex { position: [0.0, 0.0, 1.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] }, // [04]
        Vertex { position: [0.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] }, // [05]
        Vertex { position: [1.0, 0.0, 1.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] }, // [06]
        Vertex { position: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] }, // [07]
    
        // LEFT
        Vertex { position: [0.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] }, // [08]
        Vertex { position: [0.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] }, // [09]
        Vertex { position: [0.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] }, // [10]
        Vertex { position: [0.0, 1.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] }, // [11]
    
        // RIGHT
        Vertex { position: [1.0, 0.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },  // [12]
        Vertex { position: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },  // [13]
        Vertex { position: [1.0, 0.0, 0.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },  // [14]
        Vertex { position: [1.0, 1.0, 0.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },  // [15]
    
        // TOP
        Vertex { position: [0.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },  // [16]
        Vertex { position: [0.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },  // [17]
        Vertex { position: [1.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },  // [18]
        Vertex { position: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },  // [19]
    
        // BOTTOM
        Vertex { position: [0.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] }, // [20]
        Vertex { position: [0.0, 0.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] }, // [21]
        Vertex { position: [1.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] }, // [22]
        Vertex { position: [1.0, 0.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] }, // [23]
    ];
    
    
    let tea_indices = vec![
        // FRONT
        0, 3, 2, 1, 3, 0,
    
        // BACK
        6, 7, 4, 4, 7, 5,
    
        // LEFT
        8, 11, 10, 9, 11, 8,
    
        // RIGHT
        14, 15, 12, 12, 15, 13,
    
        // TOP
        16, 19, 18, 17, 19, 16,
    
        // BOTTOM
        22, 23, 20, 20, 23, 21,
    ];

    let quad_vertex = vec![
        Vertex{position: [0.1, -0.1, 0.0], normal: [0.1,0.1,0.0], tex_coords: [1.0, 0.0]}, 
        Vertex{position: [0.1, 0.1, 0.0], normal: [0.1,0.1,0.0], tex_coords: [1.0, 1.0]},
        Vertex{position: [-0.1, 0.1, 0.0], normal: [0.1,0.1,0.0], tex_coords: [0.0, 1.0]},
        Vertex{position: [-0.1, -0.1, 0.0], normal: [0.1,0.1,0.0], tex_coords: [0.0, 0.0]}
    ]; 


    let obj_vert = util::read_shader(include_bytes!(r"../shaders/vert2.glsl"));
    let obj_frag = util::read_shader(include_bytes!(r"../shaders/frag2.glsl"));

    let point_vert = util::read_shader(include_bytes!(r"../shaders/point_vert.glsl"));
    let point_frag = util::read_shader(include_bytes!(r"../shaders/point_frag.glsl"));
    let mult_point_vert = util::read_shader(include_bytes!(r"../shaders/bezier_points/vert.glsl"));
    let mult_point_frag = util::read_shader(include_bytes!(r"../shaders/bezier_points/frag.glsl"));

    let line_vert_shader = util::read_shader(include_bytes!(r"../shaders/line_vert.glsl"));
    let line_frag_shader = util::read_shader(include_bytes!(r"../shaders/line_frag.glsl"));

    let text_vert_shader  = util::read_shader(include_bytes!(r"../shaders/text_vert.glsl"));
    let text_frag_shader  = util::read_shader(include_bytes!(r"../shaders/text_frag.glsl"));

    let surface_vert_shader  = util::read_shader(include_bytes!(r"../shaders/bezier_surface/vert.glsl"));
    let surface_frag_shader  = util::read_shader(include_bytes!(r"../shaders/bezier_surface/frag.glsl"));
    let surface_tess_ctrl_shader  = util::read_shader(include_bytes!(r"../shaders/bezier_surface/tess_ctrl.glsl"));
    let surface_tess_eval_shader  = util::read_shader(include_bytes!(r"../shaders/bezier_surface/tess_eval.glsl"));
    
    // Setup specific parameters

    let light = [-1.0, 0.4, 0.9f32];

    let line_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
      polygon_mode: glium::draw_parameters::PolygonMode::Line,
      line_width: Some(5.0),
        .. Default::default()
    };

    let surface_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let text_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let point_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let mult_point_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    //Read textures
        //Tile textures
    //let tile_texture_atlas_image = image::load(std::io::Cursor::new(&include_bytes!(r"textures\texture_atlas_tiles.png")),image::ImageFormat::Png).unwrap().to_rgba8();
    //let image_dimensions = tile_texture_atlas_image.dimensions();
    //let tile_texture_atlas_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&tile_texture_atlas_image.into_raw(), image_dimensions);
    //let tile_texture_atlas = glium::texture::Texture2d::new(&display, tile_texture_atlas_image).unwrap();
    
        //Font textures
            // Font chars are of size 12 x 6
    let font_raw_image = image::load(std::io::Cursor::new(&include_bytes!(r"textures\standard_font.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let font_dimensions = font_raw_image.dimensions();
    let font_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&font_raw_image.into_raw(), font_dimensions);
    let font_atlas = glium::texture::Texture2d::new(&display, font_image).unwrap();

    let mut point = WorldPoint::new(0.5,Vec2::ZERO,Vec3::ZERO);

    //Shape of quad
    let quad_shape:Vec<Vertex> = vec![
        Vertex{position: [-1.0*0.1, -1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 0.0]}, 
        Vertex{position: [1.0*0.1, -1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 0.0]},
        Vertex{position: [1.0*0.1, 1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 1.0]},
        Vertex{position: [-1.0*0.1, 1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 1.0]}
    ];
    
    let quad_indicies = vec![0, 2, 1, 0, 2, 3];

    let obj_renderer = rendering::render::Renderer::new(&tea_positions, &tea_indices, Some(glium::index::PrimitiveType::TrianglesList), &obj_vert, &obj_frag, None, None, None, &display, None).unwrap();
    let mut line_renderer = rendering::render::Renderer::new_empty_dynamic(100, Some(glium::index::PrimitiveType::LinesList), &line_vert_shader, &line_frag_shader, None, &display, Some(line_params)).unwrap();
    let ui_renderer = rendering::render::Renderer::new_empty_dynamic(100, Some(glium::index::PrimitiveType::TrianglesList), &line_vert_shader, &line_frag_shader, None, &display, None).unwrap();
    let text_renderer = rendering::render::Renderer::new(&quad_shape, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &text_vert_shader, &text_frag_shader, None, None, None, &display, Some(text_params)).unwrap();
    let point_renderer = rendering::render::Renderer::new(&quad_vertex, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &point_vert, &point_frag, None, None, None, &display, Some(point_params)).unwrap();
    let mult_point_renderer = rendering::render::Renderer::new(&quad_vertex, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &mult_point_vert, &mult_point_frag, None, None, None, &display, Some(mult_point_params)).unwrap();
    let mut selected_point: i32 = -1;


    let mut surface_points = vec![
        // Row 0 
        VertexSimple { w_position: [-1.0, -1.0, 0.0]}, // c00
        VertexSimple { w_position: [-0.33, -1.0, 3.0]}, // c01
        VertexSimple { w_position: [0.33, -1.0, 1.0]},  // c02
        VertexSimple { w_position: [1.0, -1.0, 0.0]},   // c03
        // Row 1 
        VertexSimple { w_position: [-1.0, -0.33, 0.0]}, // c10
        VertexSimple { w_position: [-0.33, -0.33, 3.0]}, // c11
        VertexSimple { w_position: [0.33, -0.33, 2.0]},  // c12
        VertexSimple { w_position: [1.0, -0.33, 0.0]},   // c13
        // Row 2
        VertexSimple { w_position: [-4.0, 0.33, -1.0]},  // c20
        VertexSimple { w_position: [-0.33, 0.33, 0.0]}, // c21
        VertexSimple { w_position: [0.33, 0.33, 1.0]},   // c22
        VertexSimple { w_position: [1.0, 0.33, 0.0]},    // c23
        // Row 3
        VertexSimple { w_position: [-1.0, 1.0, -1.0]},   // c30
        VertexSimple { w_position: [-0.33, 3.0, 0.0]},  // c31
        VertexSimple { w_position: [0.33, 1.0, 0.0]},    // c32
        VertexSimple { w_position: [1.0, 1.0, 0.0]},     // c33

        // Row 1 
        VertexSimple { w_position: [-1.0+6.0, -0.33, 0.0]}, // c10
        VertexSimple { w_position: [-0.33+6.0, -0.33, 3.0]}, // c11
        VertexSimple { w_position: [0.33+6.0, -0.33, 2.0]},  // c12
        VertexSimple { w_position: [1.0+6.0, -0.33, 0.0]},   // c13
        // Row w_position
        VertexSimple { w_position: [-4.0+6.0, 0.33, -1.0]},  // c20
        VertexSimple { w_position: [-0.33+6.0, 0.33, 0.0]}, // c21
        VertexSimple { w_position: [0.33+6.0, 0.33, 1.0]},   // c22
        VertexSimple { w_position: [1.0+6.0, 0.33, 0.0]},    // c23
        // Row w_position
        VertexSimple { w_position: [-1.0+6.0, 1.0, -1.0]},   // c30
        VertexSimple { w_position: [-0.33+6.0, 3.0, 0.0]},  // c31
        VertexSimple { w_position: [0.33+6.0, 1.0, 0.0]},    // c32
        VertexSimple { w_position: [1.0+6.0, 1.0, 0.0]},     // c33
    ];
    let mut surface_vbo = glium::VertexBuffer::new(&display, &surface_points).unwrap();
    let surface_renderer = rendering::render::Renderer::new(&vec![], &vec![0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,15,11,7,3,16,17,18,19,20,21,22,23,24,25,26,27], Some(PrimitiveType::Patches {vertices_per_patch: 16,}), &surface_vert_shader, &surface_frag_shader, None, Some(&surface_tess_ctrl_shader), Some(&surface_tess_eval_shader), &display, Some(surface_params)).unwrap();

    line_renderer.draw_line((-1.0,-1.0), (1.0,1.0), None);
    let mut fps_text = RenderedText::new(String::from("00000fps"));
    let mut text_vbo = TextVbo::new(100, &display);
    text_vbo.add_text((0.78,0.95), 0.085, Some([1.0,0.5,1.0]), &mut fps_text);
    // Uniform setup
        // Text uniforms
    let text_behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    let _light = [-1.0, 0.4, 0.9f32];

    camera.perspective = rendering::render::calculate_perspective(window.inner_size().into());
    let mut frames:f32 = 0.0;


    //let mut mouse_pos: Point = Point{x:0.0,y:0.0};
    let mut mouse_pos: Vec3 = Vec3::ZERO;

    let mut t: f32 = 0.0;
    let dt: f32 = 0.0167;

    let mut current_time = Instant::now();
    let mut accumulator: f32 = 0.0;
    let mut ctrl_pressed = false;
    let mut total_fps: usize = 0;
    let mut prev_mouse_pos = Vec3::ZERO;
    let mut timer = Instant::now();
    let mut overall_fps = 0.0;
    let smoothing = 0.6; // larger=more smoothing
    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                println!("Average fps was: {}", total_fps/frames as usize);
                window_target.exit()
            },
            winit::event::WindowEvent::CursorMoved { device_id: _, position } => {
                prev_mouse_pos = mouse_pos;
                mouse_pos = Vec3::new(
                    (position.x as f32 / window.inner_size().width as f32) * 2.0 - 1.0,
                    - ((position.y as f32 / window.inner_size().height as f32) * 2.0 - 1.0),
                    1.0,
                );

                if ctrl_pressed && selected_point != -1{
                    //Change to update this each frame instead...
                    //Make the mouse wheel also simpler...
                    let dist = (mouse_pos-prev_mouse_pos).normalize();
                    println!("Dist is {:#?}", dist);
                    surface_points[selected_point as usize].w_position[0] += dist.x;
                    surface_points[selected_point as usize].w_position[1] += dist.y;

                    //Onödigt att göra om hela ig. Men i dunno är just nu bara 16 punkter...
                    surface_vbo = glium::VertexBuffer::new(&display, &surface_points).unwrap();
                }else{
                    //let camera_ndc = world_to_pixel(camera.get_pos(), &camera.camera_matrix, window.inner_size(),&camera.perspective);
                    //println!("Camera pos ndc is: {:#?}", camera_ndc);
    

                }
                
            }
            winit::event::WindowEvent::MouseWheel { device_id: _, delta, phase } =>{
                if selected_point != -1 && ctrl_pressed{
                    match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            surface_points[selected_point as usize].w_position[2] += y;

                            //Onödigt att göra om hela ig. Men i dunno är just nu bara 16 punkter...
                            surface_vbo = glium::VertexBuffer::new(&display, &surface_points).unwrap();
                        }
                        _ => {}
                    }
                }
            }
            winit::event::WindowEvent::MouseInput { device_id: _, state, button } =>{
                //Change to update this each frame instead...
                //Make the mouse wheel also simpler...
                if state.is_pressed() && button == MouseButton::Left{
                    let mouse_dir = ndc_to_direction(&mouse_pos, &camera.camera_matrix, &camera.perspective);
                    let mouse_point = camera.get_pos();
                    let mut i  = -1;
                    let mut min_dist = 100.0;
                    let mut min_ind = -1;
                    for point in surface_points.iter(){
                        i += 1;
                        let dist = distance_ray_point(mouse_point,mouse_dir,Vec3::from_array(point.w_position));
                        if min_dist>dist{
                            min_dist = dist;
                            min_ind = i;
                        } 
                    }
                    selected_point = min_ind;
                }
            }

            // TODO
            // Make input a little bit nicer
            winit::event::WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } =>{

                //Handle other inputs
                if event.physical_key == keyboard::KeyCode::Escape && event.state.is_pressed(){
                    println!("Average fps was: {}", total_fps/frames as usize);
                    window_target.exit()
                } 
                else if event.physical_key == keyboard::KeyCode::KeyQ && event.state.is_pressed(){
                    camera.r#move(-CAMERA_SPEED*camera.get_front());
                    point.get_mut_model().translate(-CAMERA_SPEED*camera.get_front());
                    camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                    //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&camera.perspective)*camera.camera_matrix*Mat4::IDENTITY));
                }
                else if event.physical_key == keyboard::KeyCode::KeyE{
                    camera.r#move(CAMERA_SPEED*camera.get_front());
                    point.get_mut_model().translate(CAMERA_SPEED*camera.get_front());

                    camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                    //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&camera.perspective)*camera.camera_matrix*Mat4::IDENTITY));
                }else if event.physical_key == keyboard::KeyCode::KeyU && event.state.is_pressed(){
                    cube_object.translate(Vec3::from_array([0.0,1.0,0.0]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyH && event.state.is_pressed(){
                    cube_object.translate(Vec3::from_array([-1.0,0.0,0.0]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyJ && event.state.is_pressed(){
                    cube_object.translate(Vec3::from_array([0.0,-1.0,0.0]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyK && event.state.is_pressed(){
                    cube_object.translate(Vec3::from_array([1.0,0.0,0.0]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyY && event.state.is_pressed(){
                    cube_object.scale(Vec3::from_array([2.0,2.0,2.0]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyI && event.state.is_pressed(){
                    cube_object.scale(Vec3::from_array([0.5,0.5,0.5]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyO && event.state.is_pressed(){
                    cube_object.rotate(Vec3::from_array([1.0,0.0,1.0]).normalize(), 2.0943951);
                }
                else if event.physical_key == keyboard::KeyCode::KeyL && event.state.is_pressed(){
                    cube_object.rotate(Vec3::from_array([1.0,1.0,0.0]).normalize(), 0.785398163);
                }else if event.state.is_pressed() && event.physical_key == keyboard::KeyCode::ControlLeft{
                    ctrl_pressed = true;
                }else if !event.state.is_pressed() && event.physical_key == keyboard::KeyCode::ControlLeft{
                    ctrl_pressed = false;
                    selected_point = -1;
                }
                //Handle WASD

                input_handler.update_input(event);

            },
            winit::event::WindowEvent::Resized(window_size) => {
                camera.perspective = rendering::render::calculate_perspective(window_size.into());
                //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&camera.perspective)*camera.camera_matrix*Mat4::IDENTITY));
                display.resize(window_size.into());
                width_scale = window_size.width as f64/ std_width;
                height_scale = window_size.height as f64/ std_height;
                println!("Scale factors are: {} and {}", width_scale, height_scale);
            },
            winit::event::WindowEvent::RedrawRequested => {
                //Physics step
                let new_time = Instant::now();
                let mut frame_time = current_time.elapsed().as_secs_f32() - new_time.elapsed().as_secs_f32();

                if frame_time > 0.25{
                    frame_time = 0.25;
                }
                current_time = new_time;

                accumulator += frame_time;
                //Looks more stuttery, which I do not like
                //If we had some way to compare and interpolate states it would probably be fine but alas.
                // Could interpolate camera posistion (as long as there hasn't been a jump, is still possible but will be a little bit harder)?
                //println!("Before physics: {} ms",current_time.elapsed().as_millis());
                while accumulator >= dt {
                    //println!("Clicked Unit has ID:{:?}", entity_handler.get_selected_unit());
                    let time_update = Instant::now();

                    update_game_logic(dt, &mut camera, &mut world_camera, &input_handler, &mut mouse_pos, &mut point, &window); 
                    //println!("Update game: {} ms", time_update.elapsed().as_millis());
                    t += dt;
                    accumulator -= dt;
                }
                

                //Render step

                let time_update = Instant::now();
                //println!("Before fps-counter: {} ms",current_time.elapsed().as_millis());
                //Linear interpolation between states, cant really do it but yeah...
                //State state = currentState * alpha +  previousState * ( 1.0 - alpha );
                let delta_time = timer.elapsed().as_secs_f32();
                timer = Instant::now();
                // Get fps 
                    //This has to be done faster (is very slow now...)
                let current = 1.0 / delta_time;
                overall_fps = ((overall_fps * smoothing) + (current * (1.0-smoothing))).min(50_000.0);
                total_fps += overall_fps as usize;
                let fps_as_text = format_to_exact_length(overall_fps as u32, 5) + "fps";
                fps_text.change_text(fps_as_text);

                //It is this that takes the majority of the time
                text_vbo.replace_text(&fps_text);           
                
                //println!("Redraw requested");´
                //println!("Time for updating fps counter {}", dur2.elapsed().as_secs_f32());
                //dur2 = Instant::now();
                //println!("Time for updating game logic {}", dur2.elapsed().as_secs_f32());
                //dur2 = Instant::now();
                //time += 0.02;

                //let x_off = time.sin() * 0.5;
                //println!("Before clearing: {} ms",current_time.elapsed().as_millis());
                let mut target = display.draw();

                target.clear_color_and_depth((0.0, 0.1, 1.0, 1.0), 1.0);
                //println!("Before drawing: {} ms",current_time.elapsed().as_millis());
                let shader_time = (t*8.0).floor()%8.0;
                //println!("Time is: {}", shader_time);
                let un_modded_pos = 0.0+0.125*shader_time;
                //println!("Pos is: {}", un_modded_pos);
                //    float animation_step = mod(tex_offsets.x+1.0*tex_offsets.z*time,animation_length);
                

                target.draw(&surface_vbo, &surface_renderer.indicies, &surface_renderer.program, &uniform! {u_light: light, steps: 32.0 as f32, model: Mat4::IDENTITY.to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &surface_renderer.draw_params).unwrap();

                //target.draw(&line_renderer.vbo, &line_renderer.indicies, &line_renderer.program, &uniform! {}, &line_renderer.draw_params).unwrap();
                target.draw(&point_renderer.vbo, &point_renderer.indicies, &point_renderer.program, &uniform!{radius: point.get_radius(), model: point.get_model().get_model().to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &point_renderer.draw_params).unwrap();
                target.draw((&mult_point_renderer.vbo, surface_vbo.per_instance().unwrap()), &mult_point_renderer.indicies, &mult_point_renderer.program, &uniform! {selected: selected_point, model: (0.1*Mat4::IDENTITY).to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &mult_point_renderer.draw_params).unwrap();
                //println!("Buffer is: {:#?}", &surface_renderer.vbo);


                //target.draw(&obj_renderer.vbo, &obj_renderer.indicies, &obj_renderer.program, &uniform! { u_light: light, model: cube_object.get_model().to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &Default::default()).unwrap();
                

                
                target.draw(&ui_renderer.vbo, &ui_renderer.indicies, &ui_renderer.program, &uniform! {tex:&font_atlas}, &Default::default()).unwrap();
                target.draw((&text_renderer.vbo, text_vbo.vbo.per_instance().unwrap()), &text_renderer.indicies, &text_renderer.program, &uniform! {tex:glium::uniforms::Sampler(&font_atlas, text_behavior)}, &text_renderer.draw_params).unwrap();
                
                //trig_renderer.draw(&mut target, Some(&params), Some(&uniform! { model: obj_size, projection: perspective, view:camera.camera_matrix.to_cols_array_2d(), u_light:light}));
                //hex_renderer.draw(&mut target, Some(&params), Some(&uniform!{matrix: hex_size, perspective: perspective}));
                //println!("\t\tUploading info to GPU took: {} ms", dur2.elapsed().as_millis());
                //sleep(Duration::from_millis(14));
                //println!("Time for drawing {}", dur2.elapsed().as_secs_f32());
                //dur2 = Instant::now();
                //println!("Before finishing: {} ms",current_time.elapsed().as_millis());
                target.finish().unwrap();
                //println!("In total: {} ms\n",current_time.elapsed().as_millis());
                frames = frames + 1.0;
                //println!("Time for rendering to screen {}", dur2.elapsed().as_secs_f32());
                //println!("\t\tTime for drawing frame: {} ms\n", dur2.elapsed().as_millis());
            },
            _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };

        // I think this solution is broken. 
        // Can get stuck in infinite screen or something
        // Works for now but needs to be fixed...
        //println!("One frame took {} ms\n", now.elapsed().as_millis());
    });
}


fn update_game_logic(delta_time: f32, camera: &mut RenderCamera,world_camera: &mut WorldCamera,input_handler: &InputHandler,mut mouse_pos:&mut Vec3, mouse_point: &mut WorldPoint, window: &Window){
    //Update movement (Kanske göra efter allt annat... possibly):
    let mut movement = input_handler.get_movement();
    if movement.length() > 0.0{
        movement = movement.normalize();
        //Flytta en i taget...
        // Make this be calculated once instead of twice...
        
        //I should make the camera more robust... To have pitch and yaw and whatever...

        //Up borde ju också ändra sig right?
        let front = -(camera.get_pos() - mouse_point.get_model().get_posistion()).normalize();

        camera.r#move(delta_time*movement[1]*CAMERA_SPEED*camera.get_up());

        let y_pos = camera.get_pos()[1];

        camera.r#move(delta_time*movement[0]*CAMERA_SPEED*(front.cross(camera.get_up())).normalize());

        let x_pos = camera.get_pos()[0];
                //Kom på varför det är 0.12 här och inget annat nummer...
                //Verkar ju bara bero på hex_size och inte scale....
        //Now convert world space traveled to screen space traveled...

    }
    //println!("Mouse NDC is: {:#?}", current_ndc);
    
    // I have to modify the z-axis right, since if I rotate it will not always intersect with z = 0.0. I want it to intersect with a point 5 units in front of the camera..
    
    // I now only want to make the NDC control the amount...
    let intersect = ndc_to_point(&(Vec3::new(-mouse_pos.x,-mouse_pos.y,0.0)),&camera.camera_matrix,camera.get_pos(),&camera.perspective, 5.0);
    mouse_point.get_mut_model().set_translation(intersect);
    //Rotate point to face camera:

    camera.change_target(intersect);
    camera.camera_matrix = camera.look_at(intersect+camera.get_front());

}

pub fn get_clicked_pos(mouse_pos: &mut Point, world_camera: &mut WorldCamera){

}