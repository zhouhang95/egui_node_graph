#![allow(dead_code, unused_imports)]
use std::{borrow::Cow, collections::HashMap, fmt::format};

use eframe::egui::{self, DragValue, TextStyle};
use egui_node_graph::*;

// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct MyNodeData {
    template: MyNodeType,
}

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum MyDataType {
    Scalar,
    Vec3,
}

struct InputSocketType {
    name: String,
    ty: MyDataType,
    default: Option<MyValueType>,
}
impl InputSocketType {
    fn get_default_value(&self) -> MyValueType {
        if let Some(def) = self.default {
            def
        } else {
            match self.ty {
                MyDataType::Scalar => MyValueType::Scalar { value: 0.0 },
                MyDataType::Vec3 => MyValueType::Vec3 { value: [0.0; 3] },
            }
        }
    }
}
struct OutputSocketType {
    name: String,
    ty: MyDataType,
}
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
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
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
    Saturate,
    Saturate3,
}

pub struct AllMyNodeTypes;
impl NodeTemplateIter for AllMyNodeTypes {
    type Item = MyNodeType;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user. Crates like strum can reduce the
        // boilerplate in enumerating all variants of an enum.
        vec![
            MyNodeType::MakeScalar,
            MyNodeType::MakeVector,
            MyNodeType::AddScalar,
            MyNodeType::SubtractScalar,
            MyNodeType::AddVector,
            MyNodeType::SubtractVector,
            MyNodeType::VectorTimesScalar,
            MyNodeType::NormalDirection,
            MyNodeType::LightDirection,
            MyNodeType::DotProduct,
            MyNodeType::Main,
            MyNodeType::FloatToVector3,
            MyNodeType::Saturate,
            MyNodeType::Saturate3,
        ]
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
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
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
                        InputSocketType { name: "value".into(), ty: MyDataType::Scalar, default: None },
                    ],
                    output_sockets: vec![
                        OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
                    ],
                }),
                (MyNodeType::AddScalar, NodeTypeInfo {
                    label: "AddScalar".into(),
                    categories: vec!["Scalar".into()],
                    input_sockets: vec![
                        InputSocketType { name: "v1".into(), ty: MyDataType::Scalar, default: None },
                        InputSocketType { name: "v2".into(), ty: MyDataType::Scalar, default: None },
                    ],
                    output_sockets: vec![
                        OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
                    ],
                }),
                (MyNodeType::SubtractScalar, NodeTypeInfo {
                    label: "SubtractScalar".into(),
                    categories: vec!["Scalar".into()],
                    input_sockets: vec![
                        InputSocketType { name: "v1".into(), ty: MyDataType::Scalar, default: None },
                        InputSocketType { name: "v2".into(), ty: MyDataType::Scalar, default: None },
                    ],
                    output_sockets: vec![
                        OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
                    ],
                }),
                (MyNodeType::MakeVector, NodeTypeInfo {
                    label: "MakeVector".into(),
                    categories: vec!["VectorOperations".into()],
                    input_sockets: vec![
                        InputSocketType { name: "x".into(), ty: MyDataType::Scalar, default: None },
                        InputSocketType { name: "y".into(), ty: MyDataType::Scalar, default: None },
                        InputSocketType { name: "z".into(), ty: MyDataType::Scalar, default: None },
                    ],
                    output_sockets: vec![
                        OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
                    ],
                }),
                (MyNodeType::AddVector, NodeTypeInfo {
                    label: "AddVector".into(),
                    categories: vec!["VectorOperations".into()],
                    input_sockets: vec![
                        InputSocketType { name: "v1".into(), ty: MyDataType::Vec3, default: None },
                        InputSocketType { name: "v2".into(), ty: MyDataType::Vec3, default: None },
                    ],
                    output_sockets: vec![
                        OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
                    ],
                }),
                (MyNodeType::SubtractVector, NodeTypeInfo {
                    label: "SubtractVector".into(),
                    categories: vec!["VectorOperations".into()],
                    input_sockets: vec![
                        InputSocketType { name: "v1".into(), ty: MyDataType::Vec3, default: None },
                        InputSocketType { name: "v2".into(), ty: MyDataType::Vec3, default: None },
                    ],
                    output_sockets: vec![
                        OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
                    ],
                }),
                (MyNodeType::VectorTimesScalar, NodeTypeInfo {
                    label: "VectorTimesScalar".into(),
                    categories: vec!["VectorOperations".into()],
                    input_sockets: vec![
                        InputSocketType { name: "vector".into(), ty: MyDataType::Vec3, default: None },
                        InputSocketType { name: "scalar".into(), ty: MyDataType::Scalar, default: None },
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
                        InputSocketType { name: "v1".into(), ty: MyDataType::Vec3, default: None },
                        InputSocketType { name: "v2".into(), ty: MyDataType::Vec3, default: None },
                    ],
                    output_sockets: vec![
                        OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
                    ],
                }),
                (MyNodeType::Main, NodeTypeInfo {
                    label: "Main".into(),
                    categories: vec!["Main".into()],
                    input_sockets: vec![
                        InputSocketType { name: "color".into(), ty: MyDataType::Vec3, default: None },
                        InputSocketType { name: "alpha".into(), ty: MyDataType::Scalar, default: None },
                    ],
                    output_sockets: Vec::new(),
                }),
                (MyNodeType::FloatToVector3, NodeTypeInfo {
                    label: "FloatToVector3".into(),
                    categories: vec!["VectorOperations".into()],
                    input_sockets: vec![
                        InputSocketType { name: "value".into(), ty: MyDataType::Scalar, default: None },
                    ],
                    output_sockets: vec![
                        OutputSocketType { name: "out".into(), ty: MyDataType::Vec3 }
                    ],
                }),
                (MyNodeType::Saturate, NodeTypeInfo {
                    label: "Saturate".into(),
                    categories: vec!["Arithmetic".into()],
                    input_sockets: vec![
                        InputSocketType { name: "value".into(), ty: MyDataType::Scalar, default: None },
                    ],
                    output_sockets: vec![
                        OutputSocketType { name: "out".into(), ty: MyDataType::Scalar }
                    ],
                }),
                (MyNodeType::Saturate3, NodeTypeInfo {
                    label: "Saturate3".into(),
                    categories: vec!["Arithmetic".into()],
                    input_sockets: vec![
                        InputSocketType { name: "value".into(), ty: MyDataType::Vec3, default: None },
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
        match self {
            MyValueType::Vec3 { value } => {
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(DragValue::new(&mut value[0]));
                    ui.label("y");
                    ui.add(DragValue::new(&mut value[1]));
                    ui.label("z");
                    ui.add(DragValue::new(&mut value[2]));
                });
            }
            MyValueType::Scalar { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value));
                });
            }
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
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
}

#[cfg(feature = "persistence")]
const PERSISTENCE_KEY: &str = "egui_node_graph";

#[cfg(feature = "persistence")]
impl NodeGraphExample {
    /// If the persistence feature is enabled, Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
            .unwrap_or_default();
        Self {
            state,
            user_state: MyGraphState::default(),
        }
    }
}

impl eframe::App for NodeGraphExample {
    #[cfg(feature = "persistence")]
    /// If the persistence function is enabled,
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, &self.state);
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
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
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    MyResponse::SetActiveNode(node) => self.user_state.active_node = Some(node),
                    MyResponse::ClearActiveNode => self.user_state.active_node = None,
                }
            }
        }
        if let Some(node_id) = self.user_state.active_node {
            let mut topological_order = Vec::new();
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
            postorder_traversal(&self.state.graph, node_id, &mut topological_order);

            let mut text = String::new();
            for (i, nid) in topological_order.iter().enumerate() {
                let label = &self.state.graph[*nid].label;
                let cg_node_name = format!("_{}_{}", i, label);
                let my_node_type = self.state.graph[*nid].user_data.template;
                let output_sockets = &self.user_state.node_type_infos[&my_node_type].output_sockets;
                let params = String::new();
                if output_sockets.len() > 0 {
                    let output_type = output_sockets[0].ty;
                    let main_cmd = format!(
                        "{} {}_o0 = {}({})",
                        match output_type {
                            MyDataType::Scalar => "float ",
                            MyDataType::Vec3 => "float3",
                        },
                        cg_node_name,
                        label,
                        &params,
                    );
                    text += &format!("{}\n", main_cmd);
                }
            }

            ctx.debug_painter().text(
                egui::pos2(10.0, 35.0),
                egui::Align2::LEFT_TOP,
                text,
                TextStyle::Button.resolve(&ctx.style()),
                egui::Color32::WHITE,
            );
        }
    }
}

