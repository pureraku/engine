#version 330 core
in vec3 vWorldPos;
in vec3 vNormal;
in vec2 vUV;

out vec4 FragColor;

uniform vec3 lightPos;
uniform vec3 lightColor;
uniform float lightIntensity;
uniform vec3 cameraPos;
uniform vec3 baseColor;
uniform float time;

void main()
{
    vec3 norm = normalize(vNormal);
    vec3 lightDir = normalize(lightPos - vWorldPos);
    vec3 viewDir = normalize(cameraPos - vWorldPos);

    // Diffuse
    float diff = max(dot(norm, lightDir), 0.0);

    // Specular
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0);

    // Grid calculation using world coordinates for sharp grid lines
    vec2 gridCoord = vWorldPos.xz * 0.5;
    vec2 gridFrac = abs(fract(gridCoord - 0.5) - 0.5) / fwidth(gridCoord);
    float gridLine = 1.0 - min(min(gridFrac.x, gridFrac.y), 1.0);

    // Sub-grid
    vec2 subGridCoord = vWorldPos.xz * 2.0;
    vec2 subGridFrac = abs(fract(subGridCoord - 0.5) - 0.5) / fwidth(subGridCoord);
    float subGridLine = 1.0 - min(min(subGridFrac.x, subGridFrac.y), 1.0);

    // Pulse effect
    float pulse = 0.5 + 0.5 * sin(time * 2.0 - length(vWorldPos.xz) * 0.2);

    // Base surface color
    vec3 darkSurface = vec3(0.08, 0.09, 0.12);
    vec3 gridColor = baseColor * (0.6 + 0.4 * pulse);

    vec3 surfaceColor = mix(darkSurface, gridColor * 0.4, subGridLine * 0.5);
    surfaceColor = mix(surfaceColor, gridColor, gridLine * 0.9);

    // Combine lighting
    float ambient = 0.25;
    vec3 litColor = surfaceColor * (ambient + diff * lightIntensity * 0.8) + spec * vec3(0.5, 0.8, 1.0) * 0.4;

    // Distance fog towards dark background
    float dist = length(cameraPos - vWorldPos);
    float fog = clamp((dist - 20.0) / 70.0, 0.0, 0.85);
    vec3 fogColor = vec3(0.05, 0.06, 0.1);

    vec3 finalColor = mix(litColor, fogColor, fog);

    FragColor = vec4(finalColor, 1.0);
}
