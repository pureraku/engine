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
uniform float lightIntensity;

uniform float time;

void main()
{

    // FragColor = vec4(vec3(1.0), 1.0);
    vec3 color = 0.5 + 0.5 * cos(

        time * 10.0 +

        vec3(0.0, 2.0, 4.0) +

        vec3(vUV.x * 8.0, vUV.y * 8.0, (vUV.x + vUV.y) * 8.0)

    );

    color *= lightIntensity;

    FragColor = vec4(color, 1.0);
}
