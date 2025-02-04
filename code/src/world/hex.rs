use std::{cmp::{max, min}, ops};


use super::{layout::EVEN, offset_coords::{qoffset_from_cube, qoffset_to_cube}, OffsetTile};


#[derive(Copy, Clone,Debug)]
pub struct Hex{ //Axial storage, cube constructor
    q: i32,
    r: i32,
}

impl Hex{

    const HEX_DIRECTIONS:[Hex;6]  = [
        Hex{q:1, r:0}, Hex{q:1, r:-1}, Hex{q:0, r:-1}, 
        Hex{q:-1, r:0}, Hex{q:-1, r:1}, Hex{q:0, r:1}
    ];

    pub fn new(q: i32, r:i32, s:i32) -> Hex{
        assert!(q+r+s==0);
        Hex { q: q, r: r }
    }

    pub fn get_s(&self) -> i32{
        return -self.q - self.r
    }

    pub fn get_r(&self) -> i32{
        return self.r
    }

    pub fn get_q(&self) -> i32{
        return self.q
    }

    pub fn hex_length(&self) -> u32 {
        return (self.q.abs() + self.r.abs() +(self.get_s().abs()) / 2) as u32;
    }

    pub fn hex_distance(a:&Hex, b:&Hex) -> u32{
        return (a - b).hex_length();
    }

    // Make these two functions work better...
    pub fn hex_direction(direction:usize /* 0 to 5 */) -> Hex {
        assert!(direction < 6);
        return Hex::HEX_DIRECTIONS[direction];
    }

    pub fn hex_neighbor(&self, direction:usize)->Hex{
        return self+&Hex::hex_direction(direction)
    }

    pub fn neighbors_in_range(center: Hex, n: u16) -> Vec<Hex>{
        let range: i32 = n as i32;
        let mut results:Vec<Hex> = vec![];

        for q in -range..=range{
            for r in max(-range,-q-range)..=min(range,-q+range){
                let s = -q - r;
                results.push(center+Hex::new(q, r, s));
            }
        }

        return results;
    }

    pub fn neighbors_in_range_offset(center: OffsetTile, n: u16) -> Vec<OffsetTile>{
        let center_hex = qoffset_to_cube(EVEN, center);
        let range: i32 = n as i32;
        let mut results:Vec<OffsetTile> = vec![];
        for q in -range..range+1{
            //println!("Q: {}", q);
            let r1 = (-range).max(-q - range);
            let r2 = range.min(-q + range);
            for r in r1..r2+1{
                //println!("R: {}", r);
                let s = -q - r;
                let to_push = center_hex+Hex::new(q, r, s);
                // This becomes wrong for some reason
                // Since the offset goes the wrong way or something weird as fuck my dude.


                // Qoffset from cube ger kanske inte direkt utifrån vår vector...
                let tile = qoffset_from_cube(EVEN, &to_push);

                results.push(tile);
            }
        }

        return results;
    }

    pub fn axial_subtract(a: &Hex, b:&Hex) -> Hex{
        let q = a.q - b.q;
        let r = a.r - b.r;
        return Hex::new(q, r, -q-r);
    }
    
    pub fn distance_slow(&self, b: Hex) -> i32{
        return ((self.get_q()-b.get_q()).abs()).max((self.get_r()-b.get_r()).abs().max((self.get_s()-b.get_s()).abs()))
    }

    pub fn distance(&self, b: Hex) -> u16{
        let sub_hex = Hex::axial_subtract(self, &b);
        return ((sub_hex.q.abs() 
            + (sub_hex.q + sub_hex.r).abs()
            + sub_hex.r.abs() ) / 2) as u16;
    }

}



impl PartialEq<Hex> for Hex {
    fn eq(&self, other: &Hex) -> bool {
        return self.q == other.q && self.r == other.r && -self.q-self.r == -other.q - other.r;
    }
}

impl ops::Add<Hex> for Hex{
    type Output = Hex;

    fn add(self, other: Hex) -> Hex {
        return Hex::new(self.q+other.q, self.r+other.r, self.get_s()+other.get_s())
    }
}

impl ops::Add<&Hex> for &Hex{
    type Output = Hex;

    fn add(self, other: &Hex) -> Hex {
        return Hex::new(self.q+other.q, self.r+other.r, self.get_s()+other.get_s())
    }
}

impl ops::Sub<Hex> for Hex{
    type Output = Hex;

    fn sub(self, other: Hex) -> Hex {
        return Hex::new(self.q-other.q, self.r-other.r, self.get_s()-other.get_s())
    }
}

impl ops::Sub<&Hex> for &Hex{
    type Output = Hex;

    fn sub(self, other: &Hex) -> Hex {
        return Hex::new(self.q-other.q, self.r-other.r, self.get_s()-other.get_s())
    }
}

impl ops::Mul<&Hex> for &Hex{
    type Output = Hex;

    fn mul(self, other: &Hex) -> Hex {
        return Hex::new(self.q*other.q, self.r*other.r, self.get_s()*other.get_s())
    }
}

impl ops::Mul<Hex> for Hex{
    type Output = Hex;

    fn mul(self, other: Hex) -> Hex {
        return Hex::new(self.q*other.q, self.r*other.r, self.get_s()*other.get_s())
    }
}

pub struct FractionalHex{
    q:f32,
    r:f32,
    s:f32,
}

impl FractionalHex{
    pub fn new(q:f32,r:f32,s:f32) -> FractionalHex{
        FractionalHex{
            q:q,
            r:r,
            s:s,
        }
    }
    
    pub fn hex_round(&self) -> Hex{
        let mut q:i32 = self.q.round() as i32;
        let mut r:i32 = self.r.round() as i32;
        let mut s:i32 = self.s.round() as i32;
        let q_diff:f32 = (q as f32 - self.q).abs();
        let r_diff:f32 = (r as f32 - self.r).abs();
        let s_diff:f32 = (s as f32 - self.s).abs();
        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        } else {
            s = -q - r;
        }
        return Hex::new(q,r,s);
    }
}