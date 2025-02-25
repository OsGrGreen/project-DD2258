#version 330 core

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

void main() {

    vec3 center_pos = vec3(model[0][3],model[1][3],model[2][3]);

    vec3 right = vec3(view[0][0], view[1][0], view[2][0]);
    vec3 up = vec3(view[0][1], view[1][1], view[2][1]);

    // Calculate the billboard's vertex positions
    // Make it always face camera
    vec3 billboard_pos = center_pos
        + (right * (position.x)/2)
        + (up * (position.y)/2);
    
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = projection * view *model * vec4(billboard_pos, 1.0);
    vec4 vertPos4 = modelview * vec4(position, 1.0);
    v_position = vec3(vertPos4) / vertPos4.w;
    v_tex_coords = tex_coords;
}