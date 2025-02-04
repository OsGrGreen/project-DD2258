#version 330 core

in vec3 position;
in vec3 world_position;
in vec3 colour;
in vec4 tex_offsets;
in vec2 tex_coords;


out vec3 col;
out vec2 v_tex_coords;

void main() {
    float widthHeightRelation = tex_offsets.w / tex_offsets.z;
    gl_Position = vec4(position.x*world_position.z + world_position.x, position.y*world_position.z*widthHeightRelation + world_position.y, position.z, 1.0);
    col = vec3(colour);
    v_tex_coords = vec2(tex_offsets.x+tex_coords.x*tex_offsets.z,tex_offsets.y+tex_coords.y*tex_offsets.w);
}