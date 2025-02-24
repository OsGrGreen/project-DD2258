#version 450 core

layout(triangles) in;
layout(triangle_strip, max_vertices = 6) out;

in vec3 fragment_Normal[];
in vec4 fragment_Position[];
in vec3 w_position[];

out vec3 g_Normal;
out vec4 g_Position;
out vec3 g_wPosition;

void main() {
    // We get 4 vertices per quad from the tessellation stage
    vec4 p0 = gl_in[0].gl_Position;
    vec4 p1 = gl_in[1].gl_Position;
    vec4 p2 = gl_in[2].gl_Position;
    vec4 p3 = gl_in[3].gl_Position;

    vec3 n0 = fragment_Normal[0];
    vec3 n1 = fragment_Normal[1];
    vec3 n2 = fragment_Normal[2];
    vec3 n3 = fragment_Normal[3];

    vec3 w0 = w_position[0];
    vec3 w1 = w_position[1];
    vec3 w2 = w_position[2];
    vec3 w3 = w_position[3];

    // Emit first triangle (p0, p1, p2)
    g_Position = p0;
    g_Normal = n0;
    g_wPosition = w0;
    gl_Position = p0;
    EmitVertex();

    g_Position = p1;
    g_Normal = n1;
    g_wPosition = w1;
    gl_Position = p1;
    EmitVertex();

    g_Position = p2;
    g_Normal = n2;
    g_wPosition = w2;
    gl_Position = p2;
    EmitVertex();

    EndPrimitive();

    // Emit second triangle (p2, p3, p0)
    g_Position = p2;
    g_Normal = n2;
    g_wPosition = w2;
    gl_Position = p2;
    EmitVertex();

    g_Position = p3;
    g_Normal = n3;
    g_wPosition = w3;
    gl_Position = p3;
    EmitVertex();

    g_Position = p0;
    g_Normal = n0;
    g_wPosition = w0;
    gl_Position = p0;
    EmitVertex();

    EndPrimitive();
}
