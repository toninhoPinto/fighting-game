use std::collections::HashSet;

use sdl2::rect::{Point, Rect};
use delaunator::{Point as DelPoint, triangulate};

use crate::scenes::overworld_scene::OverworldScene;

use self::node::{WorldNode, WorldNodeType};

pub mod node;

use rand::Rng;
use rand::rngs::SmallRng;

pub fn overworld_change_connections(overworld: &mut OverworldScene, rng: &mut SmallRng, full_conection: bool) {
    let points = &overworld.nodes.iter().map(|n| {DelPoint{x: n.position.x as f64, y: n.position.y as f64}}).collect::<Vec<DelPoint>>();

    let delaunay = triangulate(points);
    let mut graph = points.iter().map(|_| {HashSet::new()}).collect::<Vec<HashSet<usize>>>();

    if let Some(delaunay) = delaunay.as_ref() {
        for i in (0..delaunay.triangles.len()).step_by(3) {
                let point1 = points[delaunay.triangles[i]].y;
                let point2 = points[delaunay.triangles[i+1]].y;
                let point3 = points[delaunay.triangles[i+2]].y;

                let mut sorted_points = vec![(i, point1),(i+1, point2),(i+2, point3)];
                sorted_points.sort_by(|a, b| (a.1 as i32).cmp(&(b.1 as i32)));

                let triangle_bottom_to_top = sorted_points.iter().map(|p| {p.0}).collect::<Vec<usize>>();

                graph[delaunay.triangles[triangle_bottom_to_top[0]]].insert(delaunay.triangles[triangle_bottom_to_top[1]]);
                graph[delaunay.triangles[triangle_bottom_to_top[0]]].insert(delaunay.triangles[triangle_bottom_to_top[2]]);

                graph[delaunay.triangles[triangle_bottom_to_top[1]]].insert(delaunay.triangles[triangle_bottom_to_top[2]]);
        }
    }

    if full_conection {
        
        for i in 0..graph.len() {
            overworld.nodes[i].connect_to = graph[i].clone();
        }

    } else {

        let mut visited: HashSet<usize> = HashSet::new();
        let main_paths= (rng.gen::<f64>() * (graph[0].len() - 1) as f64) as usize + 1;
    
        for _ in 0..main_paths {
            let mut curr_node = 0;
            while overworld.nodes[curr_node].node_type != WorldNodeType::Boss  {
                let outgoing_connections = &graph[curr_node];
                let prev_node =  curr_node;
                let mut paths = outgoing_connections.iter().map(|&x| x).collect::<Vec<usize>>();
                paths.sort();
                let new_random_node = (rng.gen::<f64>() * outgoing_connections.len() as f64) as usize;
                curr_node = paths[new_random_node];
                overworld.nodes[prev_node].connect_to.insert(curr_node);
                visited.insert(curr_node as usize);
            }
        }
    
        for i in 1..graph.len()-1 {
            if !visited.contains(&i) {
                let mut possible_nodes_that_reach_unvisited = Vec::new();
                for j in 0..graph.len() {
                    if graph[j].contains(&i) {
                        possible_nodes_that_reach_unvisited.push(j);
                    }
                }
    
                let node_to_connected_to_unvisited = possible_nodes_that_reach_unvisited[(rng.gen::<f64>() * possible_nodes_that_reach_unvisited.len() as f64) as usize];
    
                overworld.nodes[node_to_connected_to_unvisited].connect_to.insert(i);

                let mut paths = graph[i].iter().map(|&x| x).collect::<Vec<usize>>();
                paths.sort();
                overworld.nodes[i].connect_to.insert(paths[(rng.gen::<f64>() * graph[i].len() as f64) as usize]);

                visited.insert(i);
            }
        }
    }

}


pub fn overworld_generation(area: Rect, graph_size: (i32, i32), full_conection: bool, rng: &mut SmallRng) -> Vec<WorldNode> {

    let (graph_width, graph_height) = graph_size;
    let (cell_width, cell_height) = (area.width() as i32 / graph_width, area.height() as i32 / graph_height);

    let mut overworld: Vec<WorldNode> = Vec::new();

    let start_cell = (area.x() + area.width() as i32 / graph_width * graph_width / 2, area.y());
    let start_node = WorldNode{
        node_type: WorldNodeType::Start,
        position: Point::new(start_cell.0, start_cell.1),
        connect_to: HashSet::new(),
    };

    overworld.push(start_node);

    for row_level in 1..graph_height {
        let random_n = rng.gen::<f64>();

        let n_cells_for_row: f64 = (random_n * graph_width as f64 - 1f64) + 1f64; //1 to graph_width

        let mut cells_to_spawn_level: Vec<i32> = (0..n_cells_for_row as usize).map(|_| {(rng.gen::<f64>() * graph_width as f64) as i32}).collect();
        cells_to_spawn_level.sort_by(|a, b| a.partial_cmp(b).unwrap());
        cells_to_spawn_level.dedup();

        for cell_pos in cells_to_spawn_level {

            let cell_offset_x = (cell_width as f64 / 2f64 * rng.gen::<f64>()) as i32;
            let cell_offset_y = (cell_height as f64 / 2f64 * rng.gen::<f64>()) as i32;

            let position_cell = (
                area.x() + cell_width * cell_pos + cell_width / 2 + cell_offset_x, 
                area.y() + cell_height * row_level as i32 - cell_height / 2 + cell_offset_y
            );

            overworld.push(WorldNode{
                node_type: WorldNodeType::Level(0),
                position: Point::new(position_cell.0, position_cell.1),
                connect_to: HashSet::new(),
            });

        }
    }

    let end_cell = area.y() + area.height() as i32 / graph_height * graph_height;
    let end_node = WorldNode{
        node_type: WorldNodeType::Boss,
        position: Point::new(start_cell.0 - cell_width, end_cell - 20),
        connect_to: HashSet::new(),
    };

    overworld.push(end_node);

    //Use triangulate to get all paths to all nodes (check if can use it make points position aswell tho it doesnt make sense)
    let points = &overworld.iter().map(|n| {DelPoint{x: n.position.x as f64, y: n.position.y as f64}}).collect::<Vec<DelPoint>>();
    let delaunay = triangulate(points);

    let mut graph = points.iter().map(|_| {HashSet::new()}).collect::<Vec<HashSet<usize>>>();

    if let Some(delaunay) = delaunay.as_ref() {
        for i in (0..delaunay.triangles.len()).step_by(3) {
                let point1 = points[delaunay.triangles[i]].y;
                let point2 = points[delaunay.triangles[i+1]].y;
                let point3 = points[delaunay.triangles[i+2]].y;

                let mut sorted_points = vec![(i, point1),(i+1, point2),(i+2, point3)];
                sorted_points.sort_by(|a, b| (a.1 as i32).cmp(&(b.1 as i32)));

                let triangle_bottom_to_top = sorted_points.iter().map(|p| {p.0}).collect::<Vec<usize>>();

                graph[delaunay.triangles[triangle_bottom_to_top[0]]].insert(delaunay.triangles[triangle_bottom_to_top[1]]);
                graph[delaunay.triangles[triangle_bottom_to_top[0]]].insert(delaunay.triangles[triangle_bottom_to_top[2]]);

                graph[delaunay.triangles[triangle_bottom_to_top[1]]].insert(delaunay.triangles[triangle_bottom_to_top[2]]);
        }
    } else {
        //retry with different seed
    }

    if full_conection {
        
        for i in 0..graph.len() {
            overworld[i].connect_to = graph[i].clone();
        }

    } else {

        let mut visited: HashSet<usize> = HashSet::new();
        let main_paths_rng = rng.gen::<f64>();
        let main_paths= (main_paths_rng * (graph[0].len() - 1) as f64) as usize + 1;

        for _ in 0..main_paths {
            let mut curr_node = 0;
            while overworld[curr_node].node_type != WorldNodeType::Boss  {
                let outgoing_connections = &graph[curr_node];
                let prev_node =  curr_node;
                let new_random_node = (rng.gen::<f64>() * outgoing_connections.len() as f64) as usize;
                let mut paths = outgoing_connections.iter().map(|&x| x).collect::<Vec<usize>>();
                paths.sort();
                curr_node = paths[new_random_node];
                overworld[prev_node].connect_to.insert(curr_node);
                visited.insert(curr_node as usize);
            }
        }
    
        for i in 1..graph.len()-1 {
            if !visited.contains(&i) {
                let mut possible_nodes_that_reach_unvisited = Vec::new();
                for j in 0..graph.len() {
                    if graph[j].contains(&i) {
                        possible_nodes_that_reach_unvisited.push(j);
                    }
                }
                
                let node_to_connected_to_unvisited = possible_nodes_that_reach_unvisited[(rng.gen::<f64>() * possible_nodes_that_reach_unvisited.len() as f64) as usize];
    
                overworld[node_to_connected_to_unvisited].connect_to.insert(i);
                let mut paths = graph[i].iter().map(|&x| x).collect::<Vec<usize>>();
                paths.sort();
                overworld[i].connect_to.insert(paths[(rng.gen::<f64>() * graph[i].len() as f64) as usize]);
                visited.insert(i);
            }
        }
    }

    populate_levels(&mut overworld, rng);

    overworld
}


fn populate_levels(overworld: &mut Vec<WorldNode>, rng: &mut SmallRng){
    let n_nodes = overworld.len();

    overworld[n_nodes/2].node_type = WorldNodeType::Store;

    let level_size_without_start_boss_store = overworld.len() - 3;

    let event_rng = rng.gen::<f64>();
    let events = (event_rng * (level_size_without_start_boss_store as f64 / 2.) as f64) as u32;

    for _ in 0..events {
        let event_location = (rng.gen::<f64>() * level_size_without_start_boss_store as f64) as u32;
        let event_id = (rng.gen::<f64>() * 4 as f64) as u32;

        overworld[event_location as usize].node_type = WorldNodeType::Event(event_id);
    }
}