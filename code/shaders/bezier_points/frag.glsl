#version 330 core

in vec2 v_tex_coords;
in float v_selected;

uniform float u_time;
uniform float radius;

out vec4 color;


void main() {
    if ( length(v_tex_coords-vec2(0.5,0.5)) > 0.2){
        discard;
    }
    color = vec4(1.,v_selected,0.,1.);
}
