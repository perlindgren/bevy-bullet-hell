#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct LevelMeterSettings {
    time: f32,
    level: f32,
    impulse: f32,
}

@group(2) @binding(0) var<uniform> settings: LevelMeterSettings;
@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
  let pi : f32 = 3.14;
  var out = textureSample(base_color_texture, base_color_sampler, mesh.uv);

  let cmp = 1.0 - settings.level
      + sin(settings.time + mesh.uv.x * 4.0 * pi) * 0.02 * settings.impulse
      + sin(1.33 * settings.time + (- mesh.uv.x) * 4.0 * 1.33 * pi) * 0.03;

  if  (mesh.uv.y > cmp)
    {
        out.r = pow(mesh.uv.y - cmp, 0.5) ;
    };
  return out;
}
