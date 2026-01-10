use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::ActiveEventLoop,
    event_loop::EventLoop,
    keyboard::KeyCode,
    keyboard::PhysicalKey,
    window::Window,
};
use wgpu::util::DeviceExt;

mod camera;
use camera::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices: [Vertex; 6],
    num_vertices: u32,
    camera: camera::camera::Camera,
    camera_matrix: [[f32; 3]; 3],
    delta_time: std::time::Instant,
    frame_times: FrameTimes,
}

struct FrameTimes {
    delta_time: std::time::Instant,
    delta_time_sum: std::time::Duration,
    update_time: std::time::Duration,
    sample_size: u32,
}

impl FrameTimes {
    fn reset(&mut self) {
        self.delta_time = std::time::Instant::now();
        self.delta_time_sum = std::time::Duration::from_secs(0);
        self.sample_size = 1;
    }

    fn new_sample(mut self) -> Self {
        self.delta_time = std::time::Instant::now();
        self.sample_size = self.sample_size + 1;
        
        self
    }

}



pub struct FpsCounter {}


pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    camera_matrix: [[f32; 3]; 3],
    scale_factor: f32,
}

/*
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    position: [f32; 3],
    angle_horizontal: f32,
    angle_vertical: f32,
    scale_factor: f32,
}
*/

/*
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraMatrixElements {
    sin_horizontal: f32,
    cos_horizontal: f32,
    sin_vertical: f32,
    cos_vertical: f32,
}

impl CameraMatrixElements {
    fn new(angle_horizontal: f32, angle_vertical: f32) -> Self {
        Self {
            sin_horizontal: f32::sin(angle_horizontal),
            cos_horizontal: f32::cos(angle_horizontal),
            sin_vertical: f32::sin(angle_horizontal),
            cos_vertical: f32::cos(angle_horizontal),
        }
    }
}
*/

impl Vertex {
    fn descriptor() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        use wgpu::{
            VertexAttribute,
            BufferAddress,
            VertexFormat,
            VertexStepMode,
            VertexBufferLayout,
        };

        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: size_of::<[f32; 6]>() as BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: size_of::<[f32; 9]>() as BufferAddress,
                    shader_location: 3,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 4,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: size_of::<[f32; 15]>() as BufferAddress,
                    shader_location: 5,
                    format: VertexFormat::Float32,
                },
            ]
        }
    }
}

const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.10,
    g: 0.60,
    b: 1.00,
    a: 1.00,
};

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        use std::f32::consts::PI;

        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await?;

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        }).await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        
        let camera = camera::camera::Camera::new([0.0, 0.0, 0.0], 0.0, 0.0, 5.0);

        let camera_matrix = camera.matrix();

        let scale_factor = 1.0;

        let vertices = [
            Vertex { position: [0.0, 0.0, 3.0], color: [1.0, 1.0, 1.0], camera_matrix, scale_factor },
            Vertex { position: [1.0, 0.0, 3.0], color: [1.0, 0.4, 1.0], camera_matrix, scale_factor },
            Vertex { position: [0.0, 1.0, 3.0], color: [0.4, 0.0, 1.0], camera_matrix, scale_factor },
            
            Vertex { position: [-0.7, -0.3, 3.5], color: [1.0, 0.0, 0.0], camera_matrix, scale_factor },
            Vertex { position: [0.2, -0.5, 3.5], color: [0.0, 1.0, 0.0], camera_matrix, scale_factor },
            Vertex { position: [0.1, 1.0, 5.5], color: [0.0, 0.0, 1.0], camera_matrix, scale_factor },
        ];

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[
                        Vertex::descriptor(),
                    ],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });

        let num_vertices = vertices.len() as u32;

        let delta_time = std::time::Instant::now();

        let frame_times = FrameTimes {
            delta_time: std::time::Instant::now(),
            delta_time_sum: std::time::Duration::from_secs(0),
            update_time: std::time::Duration::from_secs(1),
            sample_size: 1,
        };

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            render_pipeline,
            vertex_buffer,
            vertices,
            num_vertices,
            camera,
            camera_matrix,
            delta_time,
            frame_times,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            (KeyCode::KeyW, true) => self.camera.angle_v = self.camera.angle_v + 0.03,
            (KeyCode::KeyA, true) => self.camera.angle_h = self.camera.angle_h + 0.03,
            (KeyCode::KeyS, true) => self.camera.angle_v = self.camera.angle_v - 0.03,
            (KeyCode::KeyD, true) => self.camera.angle_h = self.camera.angle_h - 0.03,
            _ => {}
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn update(&mut self) {
        self.frame_times.sample_size = self.frame_times.sample_size + 1;
        if self.frame_times.delta_time.elapsed() >= std::time::Duration::from_millis(500) {
            println!("{:?}", self.frame_times.sample_size * 2);
            self.frame_times.sample_size = 0;
            self.frame_times.delta_time = std::time::Instant::now();
        }

        self.frame_times.sample_size = self.frame_times.sample_size + 1;
        
        //println!("{:?}", self.delta_time.elapsed());
        //self.delta_time = std::time::Instant::now();

        let camera_matrix = self.camera.matrix();
        for i in 0..6 {
            self.vertices[i].camera_matrix = camera_matrix;
        }

        self.vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
    }
}

impl App {
    pub fn new(
        #[cfg(target_arch = "wasm32")]
        event_loop: &EventLoop<State>,
    ) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_elemt = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_elements));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state = Some(pollster::block_on(State::new(window)).unwrap());
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(proxy.send_event(State::new(window).await.expect("Unable to create canvas!!!!!")).is_ok()) 
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            even.resize(
                event.window.inner_size().width,
                event.window.inner_size().height,
            );
        }
        
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => match (button, state.is_pressed()) {
                (MouseButton::Left, true) => {}
                (MouseButton::Left, false) => {}
                _ => {}
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: key_state,
                    ..
                },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {} 
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();
    
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(log::Level::Info).unwarp_throw();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(
        #[cfg(target_arch = "wasm32")]
        &event_loop,
    );
    event_loop.run_app(&mut app)?;

    Ok(())
}

