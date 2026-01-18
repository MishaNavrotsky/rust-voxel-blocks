
struct Globals {
    mouse_pos: vec2<f32>,
    time_passed: f32,
    frame_time: f32,
    frame: u32,
}

struct View {
    proj_view: mat4x4<f32>,
    camera_position: vec4<f32>,
}

struct VertexBuffer {
    position: vec4<f32>,
    normal: vec4<f32>,
    uv: vec2<f32>,
    _pad: array<f32, 6>,
}


@binding(0) @group(0) var<uniform> globals : Globals;
@binding(0) @group(1) var<uniform> view : View;
@binding(0) @group(2) var<storage> vertices : array<VertexBuffer>;

struct VertexInput {
  @location(0) position : vec4<f32>,
  @location(1) normal : vec4<f32>,
  @location(2) uv : vec2<f32>,
};

struct VertexOutput {
  @builtin(position) clip_position : vec4<f32>,
  @location(0) uv : vec2<f32>,
};

@vertex
fn main_vertex(input : VertexInput, @builtin(vertex_index) vid : u32) -> VertexOutput {
  var out : VertexOutput;

  let positions = array<vec2<f32>, 3>(
      vec2<f32>(-1.0, -1.0),
      vec2<f32>( 3.0, -1.0),
      vec2<f32>(-1.0,  3.0),
  );

  let uvs = array<vec2<f32>, 3>(
      vec2<f32>(0.0, 0.0),
      vec2<f32>(2.0, 0.0),
      vec2<f32>(0.0, 2.0),
  );

  out.clip_position = view.proj_view * vec4<f32>(positions[vid], 0.0, 1.0);
  out.uv = uvs[vid];

  return out;
}

@fragment
fn main_fragment(input : VertexOutput) -> @location(0) vec4<f32> {
  return vec4<f32>(1.0, input.uv.x * sin(globals.time_passed), input.uv.y * cos(globals.time_passed), 1.0);
}