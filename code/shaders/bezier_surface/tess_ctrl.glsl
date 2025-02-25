#version 450 core

layout(vertices=16) out;

uniform float steps; //The amount of "detail"

void main() {

	gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

	gl_TessLevelOuter[0] = steps;
	gl_TessLevelOuter[1] = steps;
	gl_TessLevelOuter[2] = steps;
	gl_TessLevelOuter[3] = steps;

	gl_TessLevelInner[0] = steps;
	gl_TessLevelInner[1] = steps;

}