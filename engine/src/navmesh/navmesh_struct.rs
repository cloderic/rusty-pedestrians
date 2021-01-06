use crate::vec2::Vec2;
use approx::{abs_diff_eq, abs_diff_ne};
use std::collections::HashSet;
use std::io::{Result, Write};
use std::vec::Vec;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum CellEdge {
  Direct(usize),
  Indirect(usize),
}

#[derive(Clone, Debug)]
pub struct Navmesh {
  vertices: Vec<Vec2>,
  edges_vertices: Vec<[usize; 2]>,
  edges_cells: Vec<[Option<usize>; 2]>,
  cells_edges: Vec<[CellEdge; 3]>,
}

impl Navmesh {
  #[allow(dead_code)]
  pub fn count_cells(&self) -> usize {
    self.cells_edges.len()
  }
  #[allow(dead_code)]
  pub fn get_cell(&self, cell_index: usize) -> Option<[&Vec2; 3]> {
    match self.cells_edges.get(cell_index) {
      Some(cell_edges) => {
        let get_cell_vertex = |index| match cell_edges[index] {
          CellEdge::Direct(edge) => self.edges_vertices[edge][0],
          CellEdge::Indirect(edge) => self.edges_vertices[edge][1],
        };
        Some([
          &self.vertices[get_cell_vertex(0)],
          &self.vertices[get_cell_vertex(1)],
          &self.vertices[get_cell_vertex(2)],
        ])
      }
      None => None,
    }
  }
  #[allow(dead_code)]
  pub fn is_belonging_to_cell(&self, cell_index: usize, position: &Vec2) -> bool {
    match self.get_cell(cell_index) {
      Some([v1, v2, v3]) => {
        let det1 = Vec2::det(*v2 - *v1, *position - *v1);
        if det1 < 0. {
          false
        } else {
          let det2 = Vec2::det(*v3 - *v2, *position - *v2);
          if det2 < 0. {
            false
          } else {
            let det3 = Vec2::det(*v1 - *v3, *position - *v3);
            if det3 < 0. {
              false
            } else {
              det1 > 0. || det2 > 0. || det3 > 0.
            }
          }
        }
      }
      None => false,
    }
  }
  fn _locate(&self, position: &Vec2, current_cell_index: usize) -> Option<usize> {
    let (belongs_to_current_cell, mut candidate_cells_index) =
      self.cells_edges[current_cell_index].iter().fold(
        (true, HashSet::new()),
        |(mut belongs_to_current_cell, mut candidates), &cell_edge| {
          // Retrieve the two vertices forming the current cell edge
          let edge_index = match cell_edge {
            CellEdge::Direct(edge_index) => edge_index,
            CellEdge::Indirect(edge_index) => edge_index,
          };
          let [v1_idx, v2_idx] = self.edges_vertices[edge_index];
          let (v1, v2) = (self.vertices[v1_idx], self.vertices[v2_idx]);
          // Computing the determinant between the edge and the position
          let det = Vec2::det(v2 - v1, *position - v1);
          // Updating wether the position belongs to the current cell
          belongs_to_current_cell = belongs_to_current_cell
            && match cell_edge {
              CellEdge::Direct(_) => det >= 0.,
              CellEdge::Indirect(_) => det <= 0.,
            };
          // Updating the potential candidate list
          if det >= 0. {
            if let Some(cell_index) = self.edges_cells[edge_index][0] {
              candidates.insert(cell_index);
            }
          }
          if det <= 0. {
            if let Some(cell_index) = self.edges_cells[edge_index][1] {
              candidates.insert(cell_index);
            }
          }
          (belongs_to_current_cell, candidates)
        },
      );

    if belongs_to_current_cell {
      Some(current_cell_index)
    } else {
      candidate_cells_index.remove(&current_cell_index);
      candidate_cells_index
        .iter()
        .find_map(|&cell_index| self.locate(position, Some(cell_index)))
    }
  }

  #[allow(dead_code)]
  pub fn locate(&self, position: &Vec2, origin_cell_index: Option<usize>) -> Option<usize> {
    if self.cells_edges.is_empty() {
      None
    } else {
      self._locate(position, origin_cell_index.unwrap_or(0))
    }
  }

  pub fn render_to_obj<W: Write>(&self, mut output: W) -> Result<()> {
    self.vertices.iter().try_for_each::<_, Result<()>>(|&v| {
      writeln!(&mut output, "v {:.3} {:.3} 0.0", v.x(), v.y())?;
      Ok(())
    })?;
    self
      .cells_edges
      .iter()
      .try_for_each::<_, Result<()>>(|&c| {
        let get_cell_vertex = |index| match c[index] {
          CellEdge::Direct(edge) => self.edges_vertices[edge][0],
          CellEdge::Indirect(edge) => self.edges_vertices[edge][1],
        };
        writeln!(
          &mut output,
          "f {} {} {}",
          get_cell_vertex(0) + 1,
          get_cell_vertex(1) + 1,
          get_cell_vertex(2) + 1
        )?;
        Ok(())
      })?;
    Ok(())
  }
}

#[derive(Debug)]
pub struct NavmeshBuilder {
  cells: Vec<(Vec2, Vec2, Vec2)>,
}

impl NavmeshBuilder {
  pub fn new() -> Self {
    NavmeshBuilder { cells: Vec::new() }
  }

  pub fn add_cell(mut self, p1: Vec2, p2: Vec2, p3: Vec2) -> Self {
    let det = Vec2::det(p2 - p1, p3 - p1);
    debug_assert!(abs_diff_ne!(det, 0.), "Can't add a sliver triangle cell.");
    if det > 0. {
      self.cells.push((p1, p2, p3));
    } else {
      self.cells.push((p1, p3, p2));
    }
    self
  }

  pub fn build(&self) -> Navmesh {
    let mut vertices = Vec::new();

    let mut add_vertex = |&position: &Vec2| match vertices
      .iter()
      .position(|vertex| abs_diff_eq!(position, vertex))
    {
      None => {
        let index = vertices.len();
        vertices.push(position);
        index
      }
      Some(index) => index,
    };

    let mut edges_vertices = Vec::new();
    let mut edges_cells = Vec::new();

    let mut add_edge = |v1: usize, v2: usize, cell: usize| {
      let cell_on_left = v1 < v2;
      let edge = if cell_on_left { [v1, v2] } else { [v2, v1] };
      match edges_vertices
        .iter()
        .position(|&other_edge| other_edge == edge)
      {
        None => {
          let index = edges_vertices.len();
          edges_vertices.push(edge);
          if cell_on_left {
            edges_cells.push([Some(cell), None]);
            CellEdge::Direct(index)
          } else {
            edges_cells.push([None, Some(cell)]);
            CellEdge::Indirect(index)
          }
        }
        Some(index) => {
          if cell_on_left {
            debug_assert!(edges_cells[index][0].is_none());
            edges_cells[index][0] = Some(cell);
            CellEdge::Direct(index)
          } else {
            debug_assert!(edges_cells[index][1].is_none());
            edges_cells[index][1] = Some(cell);
            CellEdge::Indirect(index)
          }
        }
      }
    };

    let mut cells_edges = Vec::new();
    self.cells.iter().for_each(|(p1, p2, p3)| {
      let v1 = add_vertex(p1);
      let v2 = add_vertex(p2);
      let v3 = add_vertex(p3);
      let cell_index = cells_edges.len();
      cells_edges.push([
        add_edge(v1, v2, cell_index),
        add_edge(v2, v3, cell_index),
        add_edge(v3, v1, cell_index),
      ]);
    });

    Navmesh {
      vertices,
      edges_vertices,
      edges_cells,
      cells_edges,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use approx::assert_relative_eq;
  use std::io::Cursor;

  #[test]
  fn test_unit_square() {
    let navmesh = NavmeshBuilder::new()
      .add_cell(Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(1., 1.))
      .add_cell(Vec2::new(0., 0.), Vec2::new(1., 1.), Vec2::new(0., 1.))
      .build();

    assert_eq!(navmesh.vertices.len(), 4);
    navmesh
      .vertices
      .iter()
      .zip(vec![
        Vec2::new(0., 0.),
        Vec2::new(1., 0.),
        Vec2::new(1., 1.),
        Vec2::new(0., 1.),
      ])
      .for_each(|(value, expected)| {
        assert_relative_eq!(value, &expected);
      });

    assert_eq!(navmesh.edges_vertices.len(), 5);
    navmesh
      .edges_vertices
      .iter()
      .zip(vec![[0, 1], [1, 2], [0, 2], [2, 3], [0, 3]])
      .for_each(|(value, expected)| {
        assert_eq!(value, &expected);
      });

    assert_eq!(navmesh.cells_edges.len(), 2);
    navmesh
      .cells_edges
      .iter()
      .zip(vec![
        [
          CellEdge::Direct(0),
          CellEdge::Direct(1),
          CellEdge::Indirect(2),
        ],
        [
          CellEdge::Direct(2),
          CellEdge::Direct(3),
          CellEdge::Indirect(4),
        ],
      ])
      .for_each(|(value, expected)| {
        assert_eq!(value, &expected);
      });

    assert_eq!(navmesh.edges_cells.len(), 5);
    navmesh
      .edges_cells
      .iter()
      .zip(vec![
        [Some(0), None],
        [Some(0), None],
        [Some(1), Some(0)],
        [Some(1), None],
        [None, Some(1)],
      ])
      .for_each(|(value, expected)| {
        assert_eq!(value, &expected);
      });

    assert_eq!(navmesh.count_cells(), 2);
    assert_relative_eq!(navmesh.get_cell(0).unwrap()[0], &Vec2::new(0., 0.));
    assert_relative_eq!(navmesh.get_cell(0).unwrap()[1], &Vec2::new(1., 0.));
    assert_relative_eq!(navmesh.get_cell(0).unwrap()[2], &Vec2::new(1., 1.));
    assert_relative_eq!(navmesh.get_cell(1).unwrap()[0], &Vec2::new(0., 0.));
    assert_relative_eq!(navmesh.get_cell(1).unwrap()[1], &Vec2::new(1., 1.));
    assert_relative_eq!(navmesh.get_cell(1).unwrap()[2], &Vec2::new(0., 1.));
  }

  #[test]
  fn test_is_belonging_to_cell() {
    let navmesh = NavmeshBuilder::new()
      .add_cell(Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(1., 1.))
      .add_cell(Vec2::new(0., 0.), Vec2::new(0., 1.), Vec2::new(1., 1.))
      .build();

    assert!(navmesh.is_belonging_to_cell(0, &Vec2::new(0.5, 0.1)));
    assert!(!navmesh.is_belonging_to_cell(1, &Vec2::new(0.5, 0.1)));

    assert!(!navmesh.is_belonging_to_cell(0, &Vec2::new(0.1, 0.5)));
    assert!(navmesh.is_belonging_to_cell(1, &Vec2::new(0.1, 0.5)));

    assert!(navmesh.is_belonging_to_cell(0, &Vec2::new(0.5, 0.5)));
    assert!(navmesh.is_belonging_to_cell(1, &Vec2::new(0.5, 0.5)));

    assert!(navmesh.is_belonging_to_cell(0, &Vec2::new(0.5, 0.)));
    assert!(!navmesh.is_belonging_to_cell(1, &Vec2::new(0.5, 0.)));

    assert!(!navmesh.is_belonging_to_cell(0, &Vec2::new(10., -6.3)));
    assert!(!navmesh.is_belonging_to_cell(0, &Vec2::new(10., -6.3)));
  }

  #[test]
  fn test_locate_square() {
    let navmesh = NavmeshBuilder::new()
      .add_cell(Vec2::new(-5., -5.), Vec2::new(5., -5.), Vec2::new(-5., 5.))
      .add_cell(Vec2::new(5., 5.), Vec2::new(5., -5.), Vec2::new(-5., 5.))
      .build();

    assert_eq!(navmesh.locate(&Vec2::new(-2., -2.), None), Some(0));
    assert_eq!(navmesh.locate(&Vec2::new(2., 2.), None), Some(1));

    assert_eq!(navmesh.locate(&Vec2::new(-2., -2.), Some(1)), Some(0));
    assert_eq!(navmesh.locate(&Vec2::new(2., 2.), Some(1)), Some(1));

    assert_eq!(navmesh.locate(&Vec2::new(10., 0.), Some(0)), None);
    assert_eq!(navmesh.locate(&Vec2::new(10., 0.), Some(1)), None);

    assert_eq!(navmesh.locate(&Vec2::new(-1., 1.), Some(0)), Some(0));
    assert_eq!(navmesh.locate(&Vec2::new(-1., 1.), Some(1)), Some(1));
  }

  #[test]
  fn test_render_to_obj() {
    let navmesh = NavmeshBuilder::new()
      .add_cell(Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(1., 1.))
      .add_cell(Vec2::new(0., 0.), Vec2::new(1., 1.), Vec2::new(0., 1.))
      .add_cell(Vec2::new(1., 0.), Vec2::new(2., 0.), Vec2::new(2., 1.))
      .add_cell(Vec2::new(1., 0.), Vec2::new(2., 1.), Vec2::new(1., 1.))
      .build();

    let mut output = Vec::new();
    assert!(navmesh.render_to_obj(&mut Cursor::new(&mut output)).is_ok());
    assert_eq!(
      String::from_utf8(output).unwrap(),
      "v 0.000 0.000 0.0\n\
      v 1.000 0.000 0.0\n\
      v 1.000 1.000 0.0\n\
      v 0.000 1.000 0.0\n\
      v 2.000 0.000 0.0\n\
      v 2.000 1.000 0.0\n\
      f 1 2 3\n\
      f 1 3 4\n\
      f 2 5 6\n\
      f 2 6 3\n"
    );
  }
}
