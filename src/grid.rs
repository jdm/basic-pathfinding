use std::collections::HashMap;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum GridType {
  Cardinal,
  Hex,
  Intercardinal
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
  tiles: Vec<Vec<i32>>,
  walkable_tiles: Vec<i32>,
  costs: HashMap<i32, i32>,
  extra_costs: HashMap<i32, HashMap<i32, i32>>,
  unstoppable_coords: HashMap<i32, HashMap<i32, bool>>,
  unwalkable_coords: HashMap<i32, HashMap<i32, bool>>,
  grid_type: GridType,
}

impl Grid {
  pub fn new(
    tiles: Vec<Vec<i32>>,
    walkable_tiles: Vec<i32>,
    costs: HashMap<i32, i32>,
    extra_costs: HashMap<i32, HashMap<i32, i32>>,
    unstoppable_coords: HashMap<i32, HashMap<i32, bool>>,
    unwalkable_coords: HashMap<i32, HashMap<i32, bool>>,
    grid_type: GridType,
  ) -> Grid {
    Grid {
      tiles: tiles,
      walkable_tiles: walkable_tiles,
      costs: costs,
      extra_costs: extra_costs,
      unstoppable_coords: unstoppable_coords,
      unwalkable_coords: unwalkable_coords,
      grid_type: grid_type
    }
  }

  pub fn is_cardinal(&self) -> bool {
    match self.grid_type {
      GridType::Cardinal => true,
      _ => false,
    }
  }

  pub fn is_hex(&self) -> bool {
    match self.grid_type {
      GridType::Hex => true,
      _ => false,
    }
  }

  pub fn is_intercardinal(&self) -> bool {
    match self.grid_type {
      GridType::Intercardinal => true,
      _ => false,
    }
  }

  pub fn in_grid(&self, x: i32, y: i32) -> bool {
    match self {
      _grid if (x < 0) | (y < 0) => false,
      grid if (y as usize) < grid.tiles.len() => (x as usize) < grid.tiles[y as usize].len(),
      _ => false,
    }
  }

  pub fn is_coord_stoppable(&self, x: &i32, y: &i32) -> bool {
    if get_nested_bool(&self.unstoppable_coords, x, y) {
      false
    } else {
      self.is_coord_walkable(x, y)
    }
  }

  pub fn is_coord_walkable(&self, x: &i32, y: &i32) -> bool {
    if get_nested_bool(&self.unwalkable_coords, x, y) {
      false
    } else {
      let tile = self.tiles[*y as usize][*x as usize];
      self.walkable_tiles.contains(&tile)
    }
  }

  pub fn get_coord_cost(&self, x: &i32, y: &i32) -> &i32 {
    match self.get_extra_cost(x, y) {
      Some(extra) => extra,
      _ => {
        let tile = self.tiles[*y as usize][*x as usize];
        match self.costs.get(&tile) {
          None => &1,
          Some(cost) => cost,
        }
      },
    }
  }

  fn get_extra_cost(&self, x: &i32, y: &i32) -> Option<&i32> {
    match self.extra_costs.get(&y) {
      Some(inner_hash) => inner_hash.get(&x),
      _ => None,
    }
  }
}

fn get_nested_bool(map: &HashMap<i32, HashMap<i32, bool>>, x: &i32, y: &i32) -> bool {
  match map.get(y) {
    Some(nested) =>
      match nested.get(x) {
        Some(_) => true,
        _ => false,
      },
    _ => false
  }
}