#version 330 core

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out vec3 col;
out vec2 v_tex_coords;

void main() {
    gl_Position = vec4(position, 1.0);
    col = normal;
    v_tex_coords = tex_coords;
}