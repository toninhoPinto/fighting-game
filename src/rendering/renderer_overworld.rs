use sdl2::{pixels::Color, rect::{Point, Rect}, render::WindowCanvas};

use crate::{asset_management::asset_holders::{ItemAssets, OverworldAssets}, overworld::node::{WorldNode, WorldNodeType}, ui::ingame::{segmented_bar_ui::SegmentedBar, wrapping_list_ui::WrappingList}};

use super::renderer::{pos_world_to_screen, world_to_screen};

pub fn render_overworld(canvas: &mut WindowCanvas, 
    assets: &OverworldAssets,
    player_node_pos: usize,
    next_node: usize,
    nodes: &Vec<WorldNode>, 
    map_area: &Rect) {

    let (w, h) = canvas.output_size().unwrap();

    for node in nodes.iter() {
        for &connections in node.connect_to.iter() {
            let origin_point = pos_world_to_screen(node.position + Point::new(30,30), (w, h), None);
            let destination_point =  pos_world_to_screen(nodes[connections as usize].position + Point::new(30,30), (w, h), None);
            canvas.draw_line(origin_point, destination_point).unwrap();
        }
    }

    let rect_screen_pos = world_to_screen(
        Rect::new(0,0, map_area.width(), map_area.height()), 
        map_area.top_left(), (w, h), None);
    canvas.set_draw_color(Color::RGBA(100, 50, 50, 50));
    canvas.draw_rect(rect_screen_pos).unwrap();
    canvas.fill_rect(rect_screen_pos).unwrap();

    for i in 0..nodes.len() {
        let src_rect;

        if let WorldNodeType::Level(_) = nodes[i].node_type  {
            src_rect = assets.src_rects.get("camp").unwrap();
            canvas.set_draw_color(Color::RGB(50, 255, 100));
        } else if nodes[i].node_type == WorldNodeType::Start {
            src_rect = assets.src_rects.get("start").unwrap();
            canvas.set_draw_color(Color::RGB(255, 255, 50));
        } else if nodes[i].node_type == WorldNodeType::Store {
            src_rect = assets.src_rects.get("store").unwrap();
            canvas.set_draw_color(Color::RGB(255, 255, 50));
        } else {
            src_rect = assets.src_rects.get("boss_skull").unwrap();
            canvas.set_draw_color(Color::RGB(200, 70, 70));
        }

        let node_rect = Rect::new(0,0, 60, 60);
        let rect_screen_pos = world_to_screen(node_rect, nodes[i].position, (w, h), None);
        canvas.set_draw_color(Color::RGBA(100, 50, 50, 50));

        canvas.copy(&assets.spritesheet, src_rect.clone(), rect_screen_pos).unwrap();
    }

    let src_pointer = assets.src_rects.get("arrow").unwrap();
    let pointer_screen = world_to_screen(Rect::new(0,0, 40, 40), nodes[next_node].position + Point::new(20,0), (w, h), None);
    canvas.copy_ex(&assets.spritesheet, src_pointer.clone(), pointer_screen, 90f64, Point::new(0,0), false, false).unwrap();
    
    let src_pointer = assets.src_rects.get("symbol").unwrap();
    let pointer_screen = world_to_screen(Rect::new(0,0, 40, 40), nodes[player_node_pos as usize].position - Point::new(20,0), (w, h), None);
    canvas.copy(&assets.spritesheet, src_pointer.clone(), pointer_screen).unwrap();
    
    let rect_screen_pos = world_to_screen(Rect::new(0,0, 450, 582), Point::new(0,0), (w, h), None);
    let texture = &assets.portraits.get("portrait").unwrap();
    canvas.copy(texture, Rect::new(0,0, 900, 1165), rect_screen_pos).unwrap();
}