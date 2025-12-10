// use std::fmt::Debug;

// use crate::Pos;

// #[derive(Clone, Copy, PartialEq, Eq)]
// enum Tile {
//     Red,
//     Green,
//     Empty,
// }

// impl Debug for Tile {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Red => write!(f, "#"),
//             Self::Green => write!(f, "X"),
//             Self::Empty => write!(f, "."),
//         }
//     }
// }

// #[derive(Clone, PartialEq, Eq)]
// pub struct Grid {
//     min_x: usize,
//     min_y: usize,
//     tiles: Vec<Vec<Tile>>,
// }

// impl Debug for Grid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Draw grid
//         for row in &self.tiles {
//             for tile in row {
//                 write!(f, "{:?}", tile)?;
//             }
//             writeln!(f)?;
//         }
//         Ok(())
//     }
// }

// impl Grid {
//     pub fn new(red_tiles: &[Pos]) -> Self {
//         let (x_, y_) = red_tiles[0];
//         let (min_x, max_x, min_y, max_y) =
//             red_tiles
//                 .iter()
//                 .fold((x_, x_, y_, y_), |(min_x, max_x, min_y, max_y), &(x, y)| {
//                     (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
//                 });
//         let width = max_x - min_x + 1;
//         let height = max_y - min_y + 1;
//         let mut tiles = vec![vec![Tile::Empty; width]; height];
//         for &(x, y) in red_tiles.into_iter() {
//             tiles[y - min_y][x - min_x] = Tile::Red
//         }
//         let mut grid = Self {
//             min_x,
//             min_y,
//             tiles,
//         };
//         grid.fill_green(red_tiles);
//         grid
//     }

//     pub fn fill_line(&mut self, (x1, y1): Pos, (x2, y2): Pos) {
//         if x1 == x2 {
//             for y in y1.min(y2) + 1..y1.max(y2) {
//                 self.tiles[y - self.min_y][x1 - self.min_x] = Tile::Green;
//             }
//         } else {
//             for x in x1.min(x2) + 1..x1.max(x2) {
//                 self.tiles[y1 - self.min_y][x - self.min_x] = Tile::Green;
//             }
//         }
//     }

//     pub fn fill_green(&mut self, red_tiles: &[Pos]) {
//         let wrap = [*red_tiles.first().unwrap(), *red_tiles.last().unwrap()];
//         let mut vertical_edges: Vec<_> = red_tiles
//             .windows(2)
//             .chain([&wrap[..]])
//             .filter_map(|w| {
//                 let (p1, p2) = (w[0], w[1]);
//                 (p1.0 == p2.0).then_some((p1.0, p1.1.min(p2.1), p1.1.max(p2.1)))
//             })
//             .collect();
//         vertical_edges.sort_unstable_by_key(|&(x, _, _)| x);
//         for y in 0..self.tiles.len() {
//             let mut filling = false;
//             for &(x, y1, y2) in &vertical_edges {
//                 if y1

//             }
//         }
//     }
// }
