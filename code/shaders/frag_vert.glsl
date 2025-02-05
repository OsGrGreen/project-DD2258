#version 330 core

in vec3 colPos; 
in vec2 v_tex_coords;

uniform sampler2D tex;
uniform float u_time;
uniform float radius;

out vec4 color;

void main() {
    vec3 col = colPos;
    color = texture(tex,v_tex_coords)*vec4(col, 1.0);
}