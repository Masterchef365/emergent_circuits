#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragColor;

layout(binding = 1) uniform Animation {
    float anim;
};

layout(location = 0) out vec4 outColor;

void main() {
    bool shown = anim > fragColor.x;
    vec3 color;

    if (fragColor.x < 0.5) {
        color = mix(vec3(1., 1., 0.), vec3(0., 1., 0.), fragColor.x / 0.5);
    } else {
        color = mix(vec3(0., 1., 0.), vec3(0., 1., 1.), (fragColor.x - 0.5) / 0.5);
    }

    outColor = vec4(float(shown) * color, 1.0);
}
