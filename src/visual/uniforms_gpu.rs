// 📦 Código para gestionar un buffer de Uniforms en wgpu compatible con WGSL
// El struct Uniforms ya está definido en visual/uniforms.rs con #[repr(C)] y bytemuck

use wgpu::util::DeviceExt;
use crate::visual::uniforms::Uniforms;

pub struct UniformsGpu {
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl UniformsGpu {
    pub fn new(device: &wgpu::Device, uniforms: &Uniforms) -> Self {
        // Crea el buffer con los datos iniciales
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Layout para shaders @group(0) @binding(0)
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Grupo de bind que usará el shader
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update(&self, queue: &wgpu::Queue, new_uniforms: &Uniforms) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[*new_uniforms]));
    }
}
