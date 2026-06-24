#version 410 core
in VS_OUT {
vec3 world_pos;
vec3 normal;
} fs_in;

out vec4 FragColor;

uniform vec3 lightPos;
uniform vec3 lightColor;

// Hash function for pseudo-random numbers
float hash(float n) {
return fract(sin(n) * 43758.5453123);
}

float hash3(vec3 p) {
return fract(sin(dot(p, vec3(12.9898, 78.233, 45.164))) * 43758.5453);
}

// Perlin noise approximation
float noise(vec3 p) {
vec3 i = floor(p);
vec3 f = fract(p);
f = f * f * (3.0 - 2.0 * f);

float n = mix(
mix(mix(hash3(i + vec3(0, 0, 0)), hash3(i + vec3(1, 0, 0)), f.x),
mix(hash3(i + vec3(0, 1, 0)), hash3(i + vec3(1, 1, 0)), f.x), f.y),
mix(mix(hash3(i + vec3(0, 0, 1)), hash3(i + vec3(1, 0, 1)), f.x),
mix(hash3(i + vec3(0, 1, 1)), hash3(i + vec3(1, 1, 1)), f.x), f.y), f.z);

return n;
}

float fbm(vec3 p) {
float value = 0.0;
float amplitude = 1.0;
float frequency = 1.0;

for (int i = 0; i < 6; i++) {
value += amplitude * abs(noise(p * frequency) - 0.5);
frequency *= 2.0;
amplitude *= 0.5;
}
return value;
}

void main() {
vec3 pos = fs_in.world_pos;
vec3 normal = normalize(fs_in.normal);

// Generate fractal brownian motion pattern
float pattern = fbm(pos * 3.0);
float detail = fbm(pos * 10.0 + pattern);
float fine = fbm(pos * 30.0 + detail);

// Create complex color mapping
vec3 color1 = vec3(0.1, 0.3, 0.8);  // Deep blue
vec3 color2 = vec3(0.9, 0.4, 0.1);  // Orange
vec3 color3 = vec3(0.2, 0.9, 0.4);  // Green

// Mix colors based on noise patterns
vec3 base_color = mix(color1, color2, sin(pattern * 6.28) * 0.5 + 0.5);
base_color = mix(base_color, color3, sin(detail * 6.28) * 0.5 + 0.5);

// Add fine detail variation
base_color += fine * 0.3 * vec3(0.3, 0.5, 0.8);

// Lighting
vec3 light_dir = normalize(lightPos - pos);
float diffuse = max(dot(normal, light_dir), 0.3);

// Rim lighting for extra pop
vec3 view_dir = normalize(-pos);
float rim = pow(1.0 - abs(dot(normal, view_dir)), 2.0);

// Final composition
vec3 color = base_color * diffuse + rim * 0.5 * vec3(1.0, 0.8, 1.0);

FragColor = vec4(color, 1.0);
}
