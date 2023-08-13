use std::{mem, num::NonZeroU64};

use bytemuck::{Pod, Zeroable};
use ori_graphics::{
    math::{Mat2, Vec2},
    Affine, Color, ImageHandle, Quad, Rect,
};
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt, StagingBelt},
    vertex_attr_array, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, Buffer,
    BufferBindingType, BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites,
    CommandEncoder, Device, FragmentState, IndexFormat, MultisampleState, PipelineLayoutDescriptor,
    RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderStages, TextureFormat,
    VertexBufferLayout, VertexStepMode,
};

use crate::WgpuImage;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
struct QuadUniforms {
    resolution: Vec2,
    translation: Vec2,
    matrix: Mat2,
    min: Vec2,
    max: Vec2,
    color: Color,
    border_color: Color,
    border_radius: [f32; 4],
    border_width: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
struct QuadVertex {
    position: Vec2,
    uv: Vec2,
}

#[derive(Debug)]
struct Instance {
    uniform_buffer: Buffer,
    vertex_buffer: Buffer,
    uniform_bind_group: BindGroup,
    image: Option<ImageHandle>,
    clip: Rect,
}

impl Instance {
    fn new(device: &Device, uniform_layout: &BindGroupLayout) -> Self {
        let uniform_buffer = QuadPipeline::create_uniform_buffer(device);

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Ori Quad Pipeline Uniform Bind Group"),
            layout: uniform_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            uniform_buffer,
            vertex_buffer: QuadPipeline::create_vertex_buffer(device),
            uniform_bind_group,
            image: None,
            clip: Rect::ZERO,
        }
    }

    fn write_uniform_buffer(
        &self,
        device: &Device,
        encoder: &mut CommandEncoder,
        staging_belt: &mut StagingBelt,
        quad: &Quad,
        transform: Affine,
        resolution: Vec2,
    ) {
        let uniforms = QuadUniforms {
            resolution,
            translation: transform.translation,
            matrix: transform.matrix,
            min: quad.rect.min,
            max: quad.rect.max,
            color: quad.background_color,
            border_color: quad.border_color,
            border_radius: quad.border_radius,
            border_width: quad.border_width,
        };

        let bytes = bytemuck::bytes_of(&uniforms);

        let mut buffer = staging_belt.write_buffer(
            encoder,
            &self.uniform_buffer,
            0,
            NonZeroU64::new(bytes.len() as u64).unwrap(),
            device,
        );
        buffer.copy_from_slice(bytes);
    }

    fn write_vertex_buffer(
        &self,
        device: &Device,
        encoder: &mut CommandEncoder,
        staging_belt: &mut StagingBelt,
        quad: &Quad,
    ) {
        let vertices = QuadPipeline::quad_mesh(quad);
        let bytes = bytemuck::cast_slice(&vertices);

        let mut buffer = staging_belt.write_buffer(
            encoder,
            &self.vertex_buffer,
            0,
            NonZeroU64::new(bytes.len() as u64).unwrap(),
            device,
        );
        buffer.copy_from_slice(bytes);
    }
}

#[derive(Default, Debug)]
struct Layer {
    instances: Vec<Instance>,
    instance_count: usize,
}

impl Layer {
    const fn new() -> Self {
        Self {
            instances: Vec::new(),
            instance_count: 0,
        }
    }
}

pub struct QuadPipeline {
    uniform_layout: BindGroupLayout,
    pipeline: RenderPipeline,
    index_buffer: Buffer,
    layers: Vec<Layer>,
}

impl QuadPipeline {
    pub fn new(
        device: &Device,
        image_bind_group_layout: &BindGroupLayout,
        format: TextureFormat,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shader/quad.wgsl"));

        let uniform_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Ori Quad Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Ori Quad Pipeline Layout"),
            bind_group_layouts: &[&uniform_layout, image_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Ori Quad Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex",
                buffers: &[VertexBufferLayout {
                    array_stride: mem::size_of::<QuadVertex>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                }],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fragment",
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            multisample: MultisampleState {
                count: 4,
                ..Default::default()
            },
            depth_stencil: None,
            multiview: None,
        });

        let index_buffer = Self::create_index_buffer(device);

        Self {
            uniform_layout,
            pipeline,
            index_buffer,
            layers: Vec::new(),
        }
    }

    fn create_uniform_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Ori Quad Uniform Buffer"),
            size: mem::size_of::<QuadUniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_vertex_buffer(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Ori Quad Vertex Buffer"),
            size: mem::size_of::<QuadVertex>() as u64 * 4,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_index_buffer(device: &Device) -> Buffer {
        let indices = [0, 1, 2, 2, 3, 0];

        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Ori Quad Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        })
    }

    fn quad_mesh(quad: &Quad) -> [QuadVertex; 4] {
        [
            QuadVertex {
                position: quad.rect.top_left(),
                uv: Vec2::ZERO,
            },
            QuadVertex {
                position: quad.rect.top_right(),
                uv: Vec2::X,
            },
            QuadVertex {
                position: quad.rect.bottom_right(),
                uv: Vec2::ONE,
            },
            QuadVertex {
                position: quad.rect.bottom_left(),
                uv: Vec2::Y,
            },
        ]
    }

    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        &mut self,
        device: &Device,
        encoder: &mut CommandEncoder,
        staging_belt: &mut StagingBelt,
        resolution: Vec2,
        layer: usize,
        quads: &[(&Quad, Affine, Option<Rect>)],
    ) {
        if layer >= self.layers.len() {
            self.layers.resize_with(layer + 1, Layer::new);
        }

        let layer = &mut self.layers[layer];
        layer.instance_count = quads.len();

        if quads.len() > layer.instances.len() {
            let layout = &self.uniform_layout;
            (layer.instances).resize_with(quads.len(), || Instance::new(device, layout));
        }

        let screen_rect = Rect::new(Vec2::ZERO, resolution);

        for ((quad, transform, clip), instance) in quads.iter().zip(&mut layer.instances) {
            instance.clip = match clip {
                Some(clip) => clip.intersect(screen_rect),
                None => screen_rect,
            };

            instance.write_vertex_buffer(device, encoder, staging_belt, quad);
            instance.write_uniform_buffer(
                device,
                encoder,
                staging_belt,
                quad,
                *transform,
                resolution,
            );
            instance.image = quad.background_image.clone();
        }
    }

    pub fn render<'a>(
        &'a self,
        pass: &mut RenderPass<'a>,
        layer: usize,
        default_image: &'a WgpuImage,
    ) {
        let layer = &self.layers[layer];

        pass.set_pipeline(&self.pipeline);
        pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);

        for instance in &layer.instances[..layer.instance_count] {
            if instance.clip.size().min_element() < 1.0 {
                continue;
            }

            pass.set_scissor_rect(
                instance.clip.min.x as u32,
                instance.clip.min.y as u32,
                instance.clip.width() as u32,
                instance.clip.height() as u32,
            );

            let image = instance
                .image
                .as_ref()
                .and_then(|image| image.downcast_ref::<WgpuImage>());

            if let Some(image) = image {
                pass.set_bind_group(1, &image.bind_group, &[]);
            } else {
                pass.set_bind_group(1, &default_image.bind_group, &[]);
            }

            pass.set_bind_group(0, &instance.uniform_bind_group, &[]);
            pass.set_vertex_buffer(0, instance.vertex_buffer.slice(..));
            pass.draw_indexed(0..6, 0, 0..1);
        }
    }
}
