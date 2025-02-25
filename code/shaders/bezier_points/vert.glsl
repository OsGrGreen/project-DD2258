#version 330 core

in vec3 position;
in vec3 w_position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;
uniform float time;
uniform int selected;

out vec2 v_tex_coords;
out float v_selected;

void main() {
    vec3 right = vec3(view[0][0], view[1][0], view[2][0]);
    vec3 up = vec3(view[0][1], view[1][1], view[2][1]);

    // Calculate the billboard's vertex positions
    vec3 billboard_pos = w_position
        + (right * (position.x)/2)
        + (up * (position.y)/2);

    gl_Position = projection * view *model * vec4(billboard_pos, 1.0);

    v_tex_coords = tex_coords;

    //Maybe not the fastest
    if (gl_InstanceID == selected){
        v_selected = 1.0;
    }else{
        v_selected = 0.0;
    }
}