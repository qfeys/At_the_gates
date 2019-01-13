use camera::Camera;
use cgmath::{self, Array, Rad, Vector2, Vector3};
use context::Context;
use core::battlefield::Battlefield;
use core::options::Options as CoreOptions;
use core::position::Position as MapPos;
use core::unit;
use fs;
use geom;
use glutin::ElementState::Released;
use glutin::{
    self, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode, WindowEvent,
};
use mesh::MeshId;
use mesh_manager::MeshManager;
use scene::{NodeId, Scene, SceneNode};
use std::collections::HashMap;
use std::f32::consts::PI;
use types::{ScreenPos, Size2, Time, WorldPos};
use UI::button::{Button, ButtonId, ButtonManager};
use UI::gui::{is_tap, Gui};
use UI::screen::{EventStatus, Screen, ScreenCommand, ScreenType};

const FOW_FADING_TIME: f32 = 0.6;
const ZOOM_LEVEL: f32 = 0.3;

// #[derive(Clone, Debug)]
// pub struct TacticalScreen {
//     battlefield: Battlefield,
//     scene: Scene,
//     unit_meshes: MeshManager,
//     camera: Camera,
// }

pub fn TacticalScreen(context: &mut Context) -> Gui {
    Gui::new_from_buttons(ButtonManager::new(), HashMap::new())
}
/* 
impl TacticalScreen {
    pub fn new(context: &mut Context) -> TacticalScreen {
        let battlefield = Battlefield::new();
        let scene = make_scene(&battlefield);
        let (unit_meshes, _) = fs::load_all_units(context);
        let mut camera = Camera::new(context.win_size());
        camera.set_max_pos(get_max_camera_pos(battlefield.map_size));
        camera.set_pos(get_initial_camera_pos(battlefield.map_size));
        let mut screen = TacticalScreen {
            battlefield,
            scene,
            unit_meshes,
            camera,
        };
        screen
    }

    fn draw(&mut self, context: &mut Context) {
        context.clear();
        context.set_basic_color([0.0, 0.0, 0.0, 1.0]);
        self.draw_scene(context);
        //let player_info = self.player_info.get(self.core.player_id());
        //self.map_text_manager.draw(context, &player_info.camera);
    }

    fn draw_scene(&mut self, context: &mut Context) {
        self.draw_scene_nodes(context);
        /* if let Some(ref mut event_visualizer) = self.event_visualizer {
            let player_info = self.player_info.get_mut(self.core.player_id());
            event_visualizer.draw(&mut player_info.scene);
        } */
    }

    fn draw_scene_nodes(&self, context: &mut Context) {
        let m = self.camera.mat();
        for node in self.scene().nodes().values() {
            if !(node.color[3] < 1.0) {
                self.draw_scene_node(context, node, m);
            }
        }
        for layer in self.scene().transparent_node_ids().values() {
            for &node_id in layer {
                let node = self.scene().node(node_id);
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

    fn scene(&self) -> &Scene {
        &self.scene
    }

    /////////////////////////////////////

    // TODO: show commands preview
    fn try_create_context_menu_popup(&mut self, context: &mut Context, pos: MapPos) {
        /* let options = context_menu_popup::get_options(
            &self.core,
            self.current_player_info(),
            self.selected_unit_id,
            pos,
        );
        if options == context_menu_popup::Options::new() {
            return;
        }
        let mut menu_pos = context.mouse().pos;
        menu_pos.v.y = context.win_size().h - menu_pos.v.y;
        let (tx, rx) = channel();
        let screen = ContextMenuPopup::new(
            self.current_battlefield(),
            self.core.db(),
            context,
            menu_pos,
            options,
            tx,
        );
        self.context_menu_popup_rx = Some(rx);
        context.add_command(ScreenCommand::PushPopup(Box::new(screen))); */
    }

    fn print_info(&mut self, context: &Context) {
        // TODO: move this to `fn Core::get_unit_info(...) -> &str`?
        /* let pick_result = self.pick_tile(context);
        if let Some(pos) = pick_result {
            print_pos_info(self.core.db(), self.current_battlefield(), pos);
        } */
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
                self.print_info(context);
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

    fn handle_event_lmb_release(&mut self, context: &mut Context) {
        /* if self.event_visualizer.is_some() {
            return;
        }
        if !is_tap(context) {
            return;
        }
        let pick_result = self.pick_tile(context);
        if let Some(button_id) = self.gui.button_manager.get_clicked_button_id(context) {
            self.handle_event_button_press(context, button_id);
        } else if let Some(pick_result) = pick_result {
            self.try_create_context_menu_popup(context, pick_result);
        } */
    }

    fn handle_event_button_press(&mut self, context: &mut Context, button_id: ButtonId) {
        /* if button_id == self.gui.button_end_turn_id {
            self.end_turn(context);
        } else if button_id == self.gui.button_deselect_unit_id {
            self.deselect_unit(context);
        } else if button_id == self.gui.button_prev_unit_id {
            if let Some(id) = self.selected_unit_id {
                let prev_id = position::find_prev_player_unit_id(
                    self.current_battlefield(), self.core.player_id(), id);
                self.select_unit(context, prev_id);
            }
        } else if button_id == self.gui.button_next_unit_id {
            if let Some(id) = self.selected_unit_id {
                let next_id = position::find_next_player_unit_id(
                    self.current_battlefield(), self.core.player_id(), id);
                self.select_unit(context, next_id);
            }
        } else if button_id == self.gui.button_zoom_in_id {
            self.camera.change_zoom(1.0 - ZOOM_LEVEL);
        } else if button_id == self.gui.button_zoom_out_id {
            self.camera.change_zoom(1.0 + ZOOM_LEVEL);
        } */
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
}

impl Screen for TacticalScreen {
    fn tick(&mut self, context: &mut Context) {
        //self.logic(context);
        self.draw(context);
        //self.update_fo);
        //self.handle_context_menu_popup_commands(context);
    }

    fn handle_event(&mut self, context: &mut Context, event: &WindowEvent) -> EventStatus {
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
                state: Released,
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
                        state: Released,
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
}
 */
/* pub fn make_scene(battlefield: &Battlefield) -> Scene {
    let mut scene = Scene::new();

    // Place here all unchanging things in the scene
    let indiv: &unit::Indiv = battlefield.get_indiv(&unit::IndivId { id: 0 }).unwrap();
    scene.add_indiv(
        unit::IndivId { id: 0 },
        SceneNode {
            pos: geom::map_pos_to_world_pos(indiv.pos),
            rot: Rad(0.0),
            mesh_id: Some(MeshId {
                id: indiv.type_id.id,
            }),
            color: [1.0, 1.0, 1.0, 1.0],
            children: vec![],
        },
    );

    scene
} */
