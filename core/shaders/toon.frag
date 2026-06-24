#version 330 core

in vec3 vNormal;
in vec3 vFragPos;

out vec4 FragColor;

uniform vec3 lightPos;
uniform vec3 lightColor;

uniform vec3 baseColor;
uniform float toonSteps;

void main()
{
    vec3 norm = normalize(vNormal);
    vec3 lightDir = normalize(lightPos - vFragPos);
    float diff = max(dot(norm, lightDir), 0.0);

    float steps = max(toonSteps, 1.0);
    float quant = floor(diff * steps) / steps;

    vec3 result = baseColor * lightColor * quant;
    FragColor = vec4(result, 1.0);
}

