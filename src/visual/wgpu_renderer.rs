// src/visual/wgpu_renderer.rs
// Renderizador base con wgpu y shaders WGSL para visualización robusta y moderna

use wgpu::util::DeviceExt;
use wgpu::*;
use crate::visual::uniforms::Uniforms;
use crate::visual::uniforms_gpu::UniformsGpu;

pub struct WgpuRenderer {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface,
    pub config: SurfaceConfiguration,
    pub uniforms: Uniforms,
    pub uniforms_gpu: UniformsGpu,
    pub shader_module: ShaderModule,
    pub pipeline: RenderPipeline,
}

impl WgpuRenderer {
    pub fn new(window: &winit::window::Window) -> Self {
        // Instancia wgpu
        let instance = Instance::default();
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
            features: Features::empty(),
            limits: Limits::default(),
            label: None,
        }, None)).unwrap();

        // Configuración de la superficie
        let size = window.inner_size();
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Uniforms y buffer GPU
        let uniforms = Uniforms::new();
        let uniforms_gpu = UniformsGpu::new(&device, &uniforms);

        // Cargar shader WGSL
        let shader_module = device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("Main Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/vertex.wgsl").into()),
        });

        // Crear pipeline
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&uniforms_gpu.bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Main Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[], // Define tus buffers de vértices aquí
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(config.format.into())],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        Self {
            device,
            queue,
            surface,
            config,
            uniforms,
            uniforms_gpu,
            shader_module,
            pipeline,
        }
    }

    pub fn render(&mut self) {
        // Actualiza uniforms dinámicos
        // self.uniforms.update_from_audio(...); // Llama según tu lógica
        self.uniforms_gpu.update(&self.queue, &self.uniforms);

        // Prepara frame
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Render Encoder") });

        // Render pass
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.uniforms_gpu.bind_group, &[]);
            // render_pass.draw(...); // Aquí van tus draw calls
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
