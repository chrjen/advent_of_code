use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub type BlockId = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    min_x: i32,
    min_y: i32,
    min_z: i32,
    max_x: i32,
    max_y: i32,
    max_z: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BlockFallResult {
    /// A map from block_id to an actual block struct with coordinates.
    id_map: HashMap<(i32, i32, i32), BlockId>,
    /// Map that contains the set of blocks that support any given block.
    supportee_for_id: HashMap<BlockId, HashSet<BlockId>>,
    /// Map that contains the set of blocks that any given block is supporting.
    supporter_for_id: HashMap<BlockId, HashSet<BlockId>>,
}

impl BlockFallResult {
    pub fn supportee_for_id(&self) -> &HashMap<BlockId, HashSet<BlockId>> {
        &self.supportee_for_id
    }

    pub fn supporter_for_id(&self) -> &HashMap<BlockId, HashSet<BlockId>> {
        &self.supporter_for_id
    }
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

    pub fn fall_all(blocks: &mut [Block]) -> BlockFallResult {
        let mut height_map: HashMap<(i32, i32), (i32, BlockId)> = HashMap::new();
        let mut fall_result = BlockFallResult::default();

        blocks.sort_by_key(|block| block.min_z);

        for (id, block) in blocks.iter_mut().enumerate() {
            Self::fall(block, id, &mut height_map, &mut fall_result);
        }

        fall_result
    }

    fn fall(
        block: &mut Self,
        id: BlockId,
        height_map: &mut HashMap<(i32, i32), (i32, BlockId)>,
        fall_result: &mut BlockFallResult,
    ) {
        let columns = (block.min_x..=block.max_x).cartesian_product(block.min_y..=block.max_y);

        let new_z = columns
            .clone()
            .flat_map(|column| height_map.get(&column).copied().map(|(h, _)| h))
            .max()
            .unwrap_or(0)
            + 1;
        block.move_z(new_z - block.min_z);

        fall_result
            .id_map
            .insert((block.min_x, block.min_y, block.min_z), id);
        fall_result.supporter_for_id.insert(id, HashSet::new());
        fall_result.supportee_for_id.insert(id, HashSet::new());

        // Lower the block while also updating the height map.
        let supporting_blocks = columns
            .flat_map(|column| height_map.insert(column, (new_z + block.height(), id)))
            .filter_map(|(height, id)| (height == new_z - 1).then_some(id));

        // DO NOT break out of this loop early as it has the side-effect of
        // updating the `height_map` too. Breaking out of the loop will leave
        // the `height_map` only partially updated.
        for supporting_block_id in supporting_blocks {
            fall_result
                .supporter_for_id
                .get_mut(&supporting_block_id)
                .unwrap()
                .insert(id);
            fall_result
                .supportee_for_id
                .get_mut(&id)
                .unwrap()
                .insert(supporting_block_id);
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
