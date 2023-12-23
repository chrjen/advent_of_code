use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    min_x: i32,
    min_y: i32,
    min_z: i32,
    max_x: i32,
    max_y: i32,
    max_z: i32,
}

impl Block {
    pub fn new(x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) -> Self {
        Block {
            min_x: x0.min(x1),
            min_y: y0.min(y1),
            min_z: z0.min(z1),
            max_x: x0.max(x1),
            max_y: y0.max(y1),
            max_z: z0.max(z1),
        }
    }

    pub fn fall_all(blocks: &mut [Block]) -> usize {
        let mut height_map: HashMap<(i32, i32), (i32, usize)> = HashMap::new();
        let mut id_map: HashMap<usize, Block> = HashMap::new();
        let mut removable: HashSet<usize> = HashSet::new();

        let by_min_z = blocks
            .iter_mut()
            .sorted_unstable_by_key(|block| block.min_z);

        for (id, block) in by_min_z.enumerate() {
            Self::fall(block, id, &mut id_map, &mut removable, &mut height_map);
        }

        removable.len()
    }

    pub fn fall(
        block: &mut Self,
        id: usize,
        id_map: &mut HashMap<usize, Block>,
        removable: &mut HashSet<usize>,
        height_map: &mut HashMap<(i32, i32), (i32, usize)>,
    ) {
        let columns = (block.min_x..=block.max_x).cartesian_product(block.min_y..=block.max_y);

        let new_z = columns
            .clone()
            .flat_map(|column| height_map.get(&column).copied().map(|(h, _)| h))
            .max()
            .unwrap_or(0)
            + 1;
        block.move_z(new_z - block.min_z);
        removable.insert(id);
        id_map.insert(id, block.clone());

        // Lower the block while also updating the height map.
        // If this block is only supported by a single other block we know that
        // that block can't be remove or this block will fall.
        let supporting_block: Vec<_> = columns
            .flat_map(|column| height_map.insert(column, (new_z + block.height(), id)))
            .filter_map(|(height, id)| (height == new_z - 1).then_some(id))
            .unique()
            .collect();
        let supporting_block = (supporting_block.len() < 2)
            .then(|| supporting_block.first())
            .flatten();

        // Only supported by at most one block so that block can't be removed
        // so we mark it as such.
        if let Some(b) = supporting_block {
            removable.remove(b);
        }
    }

    pub fn move_z(&mut self, offset: i32) {
        self.min_z += offset;
        self.max_z += offset;
    }

    pub fn height(&self) -> i32 {
        self.max_z - self.min_z
    }

    pub fn _to_python_value(&self) -> String {
        format!(
            "(({},{},{}),({},{},{}))",
            self.min_x, self.min_y, self.min_z, self.max_x, self.max_y, self.max_z
        )
    }
}
