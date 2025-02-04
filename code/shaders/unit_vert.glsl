#version 330 core

in vec3 position;
in vec3 world_position;
in vec3 colour;
in vec3 tex_offsets;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;
uniform float time;

out vec3 col;
out vec2 v_tex_coords;

void main() {
    gl_Position = projection*view*model*vec4(position + world_position, 1.0);
    col = vec3(colour);
    float animation_step = tex_offsets.x+tex_offsets.z*time;    
    v_tex_coords = vec2(animation_step+tex_coords.x*tex_offsets.z,tex_offsets.y+tex_coords.y*tex_offsets.z);
    //v_tex_coords = vec2(tex_offsets.x+tex_coords.x*tex_offsets.z, tex_offsets.y+tex_coords.y*tex_offsets.z);
}