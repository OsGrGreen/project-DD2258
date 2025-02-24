#version 450 core

in vec3 fragment_Normal;
in vec4 fragment_Position;

uniform vec3 u_light;
uniform float u_time;

out vec4 color;

const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);



void main() {
    vec3 light = -u_light;
    vec3 v_position = vec3(fragment_Position);
    float diffuse = max(dot(normalize(fragment_Normal), normalize(light)), 0.0);

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(fragment_Normal)), 0.0), 16.0);

    color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
    //color = vec4(fragment_Normal,1.0);
}