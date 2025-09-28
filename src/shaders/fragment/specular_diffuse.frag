#version 330 core
struct Material {
    sampler2D diffuse;
//    vec3 specular;
    sampler2D specular;
//    sampler2D emission;
    float shininess;
};

struct Light {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

out vec4 FragColor;

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;
in vec3 LightPos;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform Material material;
uniform Light light;
uniform float time;

void main()
{
    // ambient
    vec3 ambient = light.ambient * vec3(texture(material.diffuse, TexCoords));

    // diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(LightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, TexCoords));

    // specular
    float specularStrength = 0.5;
    vec3 viewDir = normalize(-FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
//    vec3 specular = light.specular * spec * material.specular;
    vec3 specular = light.specular * spec * vec3(texture(material.specular, TexCoords));

//    vec3 emission = texture(material.emission, TexCoords).rgb;
//    vec3 emission = vec3(0.0);
//    if (texture(material.specular, TexCoords).r == 0.0) {
//        emission = texture(material.emission, TexCoords).rgb;
//        emission = texture(material.emission, TexCoords + vec2(0.0, time)).rgb;
//        emission = emission * (sin(time) * 0.5 + 0.5) * 2.0;
//    }

    vec3 result = ambient + diffuse + specular;
    FragColor = vec4(result, 1.0);
}