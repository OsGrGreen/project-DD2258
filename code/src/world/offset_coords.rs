use crate::world::layout::{EVEN, ODD};

use super::{hex::Hex, OffsetTile};



/**
 * For flat rotation
 */
pub fn qoffset_from_cube(offset: i32, h:&Hex) -> OffsetTile{
    assert!(offset == EVEN || offset == ODD);
    let col = h.get_q() as isize;
    let row: i32 = h.get_r() + (h.get_q() + offset * (h.get_q() & 1)) / 2 as i32;
    return OffsetTile::new(col as u32, row as u32);
}

pub fn qoffset_from_cube_offsets(offset: i32, h:&Hex) -> (isize,isize){
    assert!(offset == EVEN || offset == ODD);
    let col = h.get_q() as isize;
    let row: i32 = h.get_r() + (h.get_q() + offset * (h.get_q() & 1)) / 2 as i32;
    return (col, row as isize);
}

/**
 * For flat rotation
 */
pub fn qoffset_to_cube(offset: i32, tile: OffsetTile) -> Hex{
    let col = tile.get_x() as i32;
    let row = tile.get_y() as i32;
    assert!(offset == EVEN || offset == ODD);
    let q = col;
    let r: i32 = row - (col + offset * (col & 1)) / 2 as i32;
    let s = -q-r;
    Hex::new(q, r, s)
}

pub fn qoffset_to_cube_offsets(offset: i32, tile: (u32,u32)) -> Hex{
    let col = tile.0 as i32;
    let row = tile.1 as i32;
    assert!(offset == EVEN || offset == ODD);
    let q = col;
    let r: i32 = row - (col + offset * (col & 1)) / 2 as i32;
    let s = -q-r;
    Hex::new(q, r, s)
}


pub fn roffset_from_cube(offset: i32, h: Hex) -> (u32,u32) {
    let col: i32 = h.get_q() + (h.get_r() + offset * (h.get_r() & 1)) / 2 as i32;
    let row: i32 = h.get_r();
    if offset != EVEN && offset != ODD {
        panic!("offset must be EVEN (+1) or ODD (-1)");
    }
    return (col as u32,row as u32);
}

/*pub fn roffset_to_cube(offset: i32, h: (u32,u32)) -> Hex {
    let col = h.0;
    let row = h.1;
    let mut q: i32 = h.col - (h.row + offset * (h.row & 1)) / 2 as i32;
    let mut r: i32 = h.row;
    let mut s: i32 = -q - r;
    if offset != EVEN && offset != ODD {
        panic!("offset must be EVEN (+1) or ODD (-1)");
    }
    return Hex { q: q, r: r, s: s };
}*/