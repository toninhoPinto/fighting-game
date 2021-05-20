use std::collections::{HashMap, HashSet};

use sdl2::rect::{Point, Rect};
use delaunator::{Point as DelPoint, triangulate};

use self::node::{WorldNode, WorldNodeType};

pub mod node;

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub fn overworld_generation(area: Rect, graph_size: (i32, i32), full_conection: bool) -> Vec<WorldNode> {

    let (graph_width, graph_height) = graph_size;
    let (cell_width, cell_height) = (area.width() as i32 / graph_width, area.height() as i32 / graph_height);

    let mut overworld: Vec<WorldNode> = Vec::new();
    let mut rng = SmallRng::seed_from_u64(96873523456);

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
        for _ in 0..n_cells_for_row as usize {
            let cell_id_to_populate = rng.gen::<f64>();
            let cell_pos = (cell_id_to_populate * graph_width as f64) as i32;
            let cell_offset_x = (cell_width as f64 / 2f64 * rng.gen::<f64>()) as i32;
            let cell_offset_y = (cell_height as f64 / 2f64 * rng.gen::<f64>()) as i32;

            let position_cell = (
                area.x() + cell_width * cell_pos + cell_width / 2 + cell_offset_x, 
                area.y() + cell_height * row_level as i32 - cell_height / 2 + cell_offset_y
            );
            overworld.push(WorldNode{
                node_type: WorldNodeType::Level,
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
    println!("points {:?}", points);
    let delaunay = triangulate(points);

    //Use minimum spanning tree to get paths
    //Add some of the original triangles to avoid having boring maps

    println!("tris {:?}", delaunay.as_ref().unwrap().triangles);

    let n_points = points.len();
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
        let main_paths= (rng.gen::<f64>() * (graph[0].len() - 1) as f64) as usize + 1;
    
        for _ in 0..main_paths {
            let mut curr_node = 0;
            while overworld[curr_node].node_type != WorldNodeType::Boss  {
                let outgoing_connections = &graph[curr_node];
                let prev_node =  curr_node;
                curr_node = outgoing_connections.iter().map(|&x| x).collect::<Vec<usize>>()[rng.gen_range(0..outgoing_connections.len())];
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
    
                let node_to_connected_to_unvisited = possible_nodes_that_reach_unvisited[rng.gen_range(0..possible_nodes_that_reach_unvisited.len())];
    
                overworld[node_to_connected_to_unvisited].connect_to.insert(i);
                overworld[i].connect_to.insert(graph[i].iter().map(|&x| x).collect::<Vec<usize>>()[rng.gen_range(0..graph[i].len())]);
                visited.insert(i);
            }
        }


    }

    overworld
}