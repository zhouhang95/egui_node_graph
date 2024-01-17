#![allow(dead_code, unused_imports)]
use std::{borrow::Cow, collections::HashMap, fmt::format, ops::Index, path::PathBuf};

use eframe::egui::{self, DragValue, TextStyle};
use egui_node_graph::*;
use strum::{IntoEnumIterator, EnumIter};

use crate::hlsl::*;

// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MyNodeData {
    template: MyNodeType,
}

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
struct InputSocketType {
    name: String,
    ty: MyDataType,
    default: Result<MyValueType, String>,
}
impl InputSocketType {
    fn get_default_value(&self) -> MyValueType {
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
struct OutputSocketType {
    name: String,
    ty: MyDataType,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct NodeTypeInfo {
    label: String,
    categories: Vec<String>,
    input_sockets: Vec<InputSocketType>,
    output_sockets: Vec<OutputSocketType>,
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
    fn default_scalar() -> Self {
        Self::Scalar { value: 0.0 }
    }
    fn default_vector() -> Self {
        Self::Vec3 { value: [0.0; 3] }
    }
}

impl MyValueType {
    /// Tries to downcast this value type to a vector
    pub fn try_to_vec3(self) -> anyhow::Result<[f32; 3]> {
        if let MyValueType::Vec3 { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to vec3", self)
        }
    }

    /// Tries to downcast this value type to a scalar
    pub fn try_to_scalar(self) -> anyhow::Result<f32> {
        if let MyValueType::Scalar { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to scalar", self)
        }
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
}

pub struct AllMyNodeTypes;
impl NodeTemplateIter for AllMyNodeTypes {
    type Item = MyNodeType;

    fn all_kinds(&self) -> Vec<Self::Item> {
        MyNodeType::iter().collect()
    }
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MyResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
    ValueChanged,
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MyGraphState {
    pub active_node: Option<NodeId>,
    node_type_infos: HashMap<MyNodeType, NodeTypeInfo>,
}

impl Default for MyGraphState {
    fn default() -> Self {
        Self {
            active_node: None,
            node_type_infos: HashMap::from([
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
            ]),
        }
    }
}

// =========== Then, you need to implement some traits ============

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<MyGraphState> for MyDataType {
    fn data_type_color(&self, _user_state: &mut MyGraphState) -> egui::ecolor::Color32 {
        match self {
            MyDataType::Scalar => egui::Color32::from_rgb(38, 109, 211),
            MyDataType::Vec3 => egui::Color32::from_rgb(238, 207, 109),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            MyDataType::Scalar => Cow::Borrowed("scalar"),
            MyDataType::Vec3 => Cow::Borrowed("3d vector"),
        }
    }
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for MyNodeType {
    type NodeData = MyNodeData;
    type DataType = MyDataType;
    type ValueType = MyValueType;
    type UserState = MyGraphState;
    type CategoryType = String;

    fn node_finder_label(&self, user_state: &mut Self::UserState) -> Cow<'_, str> {
        user_state.node_type_infos[self].label.clone().into()
    }

    // this is what allows the library to show collapsible lists in the node finder.
    fn node_finder_categories(&self, user_state: &mut Self::UserState) -> Vec<String> {
        user_state.node_type_infos[self].categories.clone()
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        MyNodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        for input_socket in &user_state.node_type_infos[self].input_sockets {
            graph.add_input_param(
                node_id,
                input_socket.name.to_string(),
                input_socket.ty,
                input_socket.get_default_value(),
                InputParamKind::ConnectionOrConstant,
                true,
            );
        }
        for output_socket in &user_state.node_type_infos[self].output_sockets {
            graph.add_output_param(
                node_id,
                output_socket.name.to_string(),
                output_socket.ty,
            );
        }
    }
}

impl WidgetValueTrait for MyValueType {
    type Response = MyResponse;
    type UserState = MyGraphState;
    type NodeData = MyNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut MyGraphState,
        _node_data: &MyNodeData,
    ) -> Vec<MyResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        let speed = 0.01;
        let mut changed = false;
        match self {
            MyValueType::Vec3 { value } => {
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label("x");
                    changed = changed || ui.add(DragValue::new(&mut value[0]).speed(speed)).changed();
                    ui.label("y");
                    changed = changed || ui.add(DragValue::new(&mut value[1]).speed(speed)).changed();
                    ui.label("z");
                    changed = changed || ui.add(DragValue::new(&mut value[2]).speed(speed)).changed();
                });
            }
            MyValueType::Scalar { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    changed = changed || ui.add(DragValue::new(value).speed(speed)).changed();
                });
            }
        }
        // This allows you to return your responses from the inline widgets.
        if changed {
            vec![MyResponse::ValueChanged]
        } else {
            Vec::new()
        }
    }
}

impl UserResponseTrait for MyResponse {}
impl NodeDataTrait for MyNodeData {
    type Response = MyResponse;
    type UserState = MyGraphState;
    type DataType = MyDataType;
    type ValueType = MyValueType;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<MyNodeData, MyDataType, MyValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<MyResponse, MyNodeData>>
    where
        MyResponse: UserResponseTrait,
    {
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
        if !is_active {
            if ui.button("Set active").clicked() {
                responses.push(NodeResponse::User(MyResponse::SetActiveNode(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(MyResponse::ClearActiveNode));
            }
        }

        responses
    }
}

type MyGraph = Graph<MyNodeData, MyDataType, MyValueType>;
type MyEditorState =
    GraphEditorState<MyNodeData, MyDataType, MyValueType, MyNodeType, MyGraphState>;

#[derive(Default)]
pub struct NodeGraphExample {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: MyEditorState,

    user_state: MyGraphState,
    core_gen_code: String,
    path_buf: Option<PathBuf>,
    shader_path_buf: Option<PathBuf>,
}

fn postorder_traversal(graph: &MyGraph, node_id: NodeId, collect: &mut Vec<NodeId>) {
    for input_id in graph[node_id].input_ids() {
        if let Some(other_output_id) = graph.connection(input_id) {
            let next_nid = graph[other_output_id].node;
            if collect.contains(&next_nid) {
                continue;
            }
            postorder_traversal(graph, next_nid, collect);
        }
    }
    collect.push(node_id);
}

fn code_gen(graph: &MyGraph, node_id: NodeId, node_type_infos: &HashMap<MyNodeType, NodeTypeInfo>) -> String {
    let mut topological_order = Vec::new();
    postorder_traversal(graph, node_id, &mut topological_order);
    let mut indexs = HashMap::new();
    let mut cg_node_names = Vec::new();
    for (i, nid) in topological_order.iter().enumerate() {
        indexs.insert(nid, i);
        let label = &graph[*nid].label;
        let cg_node_name = format!("_{}_{}", i, label);
        cg_node_names.push(cg_node_name.clone());
    }
    let mut text = String::new();
    for (i, nid) in topological_order.iter().enumerate() {
        let label = &graph[*nid].label;
        let cg_node_name = &cg_node_names[i];
        let my_node_type = graph[*nid].user_data.template;
        let input_sockets = &node_type_infos[&my_node_type].input_sockets;
        let mut params = String::new();
        let mut is_first = true;
        for (j, input_id) in graph[*nid].input_ids().enumerate() {
            if !is_first {
                params += ", ";
            }
            if let Some(other_output_id) = graph.connection(input_id) {
                let next_nid = graph[other_output_id].node;
                let mut output_index = usize::MAX;
                for (k, (_, oid)) in graph[next_nid].outputs.iter().enumerate() {
                    if other_output_id == *oid {
                        output_index = k;
                    }
                }

                let index = indexs[&next_nid];
                params += &format!("{}_o{}", cg_node_names[index], output_index);
            } else {
                match &input_sockets[j].default {
                    Ok(_) => {
                        match graph[input_id].value {
                            MyValueType::Vec3 { value } => {
                                params += &format!("float3({}, {}, {})", value[0], value[1], value[2]);
                            },
                            MyValueType::Scalar { value } => {
                                params += &value.to_string();
                            },
                        }
                    },
                    Err(def_str) => {
                        params += def_str;

                    },
                }
            }
            is_first = false;
        }
        // ad hoc
        if my_node_type == MyNodeType::NormalDirection {
            params += "vso.nrm";
        }
        else if my_node_type == MyNodeType::UV0 {
            params += "vso.uv";
        }
        let output_sockets = &node_type_infos[&my_node_type].output_sockets;
        if output_sockets.len() > 0 {
            for k in 1..output_sockets.len() {
                if !is_first {
                    params += ", ";
                }
                params += &format!(
                    "{}_o{}",
                    cg_node_name,
                    k,
                );

                let output_type = output_sockets[k].ty;
                text += &format!(
                    "{} {}_o{};\n",
                    match output_type {
                        MyDataType::Scalar => "float ",
                        MyDataType::Vec3 => "float3",
                    },
                    cg_node_name,
                    k,
                );
                is_first = false;
            }
            let output_type = output_sockets[0].ty;
            let main_cmd = format!(
                "{} {}_o0 = {}({});",
                match output_type {
                    MyDataType::Scalar => "float ",
                    MyDataType::Vec3 => "float3",
                },
                cg_node_name,
                label,
                &params,
            );
            text += &format!("{}\n", main_cmd);
            if i == topological_order.len() - 1 {
                match output_type {
                    MyDataType::Scalar => {
                        text += &format!("return float4({}_o0, {}_o0, {}_o0, 1.0);\n", cg_node_name, cg_node_name, cg_node_name);
                    },
                    MyDataType::Vec3 => {
                        text += &format!("return float4({}_o0, 1.0);\n", cg_node_name);
                    },
                }
            }
        } else {
            let main_cmd = format!(
                "return {}({});",
                label,
                &params,
            );
            text += &format!("{}\n", main_cmd);
        }
    }
    text
}

impl NodeGraphExample {
    fn save_fx_file(&self) {
        if self.core_gen_code.is_empty() {
            return;
        }
        if let Some(p) = &self.path_buf {
            let mut fx = String::new();
            fx += HLSL_0;
            fx += &self.core_gen_code;
            fx += HLSL_1;
            std::fs::write(p, fx).unwrap();
        }
    }
    fn save_graph(&self) {
        if let Some(p) = &self.shader_path_buf {
            let contents = ron::ser::to_string(&self.state).unwrap();
            std::fs::write(p, contents).unwrap();
        }
    }
}
impl eframe::App for NodeGraphExample {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Load Graph").clicked() {
                    let path_buf = rfd::FileDialog::new()
                        .add_filter("Rusty Object Notation", &["ron"])
                        .pick_file();
                    if let Some(path) = &path_buf {
                        let string = std::fs::read_to_string(&path).unwrap();
                        self.state = ron::de::from_str(&string).unwrap();
                    }
                    self.save_graph();
                }
                if ui.button("Save Graph").clicked() {
                    if self.shader_path_buf.is_none() {
                        self.shader_path_buf = rfd::FileDialog::new()
                            .add_filter("Rusty Object Notation", &["ron"])
                            .save_file();
                    }
                    self.save_graph();
                }
                if ui.button("Save Graph As ...").clicked() {
                    self.shader_path_buf = rfd::FileDialog::new()
                        .add_filter("Rusty Object Notation", &["ron"])
                        .save_file();
                    self.save_graph();
                }
                if ui.button("Save Fx").clicked() {
                    if self.path_buf.is_none() {
                        self.path_buf = rfd::FileDialog::new()
                            .add_filter("MME FX", &["fx"])
                            .save_file();
                    }
                    self.save_fx_file();
                }
                if ui.button("Save Fx As ...").clicked() {
                    self.path_buf = rfd::FileDialog::new()
                        .add_filter("MME FX", &["fx"])
                        .save_file();
                    self.save_fx_file();
                }
            });
        });
        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state.draw_graph_editor(
                    ui,
                    AllMyNodeTypes,
                    &mut self.user_state,
                    Vec::default(),
                )
            })
            .inner;
        for node_response in graph_response.node_responses {
            // Here, we ignore all other graph events. But you may find
            // some use for them. For example, by playing a sound when a new
            // connection is created
            match node_response {
                NodeResponse::User(user_event) => {
                    match user_event {
                        MyResponse::SetActiveNode(node) => {
                            self.user_state.active_node = Some(node);
                        },
                        MyResponse::ClearActiveNode => {
                            self.user_state.active_node = None;
                        },
                        MyResponse::ValueChanged => {},
                    };
                    if let Some(node_id) = self.user_state.active_node {
                        self.core_gen_code = code_gen(&self.state.graph, node_id, &self.user_state.node_type_infos);
                        self.save_fx_file();
                    } else {
                        self.core_gen_code = String::new();
                    }
                },
                _ => {},
            };
        }
        ctx.debug_painter().text(
            egui::pos2(10.0, 35.0),
            egui::Align2::LEFT_TOP,
            self.core_gen_code.clone(),
            TextStyle::Button.resolve(&ctx.style()),
            egui::Color32::WHITE,
        );
    }
}

