// This shader computes a shockwave effect

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
    time: f32,
    epicenter: vec2<f32>,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;


@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let PI = 3.1452;
    let pi_uv = PI * in.uv;
    let pi_time = PI * settings.time;

    let offset_x = sin((pi_uv.y * 1.0) + (pi_time * 0.5)) * 0.005;
    let offset_y = sin((-pi_uv.x * 1.0) + (pi_time * 0.1)) * 0.01;

    let uv_displaced = vec2<f32>(in.uv.x + offset_x, in.uv.y + offset_y);
    if uv_displaced.y > 0.0 && uv_displaced.y < 1.0 {
        return textureSample(screen_texture, texture_sampler, uv_displaced);
    } else {
        return vec4(0.0, 0.0, 0.0, 0.0);
    }
}
