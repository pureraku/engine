
#version 410 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out VS_OUT {
    vec3 world_pos;
    vec3 normal;
} vs_out;

void main() {
    vec4 world_pos = model * vec4(position, 1.0);
    vs_out.world_pos = world_pos.xyz;
    vs_out.normal = normalize(mat3(model) * normal);
    gl_Position = projection * view * world_pos;
}
