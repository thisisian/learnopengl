#version 330 core

out vec4 fragColor;
in vec3 ourColor;

uniform float greenValue;

void main()
{
    fragColor = vec4(0.0, greenValue, 0.0, 1.0);
}