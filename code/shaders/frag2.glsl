#version 330 core

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;

uniform vec3 u_light;

out vec4 color;

const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);



void main() {
    if ( length(v_tex_coords-vec2(0.5,0.5)) > 0.2){
        discard;
    }
    float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(u_light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

    color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}