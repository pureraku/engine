#version 330 core

in vec3 vNormal;
in vec3 vFragPos;
in vec2 vUV;

out vec4 FragColor;

uniform sampler2D tex;
uniform vec3 baseColor;
uniform bool useTexture;

uniform vec3 lightPos;
uniform vec3 lightColor;

void main()
{
    vec3 norm = normalize(vNormal);

    vec3 lightDir =
        normalize(lightPos - vFragPos);

    float diff =
        max(dot(norm, lightDir), 0.0);

    vec3 texColor = baseColor;
    if (useTexture)
        texColor = texture(tex, vUV).rgb;

    vec3 result =
        texColor *
        lightColor *
        diff;

    FragColor = vec4(result, 1.0);
}
