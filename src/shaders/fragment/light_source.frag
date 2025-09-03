#version 330 core
out vec4 FragColor;

uniform vec3 lightColorSource;

void main()
{
    FragColor = vec4(lightColorSource, 1.0);
}