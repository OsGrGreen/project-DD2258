#[macro_use]
extern crate glium;
extern crate winit;
use grass::get_grass_shape;
use object::{point::WorldPoint, WorldObject};
use rand::Rng;
use glam::{Mat4, Vec2, Vec3};
use util::{input_handler::InputHandler, ray_library::{distance_ray_point, ndc_to_direction, ndc_to_point}};
use winit::{event::{MouseButton, MouseScrollDelta}, event_loop::{ControlFlow, EventLoop}, keyboard, window::{Fullscreen, Window}};
use glium::{framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, implement_vertex, index::PrimitiveType, texture::DepthTexture2d, uniforms::{MagnifySamplerFilter, MinifySamplerFilter}, Blend, BlendingFunction, Display, LinearBlendingFactor, Surface, Texture2d, VertexBuffer};
use core::f32;
use std::time::Instant;


mod object;
mod grass;
mod rendering;
mod bezier_surface;
use bezier_surface::GrassVertex;
use rendering::{render::{Vertex, VertexSimple}, render_camera::RenderCamera, text::{format_to_exact_length, RenderedText, TextVbo}};


mod util;


#[derive(Copy, Clone, Debug)]
pub struct Attr {
    world_position: [f32; 3],
    colour: [f32; 3],
    tex_offsets: [f32;3], //x offset, y offset, scaling factor   For reading in texture atlas
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

    //The camera
    let mut camera = RenderCamera::new(Vec3::new(0.0,0.5,4.5), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0,0.0,-1.0));

    // Input handler
    let mut input_handler = InputHandler::new();

    //Set up camera matrix
    camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
    //Create eventloop, window and display (where everything renders)
    let (event_loop, window, display) = init_window();
    //For manipulating the window
    let monitor_handle = window.primary_monitor();

    window.set_fullscreen(Some(Fullscreen::Borderless(monitor_handle)));


    //This is literally the same as further down...
    //I dont remember why this is here...
    let quad_vertex = vec![
        Vertex{position: [0.1, -0.1, 0.0], normal: [0.1,0.1,0.0], tex_coords: [1.0, 0.0]}, 
        Vertex{position: [0.1, 0.1, 0.0], normal: [0.1,0.1,0.0], tex_coords: [1.0, 1.0]},
        Vertex{position: [-0.1, 0.1, 0.0], normal: [0.1,0.1,0.0], tex_coords: [0.0, 1.0]},
        Vertex{position: [-0.1, -0.1, 0.0], normal: [0.1,0.1,0.0], tex_coords: [0.0, 0.0]}
    ]; 

    /*Loading the shaders from the files */
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

    let grass_vert = util::read_shader(include_bytes!(r"../shaders/grass/grass_vert.glsl"));
    let grass_frag = util::read_shader(include_bytes!(r"../shaders/grass/grass_frag.glsl"));

    let low_res_vert = util::read_shader(include_bytes!(r"../shaders/resolution/vert.glsl"));
    let low_res_frag = util::read_shader(include_bytes!(r"../shaders/resolution/frag.glsl"));


    // Setup specific parameters
    let light = [-1.0, -0.5, 0.0f32];

    /*Draw paramters for the different renderers */
    /*For example if a line or not */
    /*If alpha is needed, and the depthtest */

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
    
        //Font textures
        // Font chars are of size 12 x 6
    let font_raw_image = image::load(std::io::Cursor::new(&include_bytes!(r"textures\standard_font.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let font_dimensions = font_raw_image.dimensions();
    let font_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&font_raw_image.into_raw(), font_dimensions);
    let font_atlas = glium::texture::Texture2d::new(&display, font_image).unwrap();

    //Texture of the grass billboards
    let grass_raw_image = image::load(std::io::Cursor::new(&include_bytes!(r"textures\grass2.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let grass_dimensions = grass_raw_image.dimensions();
    let grass_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&grass_raw_image.into_raw(), grass_dimensions);
    let grass_texture = glium::texture::Texture2d::new(&display, grass_image).unwrap();


    //Textures for loading the "presentation"
    //Removed to make boot up quicker!
    /*let goal_raw = image::load(std::io::Cursor::new(&include_bytes!(r"textures\whatIwant.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let goal_dimensions = goal_raw.dimensions();
    let goal_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&goal_raw.into_raw(), goal_dimensions);
    let goal_texture = glium::texture::Texture2d::new(&display, goal_image).unwrap();

    let succes_raw = image::load(std::io::Cursor::new(&include_bytes!(r"textures\whatIdid.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let succes_dimensions = succes_raw.dimensions();
    let succes_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&succes_raw.into_raw(), succes_dimensions);
    let succes_texture = glium::texture::Texture2d::new(&display, succes_image).unwrap();

    let result2_raw = image::load(std::io::Cursor::new(&include_bytes!(r"textures\result2.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let result2_dimensions = result2_raw.dimensions();
    let result2_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&result2_raw.into_raw(), result2_dimensions);
    let result2_texture = glium::texture::Texture2d::new(&display, result2_image).unwrap();

    let surface_raw = image::load(std::io::Cursor::new(&include_bytes!(r"textures\surfacequads.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let surface_dimensions = surface_raw.dimensions();
    let surface_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&surface_raw.into_raw(), surface_dimensions);
    let surface_texture = glium::texture::Texture2d::new(&display, surface_image).unwrap();

    let result1_raw = image::load(std::io::Cursor::new(&include_bytes!(r"textures\result1.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let result1_dimensions = result1_raw.dimensions();
    let result1_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&result1_raw.into_raw(), result1_dimensions);
    let result1_texture = glium::texture::Texture2d::new(&display, result1_image).unwrap();*/

    /*The point that symbolises where the mouse is! */
    let mut point = WorldPoint::new(0.5,Vec2::ZERO,Vec3::ZERO);

    //Shape of quad that is smaller (I know I could have used one and just scaled it... But I was really stressed)
    let quad_shape:Vec<Vertex> = vec![
        Vertex{position: [-1.0*0.1, -1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 0.0]}, 
        Vertex{position: [1.0*0.1, -1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 0.0]},
        Vertex{position: [1.0*0.1, 1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 1.0]},
        Vertex{position: [-1.0*0.1, 1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 1.0]}
    ];

    //Quad that covers the whole screen
    let screen_quad:Vec<Vertex> = vec![
            Vertex{position: [-1.0, -1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 0.0]}, 
            Vertex{position: [1.0, -1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 0.0]},
            Vertex{position: [1.0, 1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 1.0]},
            Vertex{position: [-1.0, 1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 1.0]}
    ];
    
    let quad_indicies = vec![0, 2, 1, 0, 2, 3];

    //The different "renderers"
    // Is basically a combination of VBO, IndexBuffer and Program
    // Is a handy way to have everything in one place..
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

    /*Bezier surface variables */
    let mut bezier_quads = 1;
    let mut grass_density = 16;
    let mut bezier_surface= bezier_surface::Surface::new(Vec3::new(-1.0, 0.0, -1.0),2.0,2.0,1,1); 

    /*The bezier surface points as a VertexBuffer and IndexBuffer */
    /*Is not saved in the usual way since I update them reguarly when I create more quads*/
    let mut surface_vbo: VertexBuffer<VertexSimple> = glium::VertexBuffer::new(&display, &bezier_surface.points).unwrap();
    let mut surface_indicies = glium::IndexBuffer::new(&display,PrimitiveType::Patches {vertices_per_patch: 16,},
    &bezier_surface.inds).unwrap();

    //Here I tried to get the posistions from the calculated bezier surface posistions from the tesselation shader.
    //However, GLIUM (openGL bindings for rust) does not currently support that these TransFormFeedbacks return quads :(
    // So this was not possible
    let surface_renderer = rendering::render::Renderer::new(&vec![], &vec![], Some(PrimitiveType::Patches {vertices_per_patch: 16,}), &surface_vert_shader, &surface_frag_shader, /*Some(surface_geometry_shader)*/None, Some(&surface_tess_ctrl_shader), Some(&surface_tess_eval_shader), &display, None, Some((
        vec![(&"out_point").to_string()], 
        glium::program::TransformFeedbackMode::Interleaved,
    ))).unwrap();
    
    /*Init grass posistions and its related Vertex Buffer (with instances) */
    let mut grass_pos:Vec<GrassVertex> = bezier_surface.get_grass_posistions(grass_density);
    let mut grass_instances = glium::VertexBuffer::new(&display, &grass_pos).unwrap();

    /*Add the text object that renders the FPS */
    let mut fps_text = RenderedText::new(String::from("00000fps"));
    let mut text_vbo = TextVbo::new(100, &display);
    text_vbo.add_text((0.78,0.95), 0.085, Some([1.0,0.5,1.0]), &mut fps_text);
        /*Text behaviour (for magnifying and so on, needed since the font is blocky) */
    let text_behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };
    

    camera.perspective = rendering::render::calculate_perspective(window.inner_size().into());
    
    /* Init some falling spheres */
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

    /* Vector/array that stores the initial objects */
    let mut world_objs: Vec<WorldObject> = vec![obj1, obj2, obj3, obj4];

    /*Used for presentation mode */
    let mut selected_tex = -1;

    /*Some random variables that are used through out the program */
    let mut mouse_pos: Vec3 = Vec3::ZERO;
    let mut render_grass = true;
    let mut t: f32 = 0.0;
    let dt: f32 = 0.0167;
    let mut prev_mouse_pos = Vec3::ZERO;
    let mut current_time = Instant::now();
    let mut accumulator: f32 = 0.0;
    let mut ctrl_pressed = false;
    let mut total_fps: usize = 0;
    let mut timer = Instant::now();
    let mut overall_fps = 0.0;
    let mut rng = rand::rng();
    let smoothing = 0.6;  //For fps
    let _light = [-1.0, 0.4, 0.9f32];
    let mut frames:f32 = 0.0;


    let _ = event_loop.run(move |event, window_target| {
        let (world_texture, depth_world_texture) = create_render_textures(&display, 384, 216);
        let mut fbo = create_fbo(&display, &world_texture, &depth_world_texture);
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                /*Window exit event */
                println!("Average fps was: {}", total_fps/frames as usize);
                window_target.exit()
            },
            winit::event::WindowEvent::CursorMoved { device_id: _, position } => {
                /*Move cursor event */
                prev_mouse_pos = mouse_pos;

                /*Get mouse posistion in NDC */
                mouse_pos = Vec3::new(
                    (position.x as f32 / window.inner_size().width as f32) * 2.0 - 1.0,
                    - ((position.y as f32 / window.inner_size().height as f32) * 2.0 - 1.0),
                    1.0,
                );

                if ctrl_pressed && selected_point != -1{
                    /* If CTRL is pressed then update the selected point... */
                    let dist = (mouse_pos-prev_mouse_pos).normalize();
                    println!("Dist is {:#?}", dist);
                    bezier_surface.points[selected_point as usize].w_position[0] += dist.x;
                    bezier_surface.points[selected_point as usize].w_position[1] += dist.y;
                    

                    grass_instances = glium::VertexBuffer::new(&display, &bezier_surface.get_grass_posistions(grass_density)).unwrap();
                    surface_vbo = glium::VertexBuffer::new(&display, &bezier_surface.points).unwrap();
                }
                
            }
            winit::event::WindowEvent::MouseWheel { device_id: _, delta, phase } =>{
                if selected_point != -1 && ctrl_pressed{
                    match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            bezier_surface.points[selected_point as usize].w_position[2] += y;

                            //Unneccessary to redo whole VertexBuffer since the Bezier surface is not C1 cont.
                            // Which means that we only have to update one singular quad
                            //However, is good enough for right now
                            surface_vbo = glium::VertexBuffer::new(&display, &bezier_surface.points).unwrap();
                        }
                        _ => {}
                    }
                }
            }
            winit::event::WindowEvent::MouseInput { device_id: _, state, button } =>{
                if state.is_pressed() && button == MouseButton::Left{
                    let mouse_dir = ndc_to_direction(&mouse_pos, &camera.camera_matrix, &camera.perspective);
                    let mouse_point = camera.get_pos();
                    let mut i  = -1;
                    let mut min_dist = 100.0;
                    let mut min_ind = -1;
                    for point in bezier_surface.points.iter(){
                        i += 1;
                        /*
                        For some reason this is behaves a little weird.
                        Possibly since I never update camera.up hehe
                         */
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

                /*
                Key inputs.
                Please see README for the relevant commands/instruction
                 */
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
                    if render_grass{
                        grass_instances = glium::VertexBuffer::new(&display, &bezier_surface.get_grass_posistions(grass_density)).unwrap();
                    }
                    surface_vbo = glium::VertexBuffer::new(&display, &bezier_surface.points).unwrap();
                    surface_indicies = glium::IndexBuffer::new(&display,PrimitiveType::Patches {vertices_per_patch: 16,},
                        &bezier_surface.inds).unwrap();
                }else if event.physical_key == keyboard::KeyCode::ArrowLeft && event.state.is_pressed(){
                    bezier_quads -= 1;
                    println!("Num quads is {}", bezier_quads*bezier_quads);
                    bezier_surface = bezier_surface::Surface::new(Vec3::new(-1.0, 0.0, -1.0),2.0,2.0,bezier_quads,bezier_quads); 
                    if render_grass{
                        grass_instances = glium::VertexBuffer::new(&display, &bezier_surface.get_grass_posistions(grass_density)).unwrap();
                    }
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
                }else if event.physical_key == keyboard::KeyCode::Backspace && event.state.is_pressed(){
                    render_grass = !render_grass;
                }else if event.physical_key == keyboard::KeyCode::Digit1 && event.state.is_pressed(){
                    selected_tex = 1;
                }else if event.physical_key == keyboard::KeyCode::Digit2 && event.state.is_pressed(){
                    selected_tex = 2;
                }else if event.physical_key == keyboard::KeyCode::Digit3 && event.state.is_pressed(){
                    selected_tex = 3;
                }else if event.physical_key == keyboard::KeyCode::Digit4 && event.state.is_pressed(){
                    selected_tex = 4;
                }else if event.physical_key == keyboard::KeyCode::Digit5 && event.state.is_pressed(){
                    selected_tex = 5;
                }else if event.physical_key == keyboard::KeyCode::Digit6 && event.state.is_pressed(){
                    selected_tex = -1;
                }else if event.physical_key == keyboard::KeyCode::CapsLock && event.state.is_pressed(){
                    let mut obj_new: WorldObject = WorldObject::new();
                    obj_new.scale(Vec3::new(10.0, 10.0, 10.0));
                    obj_new.set_translation(bezier_surface.start_pos + Vec3::new(rng.random_range(0.0..bezier_surface.num_quads_x as f32*bezier_surface.step_size_x),5.0,rng.random_range(0.0..bezier_surface.num_quads_z as f32*bezier_surface.step_size_z))); 
                    world_objs.push(obj_new);
                }
                //Handle WASD

                input_handler.update_input(event);

                //Handle inputs ends

            },
            winit::event::WindowEvent::Resized(window_size) => {
                camera.perspective = rendering::render::calculate_perspective(window_size.into());
                display.resize(window_size.into());
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

                while accumulator >= dt {

                    update_game_logic(dt, &mut camera, &input_handler, &mut mouse_pos, &mut point, &window);
                    if selected_tex == -1{
                        update_obj_physics(dt, &mut bezier_surface, &mut world_objs);
                    }
                    t += dt;
                    accumulator -= dt;
                }
                

                //Render step

                let delta_time = timer.elapsed().as_secs_f32();
                timer = Instant::now();
                let current = 1.0 / delta_time;
                overall_fps = ((overall_fps * smoothing) + (current * (1.0-smoothing))).min(50_000.0);
                total_fps += overall_fps as usize;
                let fps_as_text = format_to_exact_length(overall_fps as u32, 5) + "fps";
                fps_text.change_text(fps_as_text);

                text_vbo.replace_text(&fps_text);           
    
                let mut target = display.draw();

                fbo.clear_color_and_depth((0.0, 0.1, 1.0, 1.0), 1.0);
                target.clear_color_and_depth((0.0, 0.1, 1.0, 1.0), 1.0);


                let surface_params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    .. Default::default()
                };

                fbo.draw(&surface_vbo, &surface_indicies, &surface_renderer.program, &uniform! {object_color: [0.3 as f32,0.8 as f32,0.0 as f32], u_light: light, steps: 32.0 as f32, model: Mat4::IDENTITY.to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &surface_params).unwrap();

                if render_grass{
                    fbo.draw((&grass_renderer.vbo, grass_instances.per_instance().unwrap()), &grass_renderer.indicies, &grass_renderer.program, &uniform! {u_light: light, threshhold: 0.5 as f32, strength: 0.08 as f32,u_time: t, tex: &grass_texture, u_light: light, model: (Mat4::IDENTITY).to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &grass_renderer.draw_params).unwrap();
                }

                fbo.draw((&mult_point_renderer.vbo, surface_vbo.per_instance().unwrap()), &mult_point_renderer.indicies, &mult_point_renderer.program, &uniform! {selected: selected_point, model: (0.1*Mat4::IDENTITY).to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &mult_point_renderer.draw_params).unwrap();

                for obj in &world_objs{
                    fbo.draw(&obj_renderer.vbo, &obj_renderer.indicies, &obj_renderer.program, &uniform! { u_light: light, model: obj.get_model().to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &obj_renderer.draw_params).unwrap();

                }
                if selected_tex == -1{
                    target.draw(&low_res_renderer.vbo, &low_res_renderer.indicies,&low_res_renderer.program, &uniform! {tex: &world_texture}, &low_res_renderer.draw_params).unwrap();

                    target.draw(&point_renderer.vbo, &point_renderer.indicies, &point_renderer.program, &uniform!{radius: point.get_radius(), model: point.get_model().get_model().to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &point_renderer.draw_params).unwrap();
                    target.draw(&ui_renderer.vbo, &ui_renderer.indicies, &ui_renderer.program, &uniform! {tex:&font_atlas}, &Default::default()).unwrap();
                    target.draw((&text_renderer.vbo, text_vbo.vbo.per_instance().unwrap()), &text_renderer.indicies, &text_renderer.program, &uniform! {tex:glium::uniforms::Sampler(&font_atlas, text_behavior)}, &text_renderer.draw_params).unwrap();
                }
                //Code for printing my "presentation"

                /*else if selected_tex == 1{
                    target.draw(&low_res_renderer.vbo, &low_res_renderer.indicies,&low_res_renderer.program, &uniform! {tex: &goal_texture}, &low_res_renderer.draw_params).unwrap();
                }else if selected_tex == 2{
                    target.draw(&low_res_renderer.vbo, &low_res_renderer.indicies,&low_res_renderer.program, &uniform! {tex: &succes_texture}, &low_res_renderer.draw_params).unwrap();
                }else if selected_tex == 3{
                    target.draw(&low_res_renderer.vbo, &low_res_renderer.indicies,&low_res_renderer.program, &uniform! {tex: &result2_texture}, &low_res_renderer.draw_params).unwrap();
                }else if selected_tex == 4{
                    target.draw(&low_res_renderer.vbo, &low_res_renderer.indicies,&low_res_renderer.program, &uniform! {tex: &surface_texture}, &low_res_renderer.draw_params).unwrap();
                }else if selected_tex == 5{
                    target.draw(&low_res_renderer.vbo, &low_res_renderer.indicies,&low_res_renderer.program, &uniform! {tex: &result1_texture}, &low_res_renderer.draw_params).unwrap();
                }*/
                
            
                target.finish().unwrap();
                frames = frames + 1.0;
            },
            _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}

fn update_obj_physics(delta_time: f32, surface: &mut bezier_surface::Surface, objs:&mut Vec<WorldObject>){
    const EPS: f32 = 0.001;
    const SPHERE_RADIUS: f32 = 0.25;
    let mut distance_vec: Vec<Vec<usize>> = Vec::with_capacity(objs.len());

    // Kind of inefficient but I had to quickly make this work
    // It works and that is good enough for me...
    // I will probably make this better someday
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
        let pos: Vec3 = objs[i].get_posistion();

        //Collision
        for j in 0..distance_vec[i].len(){
                    let col_index = distance_vec[i][j];
            let dist = objs[i].get_posistion()-objs[col_index].get_posistion();
            let distance = dist.length();
            let collision_normal = dist.normalize();
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
        let to_sphere = objs[i].get_posistion() - surface_point;
        let distance = to_sphere.length();
        let normal = -d_u.cross(d_v).normalize();

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
                objs[i].velocity -= 1.5 * normal_speed * normal;
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

/* The update logic for updating the camera */
/* It is kind of scuffed, but it almost works soooo */
fn update_game_logic(delta_time: f32, camera: &mut RenderCamera,input_handler: &InputHandler,mut mouse_pos:&mut Vec3, mouse_point: &mut WorldPoint, window: &Window){
    let mut movement = input_handler.get_movement();
    if movement.length() > 0.0{
        movement = movement.normalize();
        let front = -(camera.get_pos() - mouse_point.get_model().get_posistion()).normalize();

        camera.r#move(delta_time*movement[1]*CAMERA_SPEED*camera.get_up());
        camera.r#move(delta_time*movement[0]*CAMERA_SPEED*(front.cross(camera.get_up())).normalize());


    }

    let intersect = ndc_to_point(&(Vec3::new(-mouse_pos.x,-mouse_pos.y,0.0)),&camera.camera_matrix,camera.get_pos(),&camera.perspective, 5.0);
    mouse_point.get_mut_model().set_translation(intersect);
    camera.change_target(intersect);
    camera.camera_matrix = camera.look_at(intersect+camera.get_front());

}


/* Functions for creating the low-res image that I firstly render everything to */
/* Is mostly for the aesthetic, but also gives us a few more frames to work with */
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