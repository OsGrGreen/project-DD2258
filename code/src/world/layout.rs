
use super::hex::{FractionalHex, Hex};

#[derive(Copy, Clone,Debug)]
pub struct Orientation {
    f0:f32,f1:f32,f2:f32,f3:f32,
    b0:f32,b1:f32,b2:f32,b3:f32,
    start_angle: f32, //in kultiples of 60 degrees
}

impl Orientation {
    pub const fn new(f0:f32,f1:f32,f2:f32,f3:f32,b0:f32,b1:f32,b2:f32,b3:f32,start_angle:f32) -> Orientation{
        Orientation{
            f0:f0,f1:f1,f2:f2,f3:f3,
            b0:b0,b1:b1,b2:b2,b3:b3,
            start_angle:start_angle
        }
    }
}

pub const SQRT3:f32 = 1.7320508;

pub const EVEN:i32 = 1;
pub const ODD:i32 = -1;

const LAYOUT_POINTY: Orientation = Orientation::new(SQRT3,SQRT3/2.0,0.0,3.0/2.0,SQRT3/3.0,-1.0/3.0,0.0,2.0/3.0,0.5);

const LAYOUT_FLAT: Orientation = Orientation::new(3.0 / 2.0, 0.0, SQRT3 / 2.0, SQRT3,2.0 / 3.0, 0.0, -1.0 / 3.0, SQRT3 / 3.0, 0.0);

#[derive(Copy, Clone,Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}


// See notes for the implementation design: https://www.redblobgames.com/grids/hexagons/implementation.html
// Possibly have vectors/matricies and use multiplication
#[derive(Copy, Clone,Debug)]
pub struct HexLayout{
    orientation: Orientation,
    pub size: Point,
    origin: Point,
    is_flat: bool,
}

impl HexLayout {
    
    pub const fn new(ori:Orientation,size:Point,origin:Point) -> HexLayout{
        HexLayout { orientation: ori, size: size, origin: origin, is_flat:false}
    }

    pub fn new_flat(size:Point,origin:Point) -> HexLayout{
        HexLayout { orientation: LAYOUT_FLAT, size: size, origin: origin,is_flat:true}
    }

    pub fn new_pointy(size:Point,origin:Point) -> HexLayout{
        HexLayout { orientation: LAYOUT_POINTY, size: size, origin: origin, is_flat:false}
    }

    pub fn get_height(&self) -> f32{
        SQRT3*self.size.x
    }

    pub fn get_width(&self) -> f32{
        3.0/2.0 * self.size.y
    }

    pub fn hex_to_pixel(&self,h:&Hex) -> Point{
        let x:f32 = (self.orientation.f0*h.get_q() as f32+self.orientation.f1*h.get_r() as f32) * self.size.x;
        let y:f32 = (self.orientation.f2*h.get_q() as f32+self.orientation.f3*h.get_r() as f32) * self.size.y;
        Point { x: x+self.origin.x, y: y+self.origin.y }
    }

    pub fn pixel_to_hex(&self,p:&Point) -> FractionalHex{
        //Point pt = Point((p.x - layout.origin.x) / layout.size.x, (p.y - layout.origin.y) / layout.size.y);
        let pt: Point = Point{x:(p.x-self.origin.x)/self.size.x, y:(p.y-self.origin.y)/self.size.y};
        let q: f32 = self.orientation.b0*pt.x + self.orientation.b1*pt.y; 
        let r: f32 = self.orientation.b2*pt.x + self.orientation.b3*pt.y; 
        return FractionalHex::new(q, r, -q - r);
    }

    pub fn hex_corner_offset(&self, corner:u8) -> Point{
        let angle: f32 = 2.0*std::f32::consts::PI*(self.orientation.start_angle+corner as f32)/6.0;
        return Point{x:self.size.x*angle.cos(),y:self.size.y*angle.sin()}
    }

    pub fn polygon_corners(&self, h:&Hex) -> Vec<Point>{
        let mut corners: Vec<Point> = vec![];

        let center: Point = self.hex_to_pixel(h);
        for i in 0..6{
            let offset: Point = self.hex_corner_offset(i);
            corners.push(Point{x:center.x+offset.x,y:center.y+offset.y});
        }

        return corners;
    }

}