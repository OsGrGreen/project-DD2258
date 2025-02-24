#version 330 core

in vec3 position;
in vec3 g_position;
in vec3 g_normal;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

uniform float u_time;
uniform float threshhold;
uniform float strength;

out vec3 v_normal;
out vec2 v_tex_coords;
out float v_color_change;

#define hashi(x)   triple32(x) 
#define hash(x)  ( float( hashi(x) ) / float( 0xffffffffU ) )


mat3 rotationMatrix(vec3 axis, float angle) {
    float c = cos(angle);
    float s = sin(angle);
    float t = 1.0 - c;
    axis = normalize(axis);
    
    return mat3(
        t * axis.x * axis.x + c,         t * axis.x * axis.y - s * axis.z,  t * axis.x * axis.z + s * axis.y,
        t * axis.x * axis.y + s * axis.z,  t * axis.y * axis.y + c,         t * axis.y * axis.z - s * axis.x,
        t * axis.x * axis.z - s * axis.y,  t * axis.y * axis.z + s * axis.x,  t * axis.z * axis.z + c
    );
}

//bias: 0.17353355999581582 ( very probably the best of its kind )
uint lowbias32(uint x)
{
    x ^= x >> 16;
    x *= 0x7feb352dU;
    x ^= x >> 15;
    x *= 0x846ca68bU;
    x ^= x >> 16;
    return x;
}

// bias: 0.020888578919738908 = minimal theoretic limit
uint triple32(uint x)
{
    x ^= x >> 17;
    x *= 0xed5ad4bbU;
    x ^= x >> 11;
    x *= 0xac4c1b51U;
    x ^= x >> 15;
    x *= 0x31848babU;
    x ^= x >> 14;
    return x;
}
void main() {

    vec3 g_n = normalize(g_normal);
    vec3 up = vec3(0.0,1.0,0.0);
    vec3 axis = cross(up, g_n);
    float dotVal = clamp(dot(up, g_n), -1.0, 1.0);
    float angle = acos(dotVal);

    mat3 rot = mat3(1.0);
    if(length(axis) > 0.001)
        rot = rotationMatrix(axis, angle);

    

    vec3 local_pos = position;
    float id_hash = hash(uint(gl_InstanceID));

    float cos_time;
    if (id_hash > threshhold){
        cos_time = cos(u_time * strength+id_hash);
    }else{
        cos_time = cos(u_time * strength);
    }

    float trig_value = ((cos_time * cos_time) * 0.65) - id_hash * 0.5;

    local_pos.x += tex_coords.y * trig_value * id_hash * 0.6;
    local_pos.z += tex_coords.y * trig_value * 0.4;
    local_pos.y *= tex_coords.y * (0.5);

    vec3 rotatedPosition = rot * (g_position);

    gl_Position = projection*view*model*vec4(local_pos+rotatedPosition, 1.0);
    v_tex_coords = tex_coords;

    v_color_change = (id_hash*(tex_coords.y))/0.5;
    //v_tex_coords = vec2(tex_offsets.x+tex_coords.x*tex_offsets.z, tex_offsets.y+tex_coords.y*tex_offsets.z);
}

