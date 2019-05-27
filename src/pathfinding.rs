use wasm_bindgen::prelude::*;

use crate::coord::Coord;
use crate::grid::Grid;
use crate::search::Search;
pub use crate::search::SearchOpts;


#[wasm_bindgen]
pub fn find_path_js(grid: &JsValue, start: &JsValue, end: &JsValue, opts: &JsValue) -> JsValue {
  let grid: Grid = grid.into_serde().unwrap();
  let start: Coord = start.into_serde().unwrap();
  let end: Coord = end.into_serde().unwrap();
  let opts: SearchOpts = opts.into_serde().unwrap();

  let result = find_path(&grid, start, end, Some(opts));
  JsValue::from_serde(&result).unwrap()
}

pub fn find_path(grid: &Grid, start: Coord, end: Coord, opts: Option<SearchOpts>) -> Option<Vec<Coord>> {
  let end_on_unstoppable = match &opts {
    None => false,
    Some(opts) => match opts.end_on_unstoppable {
      None => false,
      Some(end_on_unstoppable) => end_on_unstoppable,
    }
  };

  if Coord::equals(Some(start), Some(end)) {
    Some(vec![])
  } else if !grid.is_coord_stoppable(&end.x, &end.y) & !end_on_unstoppable {
    None
  } else {
    let mut search = Search::new(start, Some(end), opts);
    let start_node = search.coordinate_to_node(None, &start.x, &start.y, &0);
    search.push(start_node);

    calculate(&mut search, &grid);

    match search.pop() {
      None => None,
      Some(node) => Some(node.format_path(&search)),
    }
  }
}

#[wasm_bindgen]
pub fn find_walkable_js(grid: &JsValue, source: &JsValue, opts: &JsValue) -> JsValue {
  let grid: Grid = grid.into_serde().unwrap();
  let source: Vec<Coord> = source.into_serde().unwrap();
  let opts: Option<SearchOpts> = Some(opts.into_serde().unwrap());

  let result = find_walkable(&grid, source, opts);
  JsValue::from_serde(&result).unwrap()
}

pub fn find_walkable(grid: &Grid, source: Vec<Coord>, opts: Option<SearchOpts>) -> Vec<Coord> {
  let mut search = Search::new(*source.first().unwrap(), None, opts);

  for coord in source {
    let node = search.coordinate_to_node(None, &coord.x, &coord.y, &0);
    search.push(node);
  }

  calculate(&mut search, &grid);

  let coords = &mut vec![];
  for node in search.traversed_nodes() {
    if grid.is_coord_walkable(&node.x, &node.y) {
      coords.push(Coord::new(node.x, node.y));
    }
  }
  coords.sort();
  coords.to_owned()
}

#[wasm_bindgen]
pub fn to_coord_map(coords: &JsValue) -> JsValue {
  let coords: Vec<Coord> = coords.into_serde().unwrap();
  let hash = Grid::to_coord_map(coords);
  JsValue::from_serde(&hash).unwrap()
}

fn calculate(search: &mut Search, grid: &Grid) {
  match search.size() {
    0 => (),
    _ => {
      if search.reached_destination() {
        ()
      } else {
        let mut node = search.pop().unwrap();
        node.visited = true;
        search.cache(node);

        // cardinal
        if grid.in_grid(node.x, node.y - 1) {
          search.check_adjacent_node(grid, &node, 0, -1)
        }
        // hex & intercardinal
        if !grid.is_cardinal() & grid.in_grid(node.x + 1, node.y - 1) {
          search.check_adjacent_node(grid, &node, 1, -1)
        }
        // cardinal
        if grid.in_grid(node.x + 1, node.y) {
          search.check_adjacent_node(grid, &node, 1, 0)
        }
        // intercardinal
        if grid.is_intercardinal() & grid.in_grid(node.x + 1, node.y + 1) {
          search.check_adjacent_node(grid, &node, 1, 1)
        }
        // cardinal
        if grid.in_grid(node.x, node.y + 1) {
          search.check_adjacent_node(grid, &node, 0, 1)
        }
        // hex & intercardinal
        if !grid.is_cardinal() & grid.in_grid(node.x - 1, node.y + 1) {
          search.check_adjacent_node(grid, &node, -1, 1)
        }
        // cardinal
        if grid.in_grid(node.x - 1, node.y) {
          search.check_adjacent_node(grid, &node, -1, 0)
        }
        // intercardinal
        if grid.is_intercardinal() & grid.in_grid(node.x - 1, node.y - 1) {
          search.check_adjacent_node(grid, &node, -1, -1)
        }

        calculate(search, grid)
      }
    },
  }
}
