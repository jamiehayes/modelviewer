Texture2D albedo_map : register(t0);
SamplerState linear_wrap_sampler : register(s0);

cbuffer Constants : register(b0)
{
    row_major float4x4 worldMat;
    row_major float4x4 viewProjMat;
    float4 materialColor;
    float4 ambientColor;
    float4 sunColor;
    float3 sunDir;
}

struct VSInput
{
    float3 pos : POSITION;
    float3 normal : NORMAL;
    float2 uv : TEXCOORD0;
    float4 color : TEXCOORD1;
};

struct PSInput
{
    float4 pos : SV_Position;
    float3 normal: NORMAL;
    float2 uv: UV;
    float4 color : COLOR;
};

PSInput VSMain(VSInput input)
{
    float3 worldPos = mul(float4(input.pos, 1.0f), worldMat).xyz;

    PSInput output;
    output.pos = mul(float4(worldPos, 1.0f), viewProjMat);
    output.normal = mul(input.normal, (float3x3)worldMat);
    output.uv = input.uv;
    output.color = input.color;

    return output;
}

float4 PSMain(PSInput input) : SV_Target0
{
    float4 albedo = input.color;
    albedo.rgb *= materialColor;
    albedo *= albedo_map.Sample(linear_wrap_sampler, input.uv);

    float3 n = normalize(input.normal);
    float3 diffuse = ambientColor + sunColor.rgb * saturate(dot(n, sunDir));

    float4 color = albedo;
    color.rgb *= diffuse;

    return color;
}
