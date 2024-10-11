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
    epicenter : vec2<f32>,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // wave intensity
    let offset_strength = settings.intensity;

    var dist: f32 = distance(in.uv, settings.epicenter);

    let strength : f32 = 10.0;
    let well : f32 = 2.1;
    let width: f32 = 0.1;

    var texCoord : vec2<f32> = in.uv;

    if ((dist <= (settings.time + width)) && (dist >= (settings.time - width))) {
        var diff: f32 = dist - settings.time;
        var powerDiff: f32 = 1.0 - pow(abs(diff * strength), well);
        var diffTime: f32 = diff * powerDiff;
        var diffUV: vec2<f32> = normalize(in.uv - settings.epicenter);
        texCoord = in.uv + diffUV * diffTime;
    }

    return textureSample(screen_texture, texture_sampler, texCoord);
}

// https://www.shadertoy.com/view/XsXGR7
//
// void mainImage( out vec4 fragColor, in vec2 fragCoord )
// {
// 	 vec3 waveParams = vec3( 10.0, 2.1, 0.1 );
// 	 vec2 tmp = vec2( iMouse.xy / iResolution.xy );
// 	 vec2 uv = fragCoord.xy / iResolution.xy;
// 	 vec2 texCoord = uv;
// 	 float distance = distance(uv, tmp);

// 	 if ( (distance <= ((iTime ) + waveParams.z )) && ( distance >= ((iTime ) - waveParams.z)) )
// 	 {
// 		    float diff = (distance - (iTime));
// 		    float powDiff = 1.0 - pow(abs(diff*waveParams.x), waveParams.y);

// 		    float diffTime = diff  * powDiff;
// 		    vec2 diffUV = normalize(uv - tmp);
// 		    texCoord = uv + (diffUV * diffTime);

// 	 }
// 	 vec4 original = texture( iChannel0, texCoord);
// 	 fragColor = original;
// }
