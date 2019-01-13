uniform mat4 u_ModelViewProj;
attribute vec3 a_Pos;
attribute vec2 a_Uv;
varying vec2 v_Uv;

void main() {
    v_Uv = a_Uv;
    gl_Position = u_ModelViewProj * vec4(a_Pos, 1.0);
}
