#version 330 core

in vec3 position;
in vec3 normal;
in vec3 tex_coords;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;
uniform float u_time;


void main() {
    gl_Position = projection*view*model*vec4(position, 1.0);
}