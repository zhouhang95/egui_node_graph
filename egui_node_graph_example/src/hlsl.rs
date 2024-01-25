pub const HLSL_0: &str = r#"
float4x4 worldMatrix : WORLD ;
float4x4 viewMatrix : VIEW ;
float4x4 viewProjMatrix : VIEWPROJECTION ;
float4x4 worldViewMatrix : WORLDVIEW ;
float4x4 worldViewProjMatrix : WORLDVIEWPROJECTION ;
float4x4 lightViewMatrix : VIEW < string Object = "Light"; > ;
float4x4 worldInvMatrix : WORLDINVERSE ;
float4x4 worldViewProjTransMatrix : WORLDVIEWPROJECTIONTRANSPOSE ;
float4 materialDiffuse  : DIFFUSE  < string Object = "Geometry"; >;
float3 materialAmbient  : AMBIENT  < string Object = "Geometry"; >;
float3 materialEmmisive : EMISSIVE < string Object = "Geometry"; >;
float3 materialSpecular : SPECULAR < string Object = "Geometry"; >;
float  specularPower    : SPECULARPOWER < string Object = "Geometry"; >;
float3 materialToon     : TOONCOLOR;
float3 edgeColor        : EDGECOLOR;
float3 lightDiffuse     : DIFFUSE   < string Object = "Light"; >;
float3 lightAmbient     : AMBIENT   < string Object = "Light"; >;
float3 lightSpecular    : SPECULAR  < string Object = "Light"; >;
float4 groundShadowColor : GROUNDSHADOWCOLOR;

static float3 cam_dir = mul(float3(0, 0, 1), transpose((float3x3)viewMatrix));

float3 light_dir: DIRECTION<string Object = "Light";>;
float3 cam_pos: POSITION<string Object = "Camera";>;

float2 screenSize : VIEWPORTPIXELSIZE;

float ftime : TIME <bool SyncInEditMode=true;>;
float elapsed_time : ELAPSEDTIME;
static float fps = 1.0 / elapsed_time;

texture mat_tex: MATERIALTEXTURE;
sampler mat_tex_sampler = sampler_state {
    texture=<mat_tex>;
    MINFILTER = LINEAR;
    MAGFILTER = LINEAR;
};

texture ObjectTexture : MATERIALTEXTURE;
sampler ObjTexSampler = sampler_state
{
    texture = <ObjectTexture>;
    MINFILTER = LINEAR;
    MAGFILTER = LINEAR;
    MIPFILTER = LINEAR;
    ADDRESSU  = WRAP;
    ADDRESSV  = WRAP;
};

texture ObjectSphereMap : MATERIALSPHEREMAP;
sampler ObjSphSampler = sampler_state
{
    texture = <ObjectSphereMap>;
    MINFILTER = LINEAR;
    MAGFILTER = LINEAR;
    MIPFILTER = LINEAR;
    ADDRESSU  = WRAP;
    ADDRESSV  = WRAP;
};

texture ObjectToonTexture : MATERIALTOONTEXTURE;
sampler ObjToonSampler = sampler_state
{
    texture = <ObjectToonTexture>;
    MINFILTER = LINEAR;
    MAGFILTER = LINEAR;
    MIPFILTER = NONE;
    ADDRESSU  = CLAMP;
    ADDRESSV  = CLAMP;
};
"#;
pub const HLSL_1: &str = r#"
struct VS_OUTPUT {
    float4 pos: POSITION;
    float3 uv: TEXCOORD1;
    float3 nrm: TEXCOORD2;
    float2 screenPos: TEXCOORD3;
    float3 posWS: TEXCOORD5;
};

float3 MakeVector(float x, float y, float z) {
    return float3(x, y, z);
}

float3 NormalDirection(float3 v) {
    return v;
}

float3 UV0(float3 v) {
    return v;
}

float3 LightDirection() {
    return -light_dir;
}

float DotProduct(float3 a, float3 b) {
    return dot(a, b);
}

float MakeScalar(float v) {
    return v;
}

float AddScalar(float a, float b) {
    return a + b;
}

float SubtractScalar(float a, float b) {
    return a - b;
}

float3 AddVector(float3 a, float3 b) {
    return a + b;
}

float3 SubtractVector(float3 a, float3 b) {
    return a - b;
}

float3 VectorTimesScalar(float3 v, float s) {
    return v * s;
}

float4 Main(float3 color, float alpha) {
    return float4(color, alpha);
}

float3 FloatToVector3(float v) {
    return float3(v, v, v);
}

float Clamp01Scalar(float v) {
    return saturate(v);
}

float3 Clamp01Vector(float3 v) {
    return saturate(v);
}

float FMAScalar(float a, float b, float c) {
    return mad(a, b, c);
}

float3 FMAVector(float a, float b, float c) {
    return mad(a, b, c);
}

float Step(float edge, float x) {
    return step(edge, x);
}

float SmoothStep(float edge0, float edge1, float x) {
    return smoothstep(edge0, edge1, x);
}

float3 ScreenPos(float2 screenPos) {
    return float3(screenPos, 0);
}

float3 WorldPos(float3 pos) {
    return pos;
}

float3 ViewDirection(float3 posWS) {
    return normalize(cam_pos - posWS);
}

float3 Fresenl(float exp, float3 posWS, float3 nrmWS) {
    float base = 1 - saturate(dot(normalize(cam_pos - posWS), nrmWS));
    return pow(base, exp);
}

float3 CameraPos() {
    return cam_pos;
}

float Depth(float3 posWS) {
    return dot((posWS - cam_pos), cam_dir);
}

float Min(float a, float b) {
    return min(a, b);
}

float Max(float a, float b) {
    return max(a, b);
}

float Mul(float a, float b) {
    return a * b;
}

float Div(float a, float b) {
    return a / b;
}

float3 MainTexure2D(float3 uv, out float alpha) {
    float4 texel = tex2D(ObjTexSampler, uv.xy);
    alpha = texel.w;
    return texel.xyz;
}

float3 MatCapTexure2D(float3 uv, out float alpha) {
    float4 texel = tex2D(ObjSphSampler, uv.xy);
    alpha = texel.w;
    return texel.xyz;
}

float3 ToonTexure2D(float3 uv, out float alpha) {
    float4 texel = tex2D(ObjToonSampler, uv.xy);
    alpha = texel.w;
    return texel.xyz;
}

float3 CustomTexture2D(float3 uv, sampler s, out float alpha) {
    float4 texel = tex2D(s, uv.xy);
    alpha = texel.w;
    return texel.xyz;
}

VS_OUTPUT Basic_VS(float4 pos: POSITION, float3 normal: NORMAL, float2 uv: TEXCOORD0) {
    VS_OUTPUT vso;
    vso.pos = mul(pos, worldViewProjMatrix);
    vso.posWS = mul(pos, worldMatrix).xyz;
    vso.screenPos = mad(vso.pos.xy/ vso.pos.w, 0.5, 0.5);
    vso.nrm = normalize(mul(normal, (float3x3)worldMatrix));
    vso.uv = float3(uv, 0);
    return vso;
}

float4 Basic_PS(VS_OUTPUT vso): COLOR0 {
"#;
pub const HLSL_2: &str = r#"
}

technique MainTec <string MMDPass = "object";> {
    pass DrawObject {
        VertexShader = compile vs_2_0 Basic_VS();
        PixelShader = compile ps_2_0 Basic_PS();
    }
}

technique MainTec_ss <string MMDPass = "object_ss";> {
    pass DrawObject {
        VertexShader = compile vs_2_0 Basic_VS();
        PixelShader = compile ps_2_0 Basic_PS();
    }
}
"#;