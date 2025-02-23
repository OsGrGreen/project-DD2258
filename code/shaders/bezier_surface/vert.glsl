#version 330 core

in vec3 w_position;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;
uniform float u_time;


void main() {
    gl_Position = projection*view*model*vec4(w_position, 1.0);
}