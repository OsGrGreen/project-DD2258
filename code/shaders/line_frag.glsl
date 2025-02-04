#version 330 core


in vec3 col;
in vec2 v_tex_coords;

uniform sampler2D tex;

out vec4 color;

void main() {
    color = texture(tex,v_tex_coords)*vec4(col,1.0);
}