#version 330 core

in vec3 position;
in vec3 normal;
uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

out vec3 colPos;
out vec3 v_normal;
out vec3 v_position;

void main() {
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = projection*modelview*vec4(position, 1.0);
    colPos = vec3(1.0,0.0,0.0);
    v_position = gl_Position.xyz / gl_Position.w;
}