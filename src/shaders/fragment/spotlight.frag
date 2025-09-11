#version 330 core
struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Light {
    float cutOff;
    float outerCutOff;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};

out vec4 FragColor;

//in vec3 LightPos;
in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

uniform Material material;
uniform Light light;

void main()
{
    vec3 lightDir = normalize(-FragPos);
    // spotlight
    float theta = dot(lightDir, vec3(0.0, 0.0, 1.0));
    float epsilon = light.cutOff - light.outerCutOff;
    float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);
//    if (theta > light.cutOff) {
    // attenuation
    float distance = length(-FragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * pow(distance, 2));

    // ambient
    vec3 ambient = light.ambient * vec3(texture(material.diffuse, TexCoords));

    // diffuse
    vec3 norm = normalize(Normal);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, TexCoords));

    // specular
    vec3 viewDir = normalize(-FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * vec3(texture(material.specular, TexCoords));

    diffuse *= intensity;
    specular *= intensity;

    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;

    vec3 result = ambient + diffuse + specular;
    FragColor = vec4(result, 1.0);
//    } else {
//        FragColor = vec4(light.ambient * vec3(texture(material.diffuse, TexCoords)), 1.0);
//    }
}