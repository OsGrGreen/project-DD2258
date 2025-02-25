#version 330 core

in vec3 v_normal;
in vec2 v_tex_coords;
in float v_color_change;

uniform sampler2D tex;
uniform vec3 u_light;

out vec4 color;

void main() {
    color = texture(tex,v_tex_coords);
    color.r += v_color_change;
    //color.g -= v_color_change/3;

    //This was needed to remove some weird edges on the grass...
    //Probably not ideal however it works
    if (color.a < 0.5) discard;
}