#![allow(dead_code, unused_imports)]
use std::{borrow::Cow, collections::HashMap, path::PathBuf};

use eframe::egui::{self, DragValue, TextStyle};
use egui_node_graph::*;
use encoding::all::BIG5_2003;
use encoding::all::GBK;
use encoding::all::WINDOWS_31J;
use encoding::EncoderTrap;
use encoding::Encoding;
use strum::IntoEnumIterator;

use crate::hlsl::*;
use crate::types::*;

extern "system" { fn GetACP() -> u32; }

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
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct MyGraphState {
    pub active_node: Option<NodeId>,
    node_custom_data: HashMap<NodeId, String>,
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
    type NodeType = MyNodeType;
    type DataType = MyDataType;
    type ValueType = MyValueType;
    type UserState = MyGraphState;
    type CategoryType = String;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        NODE_TYPE_INFOS[self].label.clone().into()
    }

    // this is what allows the library to show collapsible lists in the node finder.
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<String> {
        NODE_TYPE_INFOS[self].categories.clone()
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn node_type(&self, _user_state: &mut Self::UserState) -> Self::NodeType {
        *self
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeType, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        for input_socket in &NODE_TYPE_INFOS[self].input_sockets {
            graph.add_input_param(
                node_id,
                input_socket.name.to_string(),
                input_socket.ty,
                input_socket.get_default_value(),
                InputParamKind::ConnectionOrConstant,
                true,
            );
        }
        for output_socket in &NODE_TYPE_INFOS[self].output_sockets {
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
    type NodeType = MyNodeType;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut MyGraphState,
        _node_data: &MyNodeType,
    ) -> Vec<MyResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        let speed = 0.01;
        let mut changed = false;
        match self {
            MyValueType::Vec3 { value } => {
                ui.label(param_name);
                if let Some(value) = value {
                    ui.horizontal(|ui| {
                        ui.label("x");
                        changed = changed || ui.add(DragValue::new(&mut value[0]).speed(speed)).changed();
                        ui.label("y");
                        changed = changed || ui.add(DragValue::new(&mut value[1]).speed(speed)).changed();
                        ui.label("z");
                        changed = changed || ui.add(DragValue::new(&mut value[2]).speed(speed)).changed();
                    });
                }
            }
            MyValueType::Scalar { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    if let Some(value) = value {
                        changed = changed || ui.add(DragValue::new(value).speed(speed)).changed();
                    }
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
impl NodeTypeTrait for MyNodeType {
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
        graph: &Graph<MyNodeType, MyDataType, MyValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<MyResponse, MyNodeType>>
    where
        MyResponse: UserResponseTrait,
    {
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        let node_type = graph[node_id].node_type;
        let node_custom_data = &mut user_state.node_custom_data;
        if node_type == MyNodeType::CustomTexture2D {
            if ui.button("Open file").clicked() {
                if let Some(f) = rfd::FileDialog::new().pick_file() {
                    node_custom_data.insert(node_id, f.to_string_lossy().to_string());
                }
            }
            node_custom_data.entry(node_id).or_default();
            ui.label(&node_custom_data[&node_id]);
        }
        else if node_type == MyNodeType::Main {
            node_custom_data.entry(node_id).or_insert(true.to_string());
            let mut draw_edge = node_custom_data[&node_id].parse().unwrap();
            if ui.checkbox(&mut draw_edge, "draw edge").changed() {
                node_custom_data.insert(node_id, draw_edge.to_string());
                responses.push(NodeResponse::User(MyResponse::ValueChanged));
            }
        }
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

type MyGraph = Graph<MyNodeType, MyDataType, MyValueType>;
type MyEditorState = GraphEditorState<MyNodeType, MyDataType, MyValueType, MyNodeType, MyGraphState>;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct NodeGraphExample {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: MyEditorState,

    user_state: MyGraphState,
    #[serde(skip)]
    core_gen_code: Option<GenCode>,

    path_buf: Option<PathBuf>,

    shader_path_buf: Option<PathBuf>,
    #[serde(skip)]
    show_gen_code: bool,
    #[serde(skip)]
    always_on_top: bool,
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

fn postorder_traversal_pixel_shader(graph: &MyGraph, node_id: NodeId, collect: &mut Vec<NodeId>) {
    for (input_name, input_id) in graph[node_id].inputs.iter() {
        if input_name == "posWS" || input_name == "nrmWS" {
            continue;
        }
        if let Some(other_output_id) = graph.connection(*input_id) {
            let next_nid = graph[other_output_id].node;
            if collect.contains(&next_nid) {
                continue;
            }
            postorder_traversal(graph, next_nid, collect);
        }
    }
    collect.push(node_id);
}

fn postorder_traversal_vertex_shader(graph: &MyGraph, node_id: NodeId, collect: &mut Vec<NodeId>) {
    for (input_name, input_id) in graph[node_id].inputs.iter() {
        if input_name == "posWS" || input_name == "nrmWS" {
            if let Some(other_output_id) = graph.connection(*input_id) {
                let next_nid = graph[other_output_id].node;
                if collect.contains(&next_nid) {
                    continue;
                }
                postorder_traversal(graph, next_nid, collect);
            }
        }
    }
    collect.push(node_id);
}

fn code_gen(graph: &MyGraph, node_id: NodeId, node_custom_data: &HashMap<NodeId, String>) -> GenCode {
    let mut samplers: HashMap<NodeId, usize> = HashMap::new();
    let sampler_code = code_gen_sampler(graph, node_id, node_custom_data, &mut samplers);
    let vs_code = code_gen_vertex_shader(graph, node_id, &samplers);
    let ps_code = code_gen_pixel_shader(graph, node_id, &samplers);
    GenCode {
        vs_code,
        ps_code,
        sampler_code,
    }
}
fn code_gen_sampler(graph: &MyGraph, node_id: NodeId, node_custom_data: &HashMap<NodeId, String>, samplers: &mut HashMap<NodeId, usize>) -> String {
    let mut topological_order = Vec::new();
    postorder_traversal(graph, node_id, &mut topological_order);
    let mut sampler_code = String::new();
    for (i, nid) in topological_order.iter().enumerate() {
        let my_node_type = graph[*nid].node_type;
        if my_node_type == MyNodeType::CustomTexture2D {
            samplers.insert(*nid, i);
            let template = r#"
                texture _{0}_tex < string ResourceName = "{1}"; >;
                sampler _{0}_sampler = sampler_state {
                    texture = <_{0}_tex>;
                };
                "#.to_owned();
            let template = template.replace("{0}", &i.to_string());
            let template = template.replace("{1}", &node_custom_data[nid].replace('\\', "\\\\"));
            sampler_code += &template;
        }
        else if my_node_type == MyNodeType::Main {
            if node_custom_data[nid].parse().unwrap() {
                sampler_code += "#define ENABLE_DRAW_EDGE_PASS\n";
            }
        }
    }
    sampler_code
}

fn code_gen_pixel_shader(graph: &MyGraph, node_id: NodeId, samplers: &HashMap<NodeId, usize>) -> String {
    let mut topological_order = Vec::new();
    postorder_traversal_pixel_shader(graph, node_id, &mut topological_order);
    let mut indexs = HashMap::new();
    let mut cg_node_names = Vec::new();
    for (i, nid) in topological_order.iter().enumerate() {
        indexs.insert(nid, i);
        let label = &graph[*nid].label;
        let cg_node_name = format!("_{}_{}", i, label);
        cg_node_names.push(cg_node_name.clone());
    }
    let mut ps_code = String::new();
    for (i, nid) in topological_order.iter().enumerate() {
        let label = &graph[*nid].label;
        let cg_node_name = &cg_node_names[i];
        let my_node_type = graph[*nid].node_type;
        let input_sockets = &NODE_TYPE_INFOS[&my_node_type].input_sockets;
        let mut params = String::new();
        let mut is_first = true;
        for (j, (input_name, input_id)) in graph[*nid].inputs.iter().enumerate() {
            if input_name == "posWS" || input_name == "nrmWS" {
                continue;
            }
            if !is_first {
                params += ", ";
            }
            if let Some(other_output_id) = graph.connection(*input_id) {
                let next_nid = graph[other_output_id].node;
                let mut output_index = usize::MAX;
                for (k, oid) in graph[next_nid].output_ids().enumerate() {
                    if other_output_id == oid {
                        output_index = k;
                    }
                }

                let index = indexs[&next_nid];
                params += &format!("{}_{}", cg_node_names[index], output_index);
            } else {
                match &input_sockets[j].default {
                    Ok(_) => {
                        match graph[*input_id].value {
                            MyValueType::Vec3 { value } => {
                                let value = value.unwrap();
                                params += &format!("float3({}, {}, {})", value[0], value[1], value[2]);
                            },
                            MyValueType::Scalar { value } => {
                                params += &value.unwrap().to_string();
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
        if my_node_type == MyNodeType::NrmWS {
            params += "vso.nrm";
        }
        else if my_node_type == MyNodeType::FaceNrmWS {
            params += "vso.posWS";
        }
        else if my_node_type == MyNodeType::UV0 {
            params += "vso.uv";
        }
        else if my_node_type == MyNodeType::ScreenPos {
            params += "vso.screenPos";
        }
        else if my_node_type == MyNodeType::PosWS {
            params += "vso.posWS"
        }
        else if my_node_type == MyNodeType::ViewDirWS {
            params += "vso.posWS"
        }
        else if my_node_type == MyNodeType::HalfDirection {
            params += "ViewDirWS(vso.posWS)"
        }
        else if my_node_type == MyNodeType::Fresnel {
            params += ", vso.posWS, vso.nrm"
        }
        else if my_node_type == MyNodeType::Depth {
            params += "vso.posWS"
        }
        else if my_node_type == MyNodeType::CustomTexture2D {
            params += &format!(", _{}_sampler", samplers[nid]);
        }
        let output_sockets = &NODE_TYPE_INFOS[&my_node_type].output_sockets;
        if output_sockets.len() > 0 {
            for k in 1..output_sockets.len() {
                if !is_first {
                    params += ", ";
                }
                params += &format!(
                    "{}_{}",
                    cg_node_name,
                    k,
                );

                let output_type = output_sockets[k].ty;
                ps_code += &format!(
                    "{} {}_{};\n",
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
                "{} {}_0 = {}({});",
                match output_type {
                    MyDataType::Scalar => "float ",
                    MyDataType::Vec3 => "float3",
                },
                cg_node_name,
                label,
                &params,
            );
            ps_code += &format!("{}\n", main_cmd);
            if i == topological_order.len() - 1 {
                match output_type {
                    MyDataType::Scalar => {
                        ps_code += &format!("return float4({}_0, {}_0, {}_0, 1.0);\n", cg_node_name, cg_node_name, cg_node_name);
                    },
                    MyDataType::Vec3 => {
                        ps_code += &format!("return float4({}_0, 1.0);\n", cg_node_name);
                    },
                }
            }
        } else {
            let main_cmd = format!(
                "return {}({});",
                label,
                &params,
            );
            ps_code += &format!("{}\n", main_cmd);
        }
    }
    ps_code
}


fn code_gen_vertex_shader(graph: &MyGraph, node_id: NodeId, samplers: &HashMap<NodeId, usize>) -> String {
    if graph[node_id].label != "Main" {
        return String::new();
    }
    let mut topological_order = Vec::new();
    postorder_traversal_vertex_shader(graph, node_id, &mut topological_order);
    let mut indexs = HashMap::new();
    let mut cg_node_names = Vec::new();
    for (i, nid) in topological_order.iter().enumerate() {
        indexs.insert(nid, i);
        let label = &graph[*nid].label;
        let cg_node_name = format!("_{}_{}", i, label);
        cg_node_names.push(cg_node_name.clone());
    }
    let mut vs_code = String::new();
    for (i, nid) in topological_order.iter().enumerate() {
        let label = &graph[*nid].label;
        let cg_node_name = &cg_node_names[i];
        let my_node_type = graph[*nid].node_type;
        let input_sockets = &NODE_TYPE_INFOS[&my_node_type].input_sockets;
        let mut params = String::new();
        let mut is_first = true;
        for (j, (input_name, input_id)) in graph[*nid].inputs.iter().enumerate() {
            if i == topological_order.len() - 1 && input_name != "posWS" && input_name != "nrmWS" {
                continue;
            }
            if !is_first {
                params += ", ";
            }
            if let Some(other_output_id) = graph.connection(*input_id) {
                let next_nid = graph[other_output_id].node;
                let mut output_index = usize::MAX;
                for (k, oid) in graph[next_nid].output_ids().enumerate() {
                    if other_output_id == oid {
                        output_index = k;
                    }
                }

                let index = indexs[&next_nid];
                params += &format!("{}_{}", cg_node_names[index], output_index);
            } else {
                match &input_sockets[j].default {
                    Ok(_) => {
                        match graph[*input_id].value {
                            MyValueType::Vec3 { value } => {
                                let value = value.unwrap();
                                params += &format!("float3({}, {}, {})", value[0], value[1], value[2]);
                            },
                            MyValueType::Scalar { value } => {
                                params += &value.unwrap().to_string();
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
        if my_node_type == MyNodeType::VSPosWS {
            params += "pos";
        }
        else if my_node_type == MyNodeType::VSUV0 {
            params += "uv";
        }
        else if my_node_type == MyNodeType::VSNrmWS {
            params += "normal";
        }
        else if my_node_type == MyNodeType::CustomTexture2D {
            params += &format!(", _{}_sampler", samplers[nid]);
        }
        let output_sockets = &NODE_TYPE_INFOS[&my_node_type].output_sockets;
        if output_sockets.len() > 0 {
            for k in 1..output_sockets.len() {
                if !is_first {
                    params += ", ";
                }
                params += &format!(
                    "{}_{}",
                    cg_node_name,
                    k,
                );

                let output_type = output_sockets[k].ty;
                vs_code += &format!(
                    "{} {}_{};\n",
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
                "{} {}_0 = {}({});",
                match output_type {
                    MyDataType::Scalar => "float ",
                    MyDataType::Vec3 => "float3",
                },
                cg_node_name,
                label,
                &params,
            );
            vs_code += &format!("{}\n", main_cmd);
        } else {
            let main_cmd = format!(
                "SetPosNrm({}, posWS, nrmWS);",
                &params,
            );
            vs_code += &format!("{}\n", main_cmd);
        }
    }
    vs_code
}

impl NodeGraphExample {
    fn save_fx_file(&self) {
        match &self.core_gen_code {
            Some(gen_code) => {
                if let Some(p) = &self.path_buf {
                    let mut fx = String::new();
                    fx += HLSL_0;
                    fx += &gen_code.sampler_code;
                    fx += HLSL_1;
                    fx += &gen_code.vs_code;
                    fx += HLSL_2;
                    fx += &gen_code.ps_code;
                    fx += HLSL_3;
                    let cp = unsafe { GetACP() };
                    if cp == 936 {
                        let content = GBK.encode(&fx.to_string(), EncoderTrap::Ignore).unwrap();
                        std::fs::write(p, content).unwrap();
                    }
                    else if cp == 950 {
                        let content = BIG5_2003.encode(&fx.to_string(), EncoderTrap::Ignore).unwrap();
                        std::fs::write(p, content).unwrap();
                    }
                    else if cp == 932 {
                        let content = WINDOWS_31J.encode(&fx.to_string(), EncoderTrap::Ignore).unwrap();
                        std::fs::write(p, content).unwrap();
                    }
                    else {
                        std::fs::write(p, fx).unwrap();
                    }
                }
            },
            None => {},
        }
    }
    fn save_graph(&self) {
        if let Some(p) = &self.shader_path_buf {
            let contents = ron::ser::to_string(self).unwrap();
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
                        let string = std::fs::read_to_string(path).unwrap();
                        *self = ron::de::from_str(&string).unwrap();
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
                ui.checkbox(&mut self.show_gen_code, "show code");
                if ui.checkbox(&mut self.always_on_top, "always on top").changed() {
                    let window_level = if self.always_on_top { egui::WindowLevel::AlwaysOnTop } else { egui::WindowLevel::Normal };
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::WindowLevel(window_level));
                }
            });
        });
        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state.draw_graph_editor(
                    ui,
                    MyNodeType::iter().collect(),
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
                        self.core_gen_code = Some(code_gen(&self.state.graph, node_id, &self.user_state.node_custom_data));
                        self.save_fx_file();
                    } else {
                        self.core_gen_code = None;
                    }
                },
                _ => {},
            };
        }
        if self.show_gen_code {
            if let Some(gen_code) = &self.core_gen_code {
                ctx.debug_painter().text(
                    egui::pos2(10.0, 35.0),
                    egui::Align2::LEFT_TOP,
                    &gen_code.ps_code,
                    TextStyle::Button.resolve(&ctx.style()),
                    egui::Color32::WHITE,
                );
                ctx.debug_painter().text(
                    egui::pos2(10.0, 200.0),
                    egui::Align2::LEFT_TOP,
                    &gen_code.sampler_code,
                    TextStyle::Button.resolve(&ctx.style()),
                    egui::Color32::WHITE,
                );
                ctx.debug_painter().text(
                    egui::pos2(10.0, 300.0),
                    egui::Align2::LEFT_TOP,
                    &gen_code.vs_code,
                    TextStyle::Button.resolve(&ctx.style()),
                    egui::Color32::WHITE,
                );
            }
        }
    }
}

