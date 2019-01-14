use camera::Camera;
use cgmath::{self, Rad, Vector2, Vector3};
use context::Context;
use core::battlefield::Battlefield;
use core::position::Position as MapPos;
use core::unit::IndivId;
use fs;
use geom;
use glutin::{
    self, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode, WindowEvent,
};
use mesh::{Mesh, MeshId};
use mesh_manager::MeshManager;
use std::cmp::{Ord, Ordering};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::f32::consts::PI;
use types::{ScreenPos, Size2, WorldPos};
use ui::screen::{EventStatus, ScreenCommand, ScreenType};

const ZOOM_LEVEL: f32 = 0.3;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NodeId {
    pub id: i32,
}

// TODO: Builder constructor
#[derive(Clone, Debug)]
pub struct SceneNode {
    pub pos: WorldPos,
    pub rot: Rad<f32>,
    pub mesh_id: Option<MeshId>,
    pub color: [f32; 4],
    pub children: Vec<SceneNode>,
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Z(f32);

impl Eq for Z {}

impl Ord for Z {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(Clone, Debug)]
pub struct Scene {
    indiv_id_to_node_id_map: HashMap<IndivId, NodeId>,
    //sector_id_to_node_id_map: HashMap<SectorId, NodeId>,    // probably not neccesary
    //object_id_to_node_id_map: HashMap<ObjectId, HashSet<NodeId>>,   // possibly not neccesary
    nodes: HashMap<NodeId, SceneNode>,
    transparent_node_ids: BTreeMap<Z, HashSet<NodeId>>,
    next_id: NodeId,
    unit_meshes: MeshManager,
    camera: Camera,
}

impl Scene {
    pub fn new(context: &mut Context, battlefield: &Battlefield) -> Scene {
        let (unit_meshes, _) = fs::load_all_units(context);
        let mut camera = Camera::new(context.win_size());
        camera.set_max_pos(get_max_camera_pos(battlefield.map_size));
        camera.set_pos(get_initial_camera_pos(battlefield.map_size));
        Scene {
            indiv_id_to_node_id_map: HashMap::new(), /* 
            sector_id_to_node_id_map: HashMap::new(),
            object_id_to_node_id_map: HashMap::new(), */
            nodes: HashMap::new(),
            transparent_node_ids: BTreeMap::new(),
            next_id: NodeId { id: 0 },
            unit_meshes,
            camera,
        }
    }

    pub fn draw(&mut self, context: &mut Context, battlefield: &Battlefield) {
        // Update all nodes of indivs
        for (indiv_id, indiv) in battlefield.get_indiv_iter() {
            if self.indiv_id_to_node_id_map.contains_key(indiv_id) {
                let node_id = self.indiv_id_to_node_id(*indiv_id);
                let node = self.node_mut(node_id);
                node.pos = indiv.pos.to_world_pos();
                node.rot = indiv.rot;
            } else {
                let node = SceneNode {
                    pos: indiv.pos.to_world_pos(),
                    rot: indiv.rot,
                    mesh_id: Some(MeshId {
                        id: indiv.type_id.id,
                    }),
                    color: [1.0, 1.0, 1.0, 1.0],
                    children: vec![],
                };
                self.add_indiv(*indiv_id, node);
            }
        }
        self.draw_statics(context, battlefield);
        self.draw_scene_nodes(context);
    }

    fn draw_statics(&self, context: &mut Context, battlefield: &Battlefield) {
        let m = self.camera.mat();

        let mesh = Mesh::plane(
            context,
            WorldPos {
                v: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            WorldPos {
                v: Vector3 {
                    x: battlefield.map_size.w as f64,
                    y: battlefield.map_size.h as f64,
                    z: 0.0,
                },
            },
        );
        context.set_mvp(m);
        context.set_basic_color([1.0, 1.0, 1.0, 1.0]);
        context.draw_mesh(&mesh);
    }

    fn draw_scene_nodes(&self, context: &mut Context) {
        let m = self.camera.mat();
        for node in self.nodes.values() {
            if !(node.color[3] < 1.0) {
                self.draw_scene_node(context, node, m);
            }
        }
        for layer in self.transparent_node_ids().values() {
            for &node_id in layer {
                let node = &self.nodes[&node_id];
                self.draw_scene_node(context, node, m);
            }
        }
    }

    fn draw_scene_node(&self, context: &mut Context, node: &SceneNode, m: cgmath::Matrix4<f32>) {
        let tr_mat = cgmath::Matrix4::from_translation(node.pos.v32());
        let rot_mat = cgmath::Matrix4::from(cgmath::Matrix3::from_angle_z(node.rot));
        let m = m * tr_mat * rot_mat;
        if let Some(mesh_id) = node.mesh_id {
            context.set_mvp(m); // TODO: use separate model matrix
            context.set_basic_color(node.color);
            context.draw_mesh(self.unit_meshes.get(mesh_id));
        }
        for node in &node.children {
            self.draw_scene_node(context, node, m);
        }
    }

    #[allow(dead_code)]
    pub fn indiv_id_to_node_id_opt(&self, indiv_id: IndivId) -> Option<NodeId> {
        self.indiv_id_to_node_id_map.get(&indiv_id).cloned()
    }

    pub fn indiv_id_to_node_id(&self, indiv_id: IndivId) -> NodeId {
        self.indiv_id_to_node_id_map[&indiv_id]
    }

    /* pub fn sector_id_to_node_id(&self, sector_id: SectorId) -> NodeId {
        self.sector_id_to_node_id_map[&sector_id]
    } */

    /* pub fn object_id_to_node_id(&self, object_id: ObjectId) -> &HashSet<NodeId> {
        &self.object_id_to_node_id_map[&object_id]
    } */

    #[allow(dead_code)]
    pub fn remove_node(&mut self, node_id: NodeId) {
        self.nodes.remove(&node_id).unwrap();
        for layer in self.transparent_node_ids.values_mut() {
            layer.remove(&node_id);
        }
    }

    pub fn add_node(&mut self, node: SceneNode) -> NodeId {
        let node_id = self.next_id;
        self.next_id.id += 1;
        assert!(!self.nodes.contains_key(&node_id));
        if node.color[3] < 1.0 {
            let z = Z(node.pos.v.z as f32);
            self.transparent_node_ids
                .entry(z)
                .or_insert_with(HashSet::new);
            let layer = self.transparent_node_ids.get_mut(&z).unwrap();
            layer.insert(node_id);
        }
        self.nodes.insert(node_id, node);
        node_id
    }

    #[allow(dead_code)]
    pub fn remove_indiv(&mut self, indiv_id: IndivId) {
        assert!(self.indiv_id_to_node_id_map.contains_key(&indiv_id));
        let node_id = self.indiv_id_to_node_id(indiv_id);
        self.remove_node(node_id);
        self.indiv_id_to_node_id_map.remove(&indiv_id).unwrap();
    }

    /* pub fn remove_object(&mut self, object_id: ObjectId) {
        assert!(self.object_id_to_node_id_map.contains_key(&object_id));
        let node_ids = self.object_id_to_node_id(object_id).clone();
        for node_id in node_ids {
            self.remove_node(node_id);
        }
        self.object_id_to_node_id_map.remove(&object_id).unwrap();
    } */

    pub fn add_indiv(&mut self, indiv_id: IndivId, node: SceneNode) -> NodeId {
        let node_id = self.add_node(node);
        assert!(!self.indiv_id_to_node_id_map.contains_key(&indiv_id));
        self.indiv_id_to_node_id_map.insert(indiv_id, node_id);
        node_id
    }

    /* pub fn add_sector(&mut self, sector_id: SectorId, node: SceneNode) -> NodeId {
        let node_id = self.add_node(node);
        assert!(!self.sector_id_to_node_id_map.contains_key(&sector_id));
        self.sector_id_to_node_id_map.insert(sector_id, node_id);
        node_id
    } */

    /* pub fn add_object(&mut self, object_id: ObjectId, node: SceneNode) -> NodeId {
        let node_id = self.add_node(node);
        self.object_id_to_node_id_map.entry(object_id).or_insert_with(HashSet::new);
        let node_ids = self.object_id_to_node_id_map.get_mut(&object_id).unwrap();
        node_ids.insert(node_id);
        node_id
    } */

    #[allow(dead_code)]
    pub fn nodes(&self) -> &HashMap<NodeId, SceneNode> {
        &self.nodes
    }

    pub fn transparent_node_ids(&self) -> &BTreeMap<Z, HashSet<NodeId>> {
        &self.transparent_node_ids
    }

    #[allow(dead_code)]
    pub fn node(&self, node_id: NodeId) -> &SceneNode {
        &self.nodes[&node_id]
    }

    pub fn node_mut(&mut self, node_id: NodeId) -> &mut SceneNode {
        self.nodes.get_mut(&node_id).expect("Bad node id")
    }

    pub fn handle_event(&mut self, context: &mut Context, event: &WindowEvent) -> EventStatus {
        match *event {
            WindowEvent::Resized(..) => {
                self.camera.regenerate_projection_mat(context.win_size());
            }
            WindowEvent::MouseMoved {
                position: (x, y), ..
            } => {
                let pos = ScreenPos {
                    v: Vector2 {
                        x: x as i32,
                        y: y as i32,
                    },
                };
                self.handle_event_mouse_move(context, pos);
            }
            WindowEvent::MouseInput {
                state: glutin::ElementState::Released,
                button: MouseButton::Left,
                ..
            } => {
                self.handle_event_lmb_release(context);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.handle_event_mouse_scroll(delta);
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: glutin::ElementState::Released,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => {
                self.handle_event_key_press(context, key);
            }
            WindowEvent::Touch(glutin::Touch {
                location: (x, y),
                phase,
                ..
            }) => {
                let pos = ScreenPos {
                    v: Vector2 {
                        x: x as i32,
                        y: y as i32,
                    },
                };
                match phase {
                    TouchPhase::Started | TouchPhase::Moved => {
                        self.handle_event_mouse_move(context, pos);
                    }
                    TouchPhase::Ended => {
                        self.handle_event_mouse_move(context, pos);
                        self.handle_event_lmb_release(context);
                    }
                    TouchPhase::Cancelled => {
                        unimplemented!();
                    }
                }
            }
            _ => {}
        }
        EventStatus::Handled
    }

    fn handle_event_mouse_move(&mut self, context: &Context, pos: ScreenPos) {
        self.handle_event_mouse_move_platform(context, pos);
    }

    fn handle_event_mouse_move_platform(&mut self, context: &Context, pos: ScreenPos) {
        if context.mouse().is_left_button_pressed {
            self.handle_camera_move(context, pos);
        } else if context.mouse().is_right_button_pressed {
            self.handle_camera_rotate(context, pos);
        }
    }

    fn handle_camera_move(&mut self, context: &Context, pos: ScreenPos) {
        let diff = pos.v - context.mouse().pos.v;
        let camera_move_speed = 2.0 * 12.0;
        let per_x_pixel = camera_move_speed / (context.win_size().w as f32);
        let per_y_pixel = camera_move_speed / (context.win_size().h as f32);
        let camera = &mut self.camera;
        camera.move_in_direction(Rad(PI), diff.x as f32 * per_x_pixel);
        camera.move_in_direction(Rad(PI * 1.5), diff.y as f32 * per_y_pixel);
    }

    fn handle_camera_rotate(&mut self, context: &Context, pos: ScreenPos) {
        let diff = pos.v - context.mouse().pos.v;
        let per_x_pixel = PI / (context.win_size().w as f32);
        // TODO: get max angles from camera
        let per_y_pixel = (PI / 4.0) / (context.win_size().h as f32);
        let camera = &mut self.camera;
        camera.add_horizontal_angle(Rad(diff.x as f32 * per_x_pixel));
        camera.add_vertical_angle(Rad(diff.y as f32 * per_y_pixel));
    }

    fn handle_event_mouse_scroll(&mut self, delta: MouseScrollDelta) {
        let delta_y;
        match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                delta_y = y;
            }
            MouseScrollDelta::PixelDelta(_, y) => {
                delta_y = y;
            }
        };
        if delta_y.abs() > 0.1 {
            let zoom = if delta_y > 0.0 {
                1.0 - ZOOM_LEVEL
            } else {
                1.0 + ZOOM_LEVEL
            };
            self.camera.change_zoom(zoom);
        }
    }

    fn handle_event_key_press(&mut self, context: &mut Context, key: VirtualKeyCode) {
        let camera_move_speed_on_keypress = 2.0; // Change to a variable/const
        let s = camera_move_speed_on_keypress;
        match key {
            VirtualKeyCode::Q | VirtualKeyCode::Escape => {
                context.add_command(ScreenCommand::ChangeScreen(ScreenType::ShuttingDown));
            }
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.camera.move_in_direction(Rad(PI * 1.5), s);
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.camera.move_in_direction(Rad(PI * 0.5), s);
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.camera.move_in_direction(Rad(PI * 0.0), s);
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.camera.move_in_direction(Rad(PI * 1.0), s);
            }
            VirtualKeyCode::I => {
                //self.print_info(context);
            }
            VirtualKeyCode::Subtract | VirtualKeyCode::Key1 => {
                self.camera.change_zoom(1.3);
            }
            VirtualKeyCode::Add | VirtualKeyCode::Key2 => {
                self.camera.change_zoom(0.7);
            }
            _ => println!("Unknown key pressed: {:?}", key),
        }
    }

    fn handle_event_lmb_release(&mut self, _context: &mut Context) {
        /*
        let pick_result = self.pick_tile(context);
        if let Some(button_id) = self.gui.button_manager.get_clicked_button_id(context) {
            self.handle_event_button_press(context, button_id);
        } else if let Some(pick_result) = pick_result {
            self.try_create_context_menu_popup(context, pick_result);
        }*/
    }
}

fn get_initial_camera_pos(map_size: Size2) -> WorldPos {
    let pos = get_max_camera_pos(map_size);
    WorldPos {
        v: Vector3 {
            x: pos.v.x / 2.0,
            y: pos.v.y / 2.0,
            z: 0.0,
        },
    }
}

fn get_max_camera_pos(map_size: Size2) -> WorldPos {
    let map_pos = MapPos::new(map_size.w as f64, (map_size.h - 1) as f64);
    let pos = geom::map_pos_to_world_pos(map_pos);
    WorldPos {
        v: Vector3 {
            x: -pos.v.x,
            y: -pos.v.y,
            z: 0.0,
        },
    }
}
