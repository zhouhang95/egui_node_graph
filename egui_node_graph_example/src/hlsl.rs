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
float4 edgeColor        : EDGECOLOR;
float3 lightDiffuse     : DIFFUSE   < string Object = "Light"; >;
float3 lightAmbient     : AMBIENT   < string Object = "Light"; >;
float3 lightSpecular    : SPECULAR  < string Object = "Light"; >;
float4 groundShadowColor : GROUNDSHADOWCOLOR;

static float3 cam_dir = mul(float3(0, 0, 1), transpose((float3x3)viewMatrix));

float3 light_dir: DIRECTION<string Object = "Light";>;
float3 cam_pos: POSITION<string Object = "Camera";>;

float2 screenSize : VIEWPORTPIXELSIZE;

float ftime_sync : TIME <bool SyncInEditMode=true;>;
float ftime_free : TIME <bool SyncInEditMode=false;>;
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
    float4 uv1: TEXCOORD4;
    float3 posWS: TEXCOORD5;
};

float3 MakeVector(float x, float y, float z) {
    return float3(x, y, z);
}

float TimeSync() {
    return ftime_sync;
}

float TimeFree() {
    return ftime_free;
}

float3 NrmWS(float3 v) {
    return v;
}

float3 NrmVS(float3 v) {
    return mul(v, (float3x3)viewMatrix);
}

float3 FaceNrmWS(float3 posWS) {
    return normalize(cross(ddx(posWS), ddy(posWS)));
}

float3 UV0(float3 v) {
    return v;
}

float3 UV1(float4 v, out float w) {
    w = v.w;
    return v.xyz;
}

float3 LightDirWS() {
    return -light_dir;
}

float DotProduct(float3 a, float3 b) {
    return dot(a, b);
}

float MakeScalar(float v) {
    return v;
}

float Pow(float x, float y) {
    return pow(x, y);
}

float3 Pow3(float3 x, float y) {
    return pow(x, y);
}

float Sqrt(float x) {
    return sqrt(x);
}

float Add(float a, float b) {
    return a + b;
}

float Sub(float a, float b) {
    return a - b;
}

float3 Add3(float3 a, float3 b) {
    return a + b;
}

float3 Sub3(float3 a, float3 b) {
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

float Saturate(float v) {
    return saturate(v);
}

float3 Saturate3(float3 v) {
    return saturate(v);
}

float FMA(float a, float b, float c) {
    return mad(a, b, c);
}

float3 FMA3(float3 a, float3 b, float3 c) {
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

float3 PosWS(float3 pos) {
    return pos;
}

float3 ViewDirWS(float3 posWS) {
    return normalize(cam_pos - posWS);
}

float3 Fresnel(float exp, float3 posWS, float3 nrmWS) {
    float base = 1 - saturate(dot(normalize(cam_pos - posWS), nrmWS));
    return pow(base, exp);
}

float3 CameraPos() {
    return cam_pos;
}

float Depth(float3 posWS) {
    return dot((posWS - cam_pos), cam_dir);
}

float3 Normalize(float3 nrm) {
    return normalize(nrm);
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

float3 Mul3(float3 a, float3 b) {
    return a * b;
}

float Div(float a, float b) {
    return a / b;
}

float Sin(float v) {
    return sin(v);
}

float Cos(float v) {
    return cos(v);
}

float Route(float v) {
    return v;
}

float3 Route3(float3 v) {
    return v;
}

float ControlObject(float v) {
    return v;
}

float3 ControlObject3(float3 v) {
    return v;
}

float Lerp(float a, float b, float t) {
    return lerp(a, b, t);
}

float3 Lerp3(float3 a, float3 b, float t) {
    return lerp(a, b, t);
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

float3 CustomTexture2D(float3 uv, sampler s, out float r, out float g, out float b, out float alpha) {
    float4 texel = tex2D(s, uv.xy);
    alpha = texel.w;
    r = texel.x;
    g = texel.y;
    b = texel.z;
    return texel.xyz;
}

float3 Hue(float v) {
    return saturate(3.0*abs(1.0-2.0*frac(v+float3(0.0,-1.0/3.0,1.0/3.0)))-1);
}

float3 HsvToRgb(float h, float s, float v) {
    return lerp(float3(1,1,1), Hue(h), s) * v;
}

float3 RgbToHsv(float3 rgb, out float h, out float s, out float v) {
    float4 node_1363_k = float4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    float4 node_1363_p = lerp(float4(float4(rgb,0.0).zy, node_1363_k.wz), float4(float4(rgb,0.0).yz, node_1363_k.xy), step(float4(rgb,0.0).z, float4(rgb,0.0).y));
    float4 node_1363_q = lerp(float4(node_1363_p.xyw, float4(rgb,0.0).x), float4(float4(rgb,0.0).x, node_1363_p.yzx), step(node_1363_p.x, float4(rgb,0.0).x));
    float node_1363_d = node_1363_q.x - min(node_1363_q.w, node_1363_q.y);
    float node_1363_e = 1.0e-10;
    float3 hsv = float3(abs(node_1363_q.z + (node_1363_q.w - node_1363_q.y) / (6.0 * node_1363_d + node_1363_e)), node_1363_d / (node_1363_q.x + node_1363_e), node_1363_q.x);
    h = hsv.r;
    s = hsv.g;
    v = hsv.b;
    return hsv;
}

float3 AdjustHsv(float3 rgb, float h_, float s_, float v_) {
    float h, s, v;
    RgbToHsv(rgb, h, s, v);
    return HsvToRgb(h + h_, s * s_, v * v_);
}

float MatAlpha() {
    return materialDiffuse.w;
}

float3 Reflect(float3 I, float3 N) {
    return reflect(I, N);
}

float3 HalfDirection(float3 v) {
    return normalize(v - light_dir);
}

float3 VertexPosWS(float4 pos) {
    return pos.xyz;
}

float3 VertexUV0(float2 uv) {
    return float3(uv, 0);
}

float3 VertexUV1(float4 uv, out float w) {
    w = uv.w;
    return uv.xyz;
}

float3 VertexNrmWS(float3 normal) {
    return normal;
}

void SetPosNrm(float3 pos, float3 nrm, out float3 vs_pos, out float3 vs_nrm) {
    vs_pos = pos;
    vs_nrm = nrm;
}

float ComponentMask(float3 vec, out float y, out float z) {
    y = vec.y;
    z = vec.z;
    return vec.x;
}

VS_OUTPUT Basic_VS(float4 pos: POSITION, float3 normal: NORMAL, float2 uv: TEXCOORD0, float4 uv1: TEXCOORD1) {
    VS_OUTPUT vso;
    float3 posWS = pos.xyz;
    float3 nrmWS = normal;
    "#;
    pub const HLSL_2: &str = r#"
    vso.pos = mul(float4(posWS, 1), worldViewProjMatrix);
    vso.posWS = posWS;
    vso.screenPos = mad(vso.pos.xy/ vso.pos.w, 0.5, 0.5);
    vso.nrm = nrmWS;
    vso.uv = float3(uv, 0);
    vso.uv1 = uv1;
    return vso;
}

float4 Basic_PS(VS_OUTPUT vso): COLOR0 {
"#;
pub const HLSL_3: &str = r#"
}

technique MainTec <string MMDPass = "object";> {
    pass DrawObject {
        VertexShader = compile vs_3_0 Basic_VS();
        PixelShader  = compile ps_3_0 Basic_PS();
    }
}

technique MainTec_ss <string MMDPass = "object_ss";> {
    pass DrawObject {
        VertexShader = compile vs_3_0 Basic_VS();
        PixelShader  = compile ps_3_0 Basic_PS();
    }
}

float4 ColorRender_VS(float4 Pos : POSITION) : POSITION {
    return mul( Pos, worldViewProjMatrix );
}

float4 ColorRender_PS() : COLOR {
    return edgeColor;
}

technique EdgeTec < string MMDPass = "edge"; > {
#ifdef ENABLE_DRAW_EDGE_PASS
    pass DrawEdge {
        AlphaBlendEnable = TRUE;
        AlphaTestEnable  = TRUE;

        VertexShader = compile vs_3_0 ColorRender_VS();
        PixelShader  = compile ps_3_0 ColorRender_PS();
    }
#endif
}
"#;