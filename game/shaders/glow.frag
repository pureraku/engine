#version 330 core
out vec4 FragColor;

uniform vec3 baseColor;
uniform float lightIntensity;

void main()
{
    FragColor = vec4(baseColor * max(lightIntensity * 0.5, 1.0), 1.0);
}
