use std::collections::HashMap;

use egui_node_graph::NodeTemplateIter;
use once_cell::sync::Lazy;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Default)]
pub struct GenCode {
    pub ps_code: String,
    pub sampler_code: String,
}

// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct MyNodeData {
//     pub template: MyNodeType,
// }

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
                MyDataType::Scalar => MyValueType::Scalar { value: 0.0 },
                MyDataType::Vec3 => MyValueType::Vec3 { value: [0.0; 3] },
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
    Vec3 { value: [f32; 3] },
    Scalar { value: f32 },
}

impl Default for MyValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::Scalar { value: 0.0 }
    }
}

impl  MyValueType {
    pub fn default_scalar() -> Self {
        Self::Scalar { value: 0.0 }
    }
    pub fn default_vector() -> Self {
        Self::Vec3 { value: [0.0; 3] }
    }
}

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MyNodeType {
    MakeScalar,
    AddScalar,
    SubtractScalar,
    MakeVector,
    AddVector,
    SubtractVector,
    VectorTimesScalar,
    NormalDirection,
    LightDirection,
    DotProduct,
    Main,
    FloatToVector3,
    Clamp01Scalar,
    Clamp01Vector,
    FMAScalar,
    FMAVector,
    UV0,
    MainTexure2D,
    MatCapTexure2D,
    ToonTexure2D,
    CustomTexture2D,
    Step,
    ScreenPos,
    WorldPos,
    CameraPos,
    Depth,
    Fresenl,
    ViewDirection,
    Max,
    Min,
    Mul,
    Div,
}

pub struct AllMyNodeTypes;
impl NodeTemplateIter for AllMyNodeTypes {
    type Item = MyNodeType;

    fn all_kinds(&self) -> Vec<Self::Item> {
        MyNodeType::iter().collect()
    }
}

pub static NODE_TYPE_INFOS: Lazy<HashMap<MyNodeType, NodeTypeInfo>> = Lazy::new(|| {
    HashMap::from([
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
        (MyNodeType::AddScalar, NodeTypeInfo {
            label: "AddScalar".into(),
            categories: vec!["Scalar".into()],
            input_sockets: vec![
                InputSocketType { name: "v1".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "v2".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::SubtractScalar, NodeTypeInfo {
            label: "SubtractScalar".into(),
            categories: vec!["Scalar".into()],
            input_sockets: vec![
                InputSocketType { name: "v1".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::Scalar { value: 1.0 }) },
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
        (MyNodeType::AddVector, NodeTypeInfo {
            label: "AddVector".into(),
            categories: vec!["VectorOperations".into()],
            input_sockets: vec![
                InputSocketType { name: "v1".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "v2".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::SubtractVector, NodeTypeInfo {
            label: "SubtractVector".into(),
            categories: vec!["VectorOperations".into()],
            input_sockets: vec![
                InputSocketType { name: "v1".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::Vec3 { value: [1.0; 3] }) },
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
        (MyNodeType::NormalDirection, NodeTypeInfo {
            label: "NormalDirection".into(),
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
                InputSocketType { name: "uv".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 },
                OutputSocketType { name: "alpha".into(), ty: MyDataType::Scalar },
            ],
        }),
        (MyNodeType::LightDirection, NodeTypeInfo {
            label: "LightDirection".into(),
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
                InputSocketType { name: "alpha".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::Scalar { value: 1.0 }) },
            ],
            output_sockets: Vec::new(),
        }),
        (MyNodeType::FloatToVector3, NodeTypeInfo {
            label: "FloatToVector3".into(),
            categories: vec!["VectorOperations".into()],
            input_sockets: vec![
                InputSocketType { name: "value".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::Scalar { value: 0.0 }) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::Clamp01Scalar, NodeTypeInfo {
            label: "Clamp01Scalar".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "value".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Clamp01Vector, NodeTypeInfo {
            label: "Clamp01Vector".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "value".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::FMAScalar, NodeTypeInfo {
            label: "FMAScalar".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
                InputSocketType { name: "b".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::Scalar { value: 0.5 }) },
                InputSocketType { name: "c".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::Scalar { value: 0.5 }) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::FMAVector, NodeTypeInfo {
            label: "FMAVector".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::default_vector()) },
                InputSocketType { name: "b".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::Vec3 { value: [0.5; 3] }) },
                InputSocketType { name: "c".into(), ty: MyDataType::Vec3, default: Ok(MyValueType::Vec3 { value: [0.5; 3] }) },
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
        (MyNodeType::ScreenPos, NodeTypeInfo {
            label: "ScreenPos".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
        (MyNodeType::WorldPos, NodeTypeInfo {
            label: "WorldPos".into(),
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
        (MyNodeType::Fresenl, NodeTypeInfo {
            label: "Fresenl".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "exp".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::Scalar { value: 1.0 }) },
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
                InputSocketType { name: "b".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::Scalar { value: 1.0 }) },
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
                InputSocketType { name: "b".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::Scalar { value: 1.0 }) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::Div, NodeTypeInfo {
            label: "Div".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: vec![
                InputSocketType { name: "a".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::Scalar { value: 1.0 }) },
                InputSocketType { name: "b".into(), ty: MyDataType::Scalar, default: Ok(MyValueType::default_scalar()) },
            ],
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
            ],
        }),
        (MyNodeType::ViewDirection, NodeTypeInfo {
            label: "ViewDirection".into(),
            categories: vec!["Arithmetic".into()],
            input_sockets: Vec::new(),
            output_sockets: vec![
                OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
            ],
        }),
    ])
});