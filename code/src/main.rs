#[macro_use]
extern crate glium;
extern crate winit;
use object::{point::WorldPoint, WorldObject};
use rand::distr::{Distribution, Uniform};
use glam::{Mat4, Vec2, Vec3};
use util::{input_handler::{InputHandler}, ray_library::ndc_to_intersection};
use winit::{event_loop::{ControlFlow, EventLoop}, keyboard, window::{Fullscreen, Window}};
use glium::{glutin::surface::WindowSurface, implement_vertex, uniforms::{MagnifySamplerFilter, MinifySamplerFilter}, Display, Surface, VertexBuffer};
use world::{hex::Hex, layout::{HexLayout, Point, EVEN}, offset_coords::{qoffset_from_cube_offsets, qoffset_to_cube, qoffset_to_cube_offsets}, tile::Tile, world_camera::WorldCamera, OffsetTile, NUM_COLMS, NUM_ROWS};
use std::{time::{Instant}};

mod teapot;
mod object;
mod rendering;
use rendering::{render::{array_to_vbo, Vertex}, render_camera::RenderCamera, text::{format_to_exact_length, RenderedText, TextVbo}};


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
        Vertex{position: [0.5, -0.5, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 0.0]}, 
        Vertex{position: [0.5, 0.5, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 1.0]},
        Vertex{position: [-0.5, 0.5, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 1.0]},
        Vertex{position: [-0.5, -0.5, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 0.0]}
    ]; 


    let obj_vert = util::read_shader("./shaders/vert2.glsl");
    let obj_frag = util::read_shader("./shaders/frag2.glsl");

    let point_vert = util::read_shader("./shaders/point_vert.glsl");
    let point_frag = util::read_shader("./shaders/point_frag.glsl");

    let line_vert_shader = util::read_shader("./shaders/line_vert.glsl");
    let line_frag_shader = util::read_shader("./shaders/line_frag.glsl");

    let text_vert_shader  = util::read_shader("./shaders/text_vert.glsl");
    let text_frag_shader  = util::read_shader("./shaders/text_frag.glsl");

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

    let text_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let point_params = glium::DrawParameters {
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

    let point = WorldPoint::new(0.5,Vec2::ZERO,Vec3::ZERO);

    //Shape of quad
    let quad_shape:Vec<Vertex> = vec![
        Vertex{position: [-1.0*0.1, -1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 0.0]}, 
        Vertex{position: [1.0*0.1, -1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 0.0]},
        Vertex{position: [1.0*0.1, 1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 1.0]},
        Vertex{position: [-1.0*0.1, 1.0*0.1, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 1.0]}
    ];
    
    let quad_indicies = vec![0, 2, 1, 0, 2, 3];

    let obj_renderer = rendering::render::Renderer::new(&tea_positions, &tea_indices, Some(glium::index::PrimitiveType::TrianglesList), &obj_vert, &obj_frag, None, &display, None).unwrap();
    let mut line_renderer = rendering::render::Renderer::new_empty_dynamic(100, Some(glium::index::PrimitiveType::LinesList), &line_vert_shader, &line_frag_shader, None, &display, Some(line_params)).unwrap();
    let ui_renderer = rendering::render::Renderer::new_empty_dynamic(100, Some(glium::index::PrimitiveType::TrianglesList), &line_vert_shader, &line_frag_shader, None, &display, None).unwrap();
    let text_renderer = rendering::render::Renderer::new(&quad_shape, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &text_vert_shader, &text_frag_shader, None, &display, Some(text_params)).unwrap();
    let point_renderer = rendering::render::Renderer::new(&quad_vertex, &quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &point_vert, &point_frag, None, &display, Some(point_params)).unwrap();


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


    let mut mouse_pos: Point = Point{x:0.0,y:0.0};
    let mut mouse_ndc: Vec3 = Vec3::ZERO;

    let mut t: f32 = 0.0;
    let dt: f32 = 0.0167;

    let mut current_time = Instant::now();
    let mut accumulator: f32 = 0.0;

    let mut total_fps: usize = 0;

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
                

                // Still some problem with this code?
                // Could probably be some rounding errors...
                // How could one fix this?
                // Scale everything maybe to use bigger numbers?
                mouse_ndc = Vec3::new(
                    (position.x as f32 / window.inner_size().width as f32) * 2.0 - 1.0,
                    -((position.y as f32 / window.inner_size().height as f32) * 2.0 - 1.0),
                    0.0,
                );

                let intersect = ndc_to_intersection(&mouse_ndc,&camera.camera_matrix,camera.get_pos(),&camera.perspective);


                mouse_pos.x = intersect.x as f32;
                mouse_pos.y = intersect.y as f32;
            }
            winit::event::WindowEvent::MouseInput { device_id: _, state, button } =>{
                if state.is_pressed(){

                    //get_clicked_pos(&mut mouse_pos, &mut world_camera);

                }else{

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
                    camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                    //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&camera.perspective)*camera.camera_matrix*Mat4::IDENTITY));
                }
                else if event.physical_key == keyboard::KeyCode::KeyE{
                    camera.r#move(CAMERA_SPEED*camera.get_front());
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
                    cube_object.rotate(Vec3::from_array([1.0,0.0,1.0]).normalize(), 4.1887902);
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

                    update_game_logic(dt, &mut camera, &mut world_camera, &input_handler, mouse_ndc, &mut mouse_pos); 
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
                
                target.draw(&line_renderer.vbo, &line_renderer.indicies, &line_renderer.program, &uniform! {}, &line_renderer.draw_params).unwrap();
                target.draw(&point_renderer.vbo, &point_renderer.indicies, &point_renderer.program, &uniform!{radius: point.get_radius(), model: point.get_model().to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &point_renderer.draw_params).unwrap();
                
                target.draw(&obj_renderer.vbo, &obj_renderer.indicies, &obj_renderer.program, &uniform! { u_light: light, model: cube_object.get_model().to_cols_array_2d(), projection: camera.perspective.to_cols_array_2d(), view:camera.camera_matrix.to_cols_array_2d()}, &Default::default()).unwrap();
                
                
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


fn update_game_logic(delta_time: f32, camera: &mut RenderCamera,world_camera: &mut WorldCamera,input_handler: &InputHandler,mouse_ndc:Vec3, mouse_pos: &mut Point){
    //Update movement (Kanske göra efter allt annat... possibly):
    let mut movement = input_handler.get_movement();
    if movement.length() > 0.0{
        let mut traveresed_whole_hex = false;
        movement = movement.normalize();
        //Flytta en i taget...
        camera.r#move(delta_time*movement[1]*CAMERA_SPEED*camera.get_up());
        let y_pos = camera.get_pos()[1];
        camera.r#move(delta_time*movement[0]*CAMERA_SPEED*(camera.get_front().cross(camera.get_up())).normalize());
            let x_pos = camera.get_pos()[0];
                //Kom på varför det är 0.12 här och inget annat nummer...
                //Verkar ju bara bero på hex_size och inte scale....
            let intersect = ndc_to_intersection(&mouse_ndc,&camera.camera_matrix,camera.get_pos(),&camera.perspective);
            mouse_pos.x = intersect.x as f32;
            mouse_pos.y = intersect.y as f32;
            camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
                //inverse_mat = Mat4::inverse(&(Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::IDENTITY));
        }
}

pub fn get_clicked_pos(layout: &HexLayout, mouse_pos: &mut Point, world_camera: &mut WorldCamera) -> OffsetTile{
    //println!("Dimension is: {:#?}", window.inner_size());
    let frac_hex = layout.pixel_to_hex(&mouse_pos);
    let clicked_hex = frac_hex.hex_round();
    
    let (mut clicked_y, mut clicked_x) = qoffset_from_cube_offsets(EVEN,&clicked_hex);                    
    //Make these not hard coded...
    // And move out into seperate function
    clicked_y = 25 - clicked_y as isize;
    clicked_x = 12 - clicked_x as isize;

    let camera_offsets = world_camera.offsets();

    //Make these then loop when crossing over the boundary.
    clicked_x += camera_offsets.1; 
    clicked_y += camera_offsets.0;

    if clicked_x <= 0{
        clicked_x = ((NUM_COLMS) as isize + clicked_x) % NUM_COLMS as isize;
    }else if clicked_x >= NUM_COLMS as isize{
        clicked_x = (clicked_x - (NUM_COLMS) as isize) % NUM_COLMS as isize;
    }  
    

    if clicked_y <= 0{
        clicked_y = ((NUM_ROWS) as isize + clicked_y) % NUM_ROWS as isize;
    }else if clicked_y >= NUM_ROWS as isize{
        clicked_y = (clicked_y - (NUM_ROWS) as isize) % NUM_ROWS as isize;
    }  

    return OffsetTile::new(clicked_y as u32, clicked_x as u32)
}