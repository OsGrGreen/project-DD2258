pub mod hex;
pub mod layout;
pub mod world_camera;
pub mod tile;
pub mod offset_coords;

//Must be divisible by 2
pub const NUM_ROWS:usize = 100;
pub const NUM_COLMS:usize = 100;

pub static TILE_YIELD:u32 = 2;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct OffsetTile{
    posistion: u32,
}

impl OffsetTile{
    pub fn new(x: u32, y:u32) -> OffsetTile{

        let combined_pos:u32 = ((x << 16) & 4294901760) + y;

        return OffsetTile{
            posistion: combined_pos,
        }
    }

    pub fn get_x(&self) -> u32{
        return (self.posistion & 4294901760) >> 16;
    }

    pub fn get_y(&self) -> u32{
        return self.posistion & 65535;
    }

    pub fn offset(&self, off_x: u32, off_y: u32) -> OffsetTile{
        let new_x = self.get_x()+off_x;
        let new_y = self.get_y()+off_y;
        return OffsetTile::new(new_x, new_y);
    }
}