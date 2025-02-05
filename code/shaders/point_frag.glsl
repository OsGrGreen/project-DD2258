#version 330 core

in vec2 v_tex_coords;

uniform float u_time;
uniform float radius;

out vec4 color;

// Maybe improve this to not use an if
// Is possible with smoothstep
void main() {
    if ( length(v_tex_coords-vec2(0.5,0.5)) > 0.2){
        discard;
    }
    color = vec4(0.,0.,1.,1.) ;
}
