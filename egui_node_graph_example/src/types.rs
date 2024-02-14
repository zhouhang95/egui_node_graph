use std::collections::HashMap;

use once_cell::sync::Lazy;
use strum::EnumIter;

#[derive(Debug, Clone, Default)]
pub struct GenCode {
    pub vs_code: String,
    pub ps_code: String,
    pub sampler_code: String,
}

// ========= First, define your user data types =============

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MyDataType {
    Scalar,
    Vec3,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputSocketType {
    pub name: String,
    pub ty: MyDataType,
    pub default: Result<MyValueType, String>,
}
impl InputSocketType {
    pub fn get_default_value(&self) -> MyValueType {
        if let Ok(def) = self.default {
            def
        } else {
            match self.ty {
                MyDataType::Scalar => MyValueType::Scalar { value: None },
                MyDataType::Vec3 => MyValueType::Vec3 { value: None },
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutputSocketType {
    pub name: String,
    pub ty: MyDataType,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NodeTypeInfo {
    pub label: String,
    pub categories: Vec<String>,
    pub input_sockets: Vec<InputSocketType>,
    pub output_sockets: Vec<OutputSocketType>,
}

/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Scalar and a ValueType of Vec2.
#[derive(Copy, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MyValueType {
    Vec3 { value: Option<[f32; 3]> },
    Scalar { value: Option<f32> },
}

impl Default for MyValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::Scalar { value: Some(0.0) }
    }
}

impl  MyValueType {
    pub fn scalar(value: f32) -> Self {
        Self::Scalar { value: Some(value) }
    }
    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self::Vec3 { value: Some([x, y, z]) }
    }
    pub fn default_scalar() -> Self {
        Self::Scalar { value: Some(0.0) }
    }
    pub fn default_vector() -> Self {
        Self::Vec3 { value: Some([0.0; 3]) }
    }
}

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MyNodeType {
    MakeScalar,
    Add,
    Sub,
    MakeVector,
    Add3,
    Sub3,
    VectorTimesScalar,
    NrmWS,
    NrmVS,
    FaceNrmWS,
    LightDirWS,
    DotProduct,
    Main,
    FloatToVector3,
    Saturate,
    Saturate3,
    FMA,
    FMA3,
    Pow,
    Pow3,
    Sqrt,
    UV0,
    MainTexure2D,
    MatCapTexure2D,
    ToonTexure2D,
    CustomTexture2D,
    Step,
    SmoothStep,
    ScreenPos,
    PosWS,
    CameraPos,
    Depth,
    Fresnel,
    ViewDirWS,
    Max,
    Min,
    Mul,
    Mul3,
    Div,
    Sin,
    Cos,
    Lerp,
    Lerp3,
    Normalize,
    MatAlpha,
    Reflect,
    HalfDirection,
    ComponentMask,
    VSPosWS,
    VSUV0,
    VSNrmWS,
    TimeSync,
    TimeFree,
    Route,
    Route3,
    RgbToHsv,
    HsvToRgb,
    AdjustHsv,
}

pub static NODE_TYPE_INFOS: Lazy<HashMap<MyNodeType, NodeTypeInfo>> = Lazy::new(|| {
    HashMap::from([
        (MyNodeType::TimeSync, NodeTypeInfo {
            label: "TimeSync".into(),
            categories: vec!["Scalar".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::TimeFree, NodeTypeInfo {
            label: "TimeFree".into(),
            categories: vec!["Scalar".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::MakeScalar, NodeTypeInfo {
            label: "MakeScalar".into(),
            categories: vec!["Scalar".into()],
            input_sockets: vec![
                InputSocketType { name: "value".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Add, NodeTypeInfo {
            label: "Add".into(),
            categories: vec!["Scalar".into()],
            input_sockets: vec![
                InputSocketType { name: "v1".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "v2".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Pow, NodeTypeInfo {
            label: "Pow".into(),
            categories: vec!["Scalar".into()],
            input_sockets: vec![
                InputSocketType { name: "x".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "y".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0)) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Pow3, NodeTypeInfo {
            label: "Pow3".into(),
            categories: vec!["Scalar".into()],
            input_sockets: vec![
                InputSocketType { name: "x".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "y".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0)) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::Sqrt, NodeTypeInfo {
            label: "Sqrt".into(),
            categories: vec!["Scalar".into()],
            input_sockets: vec![
                InputSocketType { name: "x".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Sub, NodeTypeInfo {
            label: "Sub".into(),
            categories: vec!["Scalar".into()],
            input_sockets: vec![
                InputSocketType { name: "v1".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0)) },
                InputSocketType { name: "v2".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::MakeVector, NodeTypeInfo {
            label: "MakeVector".into(),
            categories: vec!["VectorOperations".into()],
            input_sockets: vec![
                InputSocketType { name: "x".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "y".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "z".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::Add3, NodeTypeInfo {
            label: "Add3".into(),
            categories: vec!["VectorOperations".into()],
            input_sockets: vec![
                InputSocketType { name: "v1".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "v2".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::Sub3, NodeTypeInfo {
            label: "Sub3".into(),
            categories: vec!["VectorOperations".into()],
            input_sockets: vec![
                InputSocketType { name: "v1".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::vector(1.0, 1.0, 1.0)) },
                InputSocketType { name: "v2".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::VectorTimesScalar, NodeTypeInfo {
            label: "VectorTimesScalar".into(),
            categories: vec!["VectorOperations".into()],
            input_sockets: vec![
                InputSocketType { name: "vector".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "scalar".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::NrmWS, NodeTypeInfo {
            label: "NrmWS".into(),
            categories: vec!["GeometryData".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::NrmVS, NodeTypeInfo {
            label: "NrmVS".into(),
            categories: vec!["GeometryData".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::FaceNrmWS, NodeTypeInfo {
            label: "FaceNrmWS".into(),
            categories: vec!["GeometryData".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::UV0, NodeTypeInfo {
            label: "UV0".into(),
            categories: vec!["GeometryData".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::MainTexure2D, NodeTypeInfo {
            label: "MainTexure2D".into(),
            categories: vec!["Main".into()],
            input_sockets: vec![
                InputSocketType { name: "uv".into(), ty: MyDataType::Vec3, default: Err("vso.uv".to_string()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 },
                OutputSocketType { name: "alpha".into(), ty: MyDataType::Scalar },
            ],
        }),
        (MyNodeType::MatCapTexure2D, NodeTypeInfo {
            label: "MatCapTexure2D".into(),
            categories: vec!["Main".into()],
            input_sockets: vec![
                InputSocketType { name: "uv".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 },
                OutputSocketType { name: "alpha".into(), ty: MyDataType::Scalar },
            ],
        }),
        (MyNodeType::ToonTexure2D, NodeTypeInfo {
            label: "ToonTexure2D".into(),
            categories: vec!["Main".into()],
            input_sockets: vec![
                InputSocketType { name: "uv".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 },
                OutputSocketType { name: "alpha".into(), ty: MyDataType::Scalar },
            ],
        }),
        (MyNodeType::CustomTexture2D, NodeTypeInfo {
            label: "CustomTexture2D".into(),
            categories: vec!["Main".into()],
            input_sockets: vec![
                InputSocketType { name: "uv".into(), ty: MyDataType::Vec3, default: Err("vso.uv".to_string()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 },
                OutputSocketType { name: "r".into(), ty: MyDataType::Scalar },
                OutputSocketType { name: "g".into(), ty: MyDataType::Scalar },
                OutputSocketType { name: "b".into(), ty: MyDataType::Scalar },
                OutputSocketType { name: "alpha".into(), ty: MyDataType::Scalar },
            ],
        }),
        (MyNodeType::RgbToHsv, NodeTypeInfo {
            label: "RgbToHsv".into(),
            categories: vec!["Utility".into()],
            input_sockets: vec![
                InputSocketType { name: "rgb".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 },
                OutputSocketType { name: "h".into(), ty: MyDataType::Scalar },
                OutputSocketType { name: "s".into(), ty: MyDataType::Scalar },
                OutputSocketType { name: "v".into(), ty: MyDataType::Scalar },
            ],
        }),
        (MyNodeType::HsvToRgb, NodeTypeInfo {
            label: "HsvToRgb".into(),
            categories: vec!["Utility".into()],
            input_sockets: vec![
                InputSocketType { name: "h".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "s".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "v".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 },
            ],
        }),
        (MyNodeType::AdjustHsv, NodeTypeInfo {
            label: "AdjustHsv".into(),
            categories: vec!["Utility".into()],
            input_sockets: vec![
                InputSocketType { name: "rgb".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "h".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "s".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0)) },
                InputSocketType { name: "v".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0)) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 },
            ],
        }),
        (MyNodeType::LightDirWS, NodeTypeInfo {
            label: "LightDirWS".into(),
            categories: vec!["Lighting".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::DotProduct, NodeTypeInfo {
            label: "DotProduct".into(),
            categories: vec!["VectorOperations".into()],
            input_sockets: vec![
                InputSocketType { name: "v1".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "v2".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Main, NodeTypeInfo {
            label: "Main".into(),
            categories: vec!["Main".into()],
            input_sockets: vec![
                InputSocketType { name: "color".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "alpha".into(), ty: MyDataType::Scalar, default: Err("MatAlpha()".to_string()) },
                InputSocketType { name: "posWS".into(), ty: MyDataType::Vec3, default: Err("pos.xyz".to_string()) },
                InputSocketType { name: "nrmWS".into(), ty: MyDataType::Vec3, default: Err("normal".to_string()) },
            ],
            output_sockets: Vec::new(),
        }),
        (MyNodeType::FloatToVector3, NodeTypeInfo {
            label: "FloatToVector3".into(),
            categories: vec!["VectorOperations".into()],
            input_sockets: vec![
                InputSocketType { name: "value".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::Saturate, NodeTypeInfo {
            label: "Saturate".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "value".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Saturate3, NodeTypeInfo {
            label: "Saturate3".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "value".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::FMA, NodeTypeInfo {
            label: "FMA".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "b".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(0.5)) },
                InputSocketType { name: "c".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(0.5)) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::FMA3, NodeTypeInfo {
            label: "FMA3".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "b".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::vector(0.5, 0.5, 0.5)) },
                InputSocketType { name: "c".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::vector(0.5, 0.5, 0.5)) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::Step, NodeTypeInfo {
            label: "Step".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "edge".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "x".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::SmoothStep, NodeTypeInfo {
            label: "SmoothStep".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "min".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "max".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0) ) },
                InputSocketType { name: "x".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Lerp, NodeTypeInfo {
            label: "Lerp".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "b".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0) ) },
                InputSocketType { name: "t".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Lerp3, NodeTypeInfo {
            label: "Lerp3".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "b".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::vector(1.0, 1.0, 1.0)) },
                InputSocketType { name: "t".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::ScreenPos, NodeTypeInfo {
            label: "ScreenPos".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::PosWS, NodeTypeInfo {
            label: "PosWS".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::CameraPos, NodeTypeInfo {
            label: "CameraPos".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::Depth, NodeTypeInfo {
            label: "Depth".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::MatAlpha, NodeTypeInfo {
            label: "MatAlpha".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Normalize, NodeTypeInfo {
            label: "Normalize".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "nrm".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::Fresnel, NodeTypeInfo {
            label: "Fresnel".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "exp".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0)) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Max, NodeTypeInfo {
            label: "Max".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "b".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Min, NodeTypeInfo {
            label: "Min".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "b".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0)) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Mul, NodeTypeInfo {
            label: "Mul".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "b".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0)) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Mul3, NodeTypeInfo {
            label: "Mul3".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "b".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::vector(1.0, 1.0, 1.0)) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::Div, NodeTypeInfo {
            label: "Div".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::scalar(1.0)) },
                InputSocketType { name: "b".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Sin, NodeTypeInfo {
            label: "Sin".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "v".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Cos, NodeTypeInfo {
            label: "Cos".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "v".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Reflect, NodeTypeInfo {
            label: "Reflect".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "I".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "N".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::ComponentMask, NodeTypeInfo {
            label: "ComponentMask".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "vec".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "x".into(), ty: MyDataType::Scalar },
                OutputSocketType { name: "y".into(), ty: MyDataType::Scalar },
                OutputSocketType { name: "z".into(), ty: MyDataType::Scalar },
            ],
        }),
        (MyNodeType::HalfDirection, NodeTypeInfo {
            label: "HalfDirection".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::ViewDirWS, NodeTypeInfo {
            label: "ViewDirWS".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::VSPosWS, NodeTypeInfo {
            label: "VSPosWS".into(),
            categories: vec!["VertexShader".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::VSUV0, NodeTypeInfo {
            label: "VSUV0".into(),
            categories: vec!["VertexShader".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::VSNrmWS, NodeTypeInfo {
            label: "VSNrmWS".into(),
            categories: vec!["VertexShader".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::Route, NodeTypeInfo {
            label: "Route".into(),
            categories: vec!["Utility".into()],
            input_sockets: vec![
                InputSocketType { name: "v".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Route3, NodeTypeInfo {
            label: "Route3".into(),
            categories: vec!["Utility".into()],
            input_sockets: vec![
                InputSocketType { name: "v".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
    ])
});