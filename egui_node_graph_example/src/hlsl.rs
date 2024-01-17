pub const HLSL_0: &'static str = r#"
float4x4 worldMatrix : WORLD ;
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


float3 light_dir: DIRECTION<string Object = "Light";>;
float3 cam_pos: POSITION<string Object = "Camera";>;

float2 screenSize : VIEWPORTPIXELSIZE;

float ftime : TIME <bool SyncInEditMode=true;>;
float elapsed_time : ELAPSEDTIME;

texture mat_tex: MATERIALTEXTURE;
sampler mat_tex_sampler = sampler_state {
    texture=<mat_tex>;
    MINFILTER = LINEAR;
    MAGFILTER = LINEAR;
};

struct VS_OUTPUT {
    float4 pos: POSITION;
    float3 uv: TEXCOORD1;
    float3 nrm: TEXCOORD2;
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

float3 MainTexure2D(float3 uv) {
    return tex2D(mat_tex_sampler, uv.xy).xyz;
}


VS_OUTPUT Basic_VS(float4 pos: POSITION, float3 normal: NORMAL, float2 uv: TEXCOORD0) {
    VS_OUTPUT vso;
    vso.pos = mul(pos, worldViewProjMatrix);
    vso.nrm = normalize(mul(normal, (float3x3)worldMatrix));
    vso.uv = float3(uv, 0);
    return vso;
}

float4 Basic_PS(VS_OUTPUT vso): COLOR0 {
"#;
pub const HLSL_1: &'static str = r#"
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