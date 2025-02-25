#[macro_use]
extern crate glium;
extern crate winit;
use grass::get_grass_shape;
use object::{point::WorldPoint, WorldObject};
use rand::{distr::{Distribution, Uniform}, Rng};
use glam::{Mat4, Vec2, Vec3};
use util::{input_handler::InputHandler, ray_library::{distance_ray_point, ndc_to_direction, ndc_to_intersection, ndc_to_point, world_to_pixel}};
use winit::{dpi::PhysicalSize, event::{MouseButton, MouseScrollDelta}, event_loop::{ControlFlow, EventLoop}, keyboard, raw_window_handle::HasWindowHandle, window::{Fullscreen, Window}};
use glium::{backend::Facade, framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, implement_vertex, index::PrimitiveType, texture::DepthTexture2d, uniforms::{MagnifySamplerFilter, MinifySamplerFilter}, Blend, BlendingFunction, Display, LinearBlendingFactor, Surface, Texture2d, VertexBuffer};
use world::{hex::Hex, layout::{HexLayout, Point, EVEN}, offset_coords::{qoffset_from_cube_offsets, qoffset_to_cube, qoffset_to_cube_offsets}, tile::Tile, world_camera::WorldCamera, OffsetTile, NUM_COLMS, NUM_ROWS};
use core::f32;
use std::{io::stdout, time::Instant};


mod object;
mod grass;
mod rendering;
mod bezier_surface;
use bezier_surface::{create_surface, create_surface_quad, GrassVertex};
use rendering::{render::{self, array_to_vbo, Vertex, VertexSimple}, render_camera::RenderCamera, text::{format_to_exact_length, RenderedText, TextVbo}};


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
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_title("DD2258 project (Bezier surface)").build(&event_loop);
    
    (event_loop, window, display)
}

//Camera constants

const CAMERA_SPEED:f32 = 2.0;
const eps:f32 = 0.0001;

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
    //display.resize((256,144));
    println!("Inner size is: {:#?}", window.inner_size());
    println!("widht_scale is: {}", width_scale);
    println!("hejgut scale is: {}", height_scale);
    
    let mut world_camera = WorldCamera::new((NUM_ROWS, NUM_COLMS));


    
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
    let surface_geometry_shader = util::read_shader(include_bytes!(r"../shaders/bezier_surface/geometry.glsl"));

    let grass_vert = util::read_shader(include_bytes!(r"../shaders/grass/grass_vert.glsl"));
    let grass_frag = util::read_shader(include_bytes!(r"../shaders/grass/grass_frag.glsl"));

    let low_res_vert = util::read_shader(include_bytes!(r"../shaders/resolution/vert.glsl"));
    let low_res_frag = util::read_shader(include_bytes!(r"../shaders/resolution/frag.glsl"));


    // Setup specific parameters

    let light = [-1.0, -0.5, 0.0f32];

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

    let text_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let point_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let obj_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let mult_point_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let custom_blend = Blend {
        color: BlendingFunction::Addition {
            source: LinearBlendingFactor::SourceAlpha,
            destination: LinearBlendingFactor::OneMinusSourceAlpha,
        },
        alpha: BlendingFunction::Addition {
            source: LinearBlendingFactor::One,
            destination: LinearBlendingFactor::One,
        },
        constant_value: (1.0, 1.0, 1.0, 1.0),
    };
    

    let grass_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        blend: custom_blend,
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

    let grass_raw_image = image::load(std::io::Cursor::new(&include_bytes!(r"textures\grass.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let grass_dimensions = grass_raw_image.dimensions();
    let grass_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&grass_raw_image.into_raw(), grass_dimensions);
    let grass_texture = glium::texture::Texture2d::new(&display, grass_image).unwrap();

    let mut point = WorldPoint::new(0.5,Vec2::ZERO,Vec3::ZERO);

    //Shape of quad
    let quad_shape:Vec<Vertex> = vec![
        Vertex{position: [-1.0*0.1, -1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 0.0]}, 
        Vertex{position: [1.0*0.1, -1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 0.0]},
        Vertex{position: [1.0*0.1, 1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 1.0]},
        Vertex{position: [-1.0*0.1, 1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 1.0]}
    ];

    //Whole quad
    let screen_quad:Vec<Vertex> = vec![
            Vertex{position: [-1.0, -1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 0.0]}, 
            Vertex{position: [1.0, -1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 0.0]},
            Vertex{position: [1.0, 1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 1.0]},
            Vertex{position: [-1.0, 1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 1.0]}
    ];
    
    let quad_indicies = vec![0, 2, 1, 0, 2, 3];

    let mut obj_renderer = rendering::render::Renderer::new(&quad_shape, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &obj_vert, &obj_frag, None, None, None, &display, Some(obj_params), None).unwrap();
    let mut line_renderer = rendering::render::Renderer::new_empty_dynamic(100, Some(glium::index::PrimitiveType::LinesList), &line_vert_shader, &line_frag_shader, None, &display, Some(line_params)).unwrap();
    let ui_renderer = rendering::render::Renderer::new_empty_dynamic(100, Some(glium::index::PrimitiveType::TrianglesList), &line_vert_shader, &line_frag_shader, None, &display, None).unwrap();
    let text_renderer = rendering::render::Renderer::new(&quad_shape, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &text_vert_shader, &text_frag_shader, None, None, None, &display, Some(text_params), None).unwrap();
    let point_renderer = rendering::render::Renderer::new(&quad_vertex, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &point_vert, &point_frag, None, None, None, &display, Some(point_params), None).unwrap();
    let mult_point_renderer = rendering::render::Renderer::new(&quad_vertex, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &mult_point_vert, &mult_point_frag, None, None, None, &display, Some(mult_point_params), None).unwrap();
    let mut selected_point: i32 = -1;
    let grass = get_grass_shape();
    let grass_renderer = rendering::render::Renderer::new(&grass.0, &grass.1, Some(glium::index::PrimitiveType::TrianglesList), &grass_vert, &grass_frag, None, None, None, &display, Some(grass_params), None).unwrap();
    let low_res_renderer = rendering::render::Renderer::new(&screen_quad,&quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &low_res_vert, &low_res_frag, None, None, None, &display, None, None).unwrap();
    /*let mut surface_points = vec![
        // Row 0 
        VertexSimple { w_position: [-1.0, 0.0, -1.0]}, // c00
        VertexSimple { w_position: [-0.33, 3.0, -1.0]}, // c01
        VertexSimple { w_position: [0.33, 1.0, -1.0]},  // c02
        VertexSimple { w_position: [1.0, 0.0, -1.0]},   // c03
        // Row 1 
        VertexSimple { w_position: [-1.0, 0.0, -0.33]}, // c10
        VertexSimple { w_position: [-0.33, 3.0, -0.33]}, // c11
        VertexSimple { w_position: [0.33, 2.0, -0.33]},  // c12
        VertexSimple { w_position: [1.0, 0.0, -0.33]},   // c13
        // Row 2
        VertexSimple { w_position: [-4.0, -1.0, 0.33]},  // c20
        VertexSimple { w_position: [-0.33, 0.0, 0.33]}, // c21
        VertexSimple { w_position: [0.33, 1.0, 0.33]},   // c22
        VertexSimple { w_position: [1.0, 0.0, 0.33]},    // c23
        // Row 3
        VertexSimple { w_position: [-1.0, -1.0, 1.0]},   // c30
        VertexSimple { w_position: [-0.33, 0.0, 3.0]},  // c31
        VertexSimple { w_position: [0.33, 0.0, 1.0]},    // c32
        VertexSimple { w_position: [1.0, 0.0, 1.0]},     // c33

        // Row 1 quad 2
        VertexSimple { w_position: [-1.0+6.0, 0.0, -0.33]}, // c10
        VertexSimple { w_position: [-0.33+6.0, 3.0, -0.33]}, // c11
        VertexSimple { w_position: [0.33+6.0, 2.0, -0.33]},  // c12
        VertexSimple { w_position: [1.0+6.0, 0.0, -0.33]},   // c13
        // Row 2
        VertexSimple { w_position: [-4.0+6.0, -1.0, 0.33]},  // c20
        VertexSimple { w_position: [-0.33+6.0, 0.0, 0.33]}, // c21
        VertexSimple { w_position: [0.33+6.0, 1.0, 0.33]},   // c22
        VertexSimple { w_position: [1.0+6.0, 0.0, 0.33]},    // c23
        // Row 3
        VertexSimple { w_position: [-1.0+6.0, -1.0, 1.0]},   // c30
        VertexSimple { w_position: [-0.33+6.0, 0.0, 3.0]},  // c31
        VertexSimple { w_position: [0.33+6.0, 0.0, 1.0]},    // c32
        VertexSimple { w_position: [1.0+6.0, 0.0, 1.0]},     // c33
    ];*/
    let mut bezier_quads = 1;
    let mut grass_density = 16;
    let mut bezier_surface= bezier_surface::Surface::new(Vec3::new(-1.0, 0.0, -1.0),2.0,2.0,1,1); 
    println!("Surface points length: {:#?}", bezier_surface.points.len());
    //println!("Surface points are: {:#?}", bezier_surface.points);
    /*println!("Point {} is evaluated to: {:?}", Vec3::new(4.0,0.0,-1.0), bezier_surface.evaluate(Vec3::new(4.0,0.0,-1.0)).unwrap().0);
    println!("Point {} is evaluated to: {:?}", Vec3::new(0.5,0.0,-1.0), bezier_surface.evaluate(Vec3::new(0.5,0.0,-1.0)).unwrap().0);
    println!("Point {} is evaluated to: {:?}", Vec3::new(1.5,0.0,-1.0), bezier_surface.evaluate(Vec3::new(1.5,0.0,-1.0)).unwrap().0);
    if bezier_quads > 0{
        return;
    }*/
    let mut surface_vbo: VertexBuffer<VertexSimple> = glium::VertexBuffer::new(&display, &bezier_surface.points).unwrap();
    let mut surface_indicies = glium::IndexBuffer::new(&display,PrimitiveType::Patches {vertices_per_patch: 16,},
    &bezier_surface.inds).unwrap();
    //Pass this surface_vbo into a compute shader?
    let surface_renderer = rendering::render::Renderer::new(&vec![], &vec![], Some(PrimitiveType::Patches {vertices_per_patch: 16,}), &surface_vert_shader, &surface_frag_shader, /*Some(surface_geometry_shader)*/None, Some(&surface_tess_ctrl_shader), Some(&surface_tess_eval_shader), &display, None, Some((
        vec![(&"out_point").to_string()], // the names of the outputs to capture
        glium::program::TransformFeedbackMode::Interleaved,
    ))).unwrap();
    
    let now = Instant::now();
    let mut grass_pos:Vec<GrassVertex> = bezier_surface.get_grass_posistions(grass_density);
    let mut grass_instances = glium::VertexBuffer::new(&display, &grass_pos).unwrap();
    println!("Updating {} grass pos took: {:.2?}", grass_pos.len(), now.elapsed());

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
    let mut obj1: WorldObject = WorldObject::new();
    obj1.scale(Vec3::new(10.0, 10.0, 10.0));
    obj1.set_translation(Vec3::new(0.0, 5.0, 0.0)); 

    let mut obj2: WorldObject = WorldObject::new();
    obj2.scale(Vec3::new(10.0, 10.0, 10.0));
    obj2.set_translation(Vec3::new(0.5, 6.0, 0.5)); 

    let mut obj3: WorldObject = WorldObject::new();
    obj3.scale(Vec3::new(10.0, 10.0, 10.0));
    obj3.set_translation(Vec3::new(1.2, 7.0, 0.2)); 

    let mut obj4: WorldObject = WorldObject::new();
    obj4.scale(Vec3::new(10.0, 10.0, 10.0));
    obj4.set_translation(Vec3::new(0.7, 5.0, 0.7)); 

    let mut world_objs: Vec<WorldObject> = vec![obj1, obj2, obj3, obj4];

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
        let (world_texture, depth_world_texture) = create_render_textures(&display, 640, 360);
        let mut fbo = create_fbo(&display, &world_texture, &depth_world_texture);
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
                    bezier_surface.points[selected_point as usize].w_position[0] += dist.x;
                    bezier_surface.points[selected_point as usize].w_position[1] += dist.y;
                    //bezier_surface.points[selected_point as usize + 2].w_position[0] -= dist.x;
                    //bezier_surface.points[selected_point as usize + 2].w_position[1] -= dist.y;
                    

                    //Onödigt att göra om hela ig. Men i dunno är just nu bara 16 punkter...
                    grass_instances = glium::VertexBuffer::new(&display, &bezier_surface.get_grass_posistions(grass_density)).unwrap();
                    surface_vbo = glium::VertexBuffer::new(&display, &bezier_surface.points).unwrap();
                }else{
                    //let camera_ndc = world_to_pixel(camera.get_pos(), &camera.camera_matrix, window.inner_size(),&camera.perspective);
                    //println!("Camera pos ndc is: {:#?}", camera_ndc);
    

                }
                
            }
            winit::event::WindowEvent::MouseWheel { device_id: _, delta, phase } =>{
                if selected_point != -1 && ctrl_pressed{
                    match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            bezier_surface.points[selected_point as usize].w_position[2] += y;

                            //Onödigt att göra om hela ig. Men i dunno är just nu bara 16 punkter...
                            surface_vbo = glium::VertexBuffer::new(&display, &bezier_surface.points).unwrap();
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
                    for point in bezier_surface.points.iter(){
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
                    world_objs[0].translate(Vec3::from_array([0.0,1.0,0.0]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyH && event.state.is_pressed(){
                    world_objs[0].translate(Vec3::from_array([-1.0,0.0,0.0]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyJ && event.state.is_pressed(){
                    world_objs[0].translate(Vec3::from_array([0.0,-1.0,0.0]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyK && event.state.is_pressed(){
                    world_objs[0].translate(Vec3::from_array([1.0,0.0,0.0]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyY && event.state.is_pressed(){
                    world_objs[0].scale(Vec3::from_array([2.0,2.0,2.0]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyI && event.state.is_pressed(){
                    world_objs[0].scale(Vec3::from_array([0.5,0.5,0.5]));
                }
                else if event.physical_key == keyboard::KeyCode::KeyO && event.state.is_pressed(){
                    world_objs[0].rotate(Vec3::from_array([1.0,0.0,1.0]).normalize(), 2.0943951);
                }
                else if event.physical_key == keyboard::KeyCode::KeyL && event.state.is_pressed(){
                    world_objs[0].rotate(Vec3::from_array([1.0,1.0,0.0]).normalize(), 0.785398163);
                }else if event.state.is_pressed() && event.physical_key == keyboard::KeyCode::ControlLeft{
                    ctrl_pressed = true;
                }else if !event.state.is_pressed() && event.physical_key == keyboard::KeyCode::ControlLeft{
                    ctrl_pressed = false;
                    selected_point = -1;
                }else if event.physical_key == keyboard::KeyCode::ArrowRight && event.state.is_pressed(){
                    bezier_quads += 1;
                    println!("Quads in one dimension is: {}", bezier_quads);
                    println!("Num quads is {}", bezier_quads*bezier_quads);
                    bezier_surface = bezier_surface::Surface::new(Vec3::new(-1.0, 0.0, -1.0),2.0,2.0,bezier_quads,bezier_quads); 
                    grass_instances = glium::VertexBuffer::new(&display, &bezier_surface.get_grass_posistions(grass_density)).unwrap();
                    surface_vbo = glium::VertexBuffer::new(&display, &bezier_surface.points).unwrap();
                    surface_indicies = glium::IndexBuffer::new(&display,PrimitiveType::Patches {vertices_per_patch: 16,},
                        &bezier_surface.inds).unwrap();
                }else if event.physical_key == keyboard::KeyCode::ArrowLeft && event.state.is_pressed(){
                    bezier_quads -= 1;
                    println!("Num quads is {}", bezier_quads*bezier_quads);
                    bezier_surface = bezier_surface::Surface::new(Vec3::new(-1.0, 0.0, -1.0),2.0,2.0,bezier_quads,bezier_quads); 
                    grass_instances = glium::VertexBuffer::new(&display, &bezier_surface.get_grass_posistions(grass_density)).unwrap();
                    surface_vbo = glium::VertexBuffer::new(&display, &bezier_surface.points).unwrap();
                    surface_indicies = glium::IndexBuffer::new(&display,PrimitiveType::Patches {vertices_per_patch: 16,},
                        &bezier_surface.inds).unwrap();
                }else if event.physical_key == keyboard::KeyCode::ArrowUp && event.state.is_pressed(){
                    grass_density += 1;
                    println!("Num grass is {}", bezier_quads*bezier_quads*grass_density*grass_density);
                    grass_instances = glium::VertexBuffer::new(&display, &bezier_surface.get_grass_posistions(grass_density)).unwrap();
                }else if event.physical_key == keyboard::KeyCode::ArrowDown && event.state.is_pressed(){
                    grass_density -= 1;
                    println!("Num grass is {}", bezier_quads*bezier_quads*grass_density*grass_density);
                    grass_instances = glium::VertexBuffer::new(&display, &bezier_surface.get_grass_posistions(grass_density)).unwrap();
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
                   
                    update_obj_physics(dt, &mut bezier_surface, &mut world_objs);
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

                fbo.clear_color_and_depth((0.0, 0.1, 1.0, 1.0), 1.0);
                target.clear_color_and_depth((0.0, 0.1, 1.0, 1.0), 1.0);
                //println!("Before drawing: {} ms",current_time.elapsed().as_millis());
                let shader_time = (t*8.0).floor()%8.0;
                //println!("Time is: {}", shader_time);
                let un_modded_pos = 0.0+0.125*shader_time;
                //println!("Pos is: {}", un_modded_pos);
                //    float animation_step = mod(tex_offsets.x+1.0*tex_offsets.z*time,animation_length);
                
                //println!("Program output is: {:#?}", &surface_renderer.program.get_output_primitives());
                /*let mut surface_buffer: VertexBuffer<VertexSimple> = glium::VertexBuffer::empty_dynamic(&display, 8192).unwrap();
                let surface_session = glium::vertex::TransformFeedbackSession::new(&display, &surface_renderer.program, &mut surface_buffer)
                .expect("Failed to create transform feedback session");
            */
                let surface_params = glium::DrawParameters {
                    //transform_feedback: Some(&surface_session),
                    depth: glium::Depth {
                        test: glium::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    .. Default::default()
                };
                //println!("Have created buffers");
                fbo.draw(&surface_vbo, &surface_indicies, &surface_renderer.program, &uniform! {object_color: [0.3 as f32,0.8 as f32,0.0 as f32], u_light: light, steps: 32.0 as f32, model: Mat4::IDENTITY.to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &surface_params).unwrap();
                //println!("Have drawn surface");
                fbo.draw((&grass_renderer.vbo, grass_instances.per_instance().unwrap()), &grass_renderer.indicies, &grass_renderer.program, &uniform! {u_light: light, threshhold: 0.5 as f32, strength: 0.08 as f32,u_time: t, tex: &grass_texture, u_light: light, model: (Mat4::IDENTITY).to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &grass_renderer.draw_params).unwrap();
                //target.draw(&line_renderer.vbo, &line_renderer.indicies, &line_renderer.program, &uniform! {}, &line_renderer.draw_params).unwrap();
                fbo.draw((&mult_point_renderer.vbo, surface_vbo.per_instance().unwrap()), &mult_point_renderer.indicies, &mult_point_renderer.program, &uniform! {selected: selected_point, model: (0.1*Mat4::IDENTITY).to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &mult_point_renderer.draw_params).unwrap();
                //println!("Buffer is: {:#?}", &surface_renderer.vbo);

                for obj in &world_objs{
                    fbo.draw(&obj_renderer.vbo, &obj_renderer.indicies, &obj_renderer.program, &uniform! { u_light: light, model: obj.get_model().to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &obj_renderer.draw_params).unwrap();

                }
                
                target.draw(&low_res_renderer.vbo, &low_res_renderer.indicies,&low_res_renderer.program, &uniform! {tex: &world_texture}, &low_res_renderer.draw_params).unwrap();

                target.draw(&point_renderer.vbo, &point_renderer.indicies, &point_renderer.program, &uniform!{radius: point.get_radius(), model: point.get_model().get_model().to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &point_renderer.draw_params).unwrap();
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

fn update_obj_physics(delta_time: f32, surface: &mut bezier_surface::Surface, objs:&mut Vec<WorldObject>){
    const EPS: f32 = 0.001;
    const SPHERE_RADIUS: f32 = 0.25;
    //println!("");
    let mut distance_vec: Vec<Vec<usize>> = Vec::with_capacity(objs.len());
    for i in 0..objs.len(){
        distance_vec.push(vec![]);
    }
    for i in 0..objs.len(){
        for j in i+1..objs.len(){
            let dist = objs[i].get_posistion()-objs[j].get_posistion();
            if dist.length() < SPHERE_RADIUS + SPHERE_RADIUS {
                distance_vec[i].push(j);
            }
        }
    }
    for i in 0..objs.len(){
        
        //println!("\nObj is: {:?}", obj.get_posistion());
        //Find closest point on surface
        let pos: Vec3 = objs[i].get_posistion();

        //Collision
        for j in 0..distance_vec[i].len(){
                    let col_index = distance_vec[i][j];
            println!("obj {} is colliding with {}",i, col_index);
            let dist = objs[i].get_posistion()-objs[col_index].get_posistion();
            let distance = dist.length();
            println!("distance is: {}", distance);
            let collision_normal = dist.normalize();
            //let collision_amount = (SPHERE_RADIUS + SPHERE_RADIUS) - distance;
            objs[i].set_translation(pos-dist);
            let pos_2 = objs[col_index].get_posistion();
            objs[col_index].set_translation(pos_2+dist);
            let relative_velocity = objs[col_index].velocity - objs[i].velocity;
            let vel_at_normal = relative_velocity.dot(collision_normal);
            if vel_at_normal > 0.0{
                continue;
            } 
            let impulse_scale = -(1.0 + 0.0) * vel_at_normal;
            let impulse = impulse_scale*collision_normal;
            objs[i].velocity -= impulse;
            objs[col_index].velocity += impulse;
        }

        let (surface_point, d_u, d_v) = surface.evaluate(pos).unwrap_or((Vec3::ZERO,Vec3::ZERO,Vec3::ZERO));
        //println!("Surface point is: {}", surface_point);
        // Calculate distance
        let to_sphere = objs[i].get_posistion() - surface_point;
        //println!("To sphere is: {}", to_sphere);
        let distance = to_sphere.length();
        let normal = -d_u.cross(d_v).normalize();
        //println!("Distance is: {}", distance);
        // 3. Check sphere collision
        if distance < SPHERE_RADIUS + EPS {
            let penetration_depth = SPHERE_RADIUS + EPS - distance;
            objs[i].translate( normal * penetration_depth);
            if -3.0 < penetration_depth {
                objs[i].set_translation(pos.with_y(surface_point.y+SPHERE_RADIUS));
            }
            
            
            let velocity = objs[i].velocity;
            let normal_speed = velocity.dot(normal);
            
            //Bounce
            if normal_speed < 0.0 {
                objs[i].velocity -= 1.5 * normal_speed * normal; // Bounce factor
            }
    
            //Friction
            let tangent_velocity = velocity - normal * normal_speed;
            objs[i].velocity -= 0.01 * tangent_velocity; 
        }
        
        objs[i].velocity += Vec3::new(0.0, -2.0, 0.0) * delta_time;
        let vel = objs[i].velocity;

        objs[i].translate(vel * delta_time);
    }
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

fn create_render_textures(display: &Display<WindowSurface>, width: u32, height: u32) -> (Texture2d, DepthTexture2d) {
    let color_texture = Texture2d::empty(display, width, height).unwrap();
    let depth_texture = DepthTexture2d::empty(display, width, height).unwrap();
    (color_texture, depth_texture)
}

fn create_fbo<'a>(
    display: &'a Display<WindowSurface>,
    color_texture: &'a Texture2d,
    depth_texture: &'a DepthTexture2d,
) -> SimpleFrameBuffer<'a> {
    SimpleFrameBuffer::with_depth_buffer(display, color_texture, depth_texture).unwrap()
}