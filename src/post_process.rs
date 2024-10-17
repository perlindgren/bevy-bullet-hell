//! Custom render pass that runs after the main pass and reads the texture generated by the main pass.
//!

use std::time::Duration;

use bevy::{
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
        RenderApp,
    },
    time::Stopwatch,
};

/// It is generally encouraged to set up post processing effects as a plugin
pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<PostProcessSettings>::default(),
            UniformComponentPlugin::<PostProcessSettings>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(Core2d, PostProcessLabel)
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::Tonemapping,
                    PostProcessLabel,
                    Node2d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<PostProcessPipeline>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessLabel;

#[derive(Default)]
struct PostProcessNode;

impl ViewNode for PostProcessNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static PostProcessSettings,
        &'static DynamicUniformIndex<PostProcessSettings>,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _post_process_settings, settings_index): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<PostProcessPipeline>();

        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let settings_uniforms = world.resource::<ComponentUniforms<PostProcessSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &post_process_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &post_process_pipeline.sampler,
                settings_binding.clone(),
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[settings_index.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

#[derive(Resource)]
struct PostProcessPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<PostProcessSettings>(true),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());
        let shader = world.load_asset("shaders/shockwave.wgsl");

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("post_process_pipeline".into()),
                    layout: vec![layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}

// shared with the shader
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PostProcessSettings {
    pub intensity: f32,
    pub time: f32,
    pub epicenter: Vec2,
}

#[derive(Resource, Default)]
pub struct PostProcResource {
    timer: Stopwatch,
}

pub fn setup(mut commands: Commands) {
    commands.init_resource::<PostProcResource>();
}

use crate::{common::*, player::PlayerResource};
// Change the intensity over time to show that the effect is controlled from the main world

pub fn update_system(
    time_r: Res<Time>,
    mut timer_r: ResMut<PostProcResource>,
    player_r: Res<PlayerResource>,
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: Query<&mut PostProcessSettings>,
    camera_q: Query<(&Camera, &GlobalTransform), Without<CustomCamera>>,
) {
    for mut setting in &mut settings {
        timer_r.timer.tick(time_r.delta());
        if keys.just_pressed(KeyCode::Space) {
            timer_r.timer.reset();
            let (camera, global_transform) = camera_q.single();

            let pos = player_r.player_pos;
            let pos_ndc = camera
                .world_to_ndc(global_transform, pos.extend(0.0))
                .unwrap();

            let epi: Vec2 = ((pos_ndc.x + 1.0) * 0.5, 1.0 - (1.0 + pos_ndc.y) * 0.5).into();
            debug!("pos {:?}, pos_ndc {:?}, epi {:?}", pos, pos_ndc, epi);

            setting.epicenter = epi;
        }

        setting.time = timer_r.timer.elapsed_secs();
    }
}

#[derive(Component)]
pub struct Shockwave {
    pub epicenter: Vec2,
    pub timer: Duration,
}

pub fn update_shockwaves(time_r: Res<Time>, mut shockwaves: Query<&mut Shockwave>) {
    let delta_t = time_r.delta();
    for mut shockwave in &mut shockwaves {
        shockwave.timer += delta_t;
        println!("timeR: {}", shockwave.timer.as_secs_f32());
    }
}
