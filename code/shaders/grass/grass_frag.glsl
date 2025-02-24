#version 330 core

in vec3 v_normal;
in vec2 v_tex_coords;
in float v_color_change;

uniform sampler2D tex;
uniform vec3 u_light;

out vec4 color;

void main() {
    color = texture(tex,v_tex_coords);
    color.r *= v_color_change;
    color.g -= v_color_change/3;
    //float ndotl = dot(u_light, normalize(v_normal));
    //color = color*ndotl;
    if (color.a < 0.5) discard;
}