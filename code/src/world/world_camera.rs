pub struct WorldCamera{
    x_offset: isize,
    y_offset: isize,
    world_size: (isize, isize),
}

impl WorldCamera{

    pub fn new(world_size: (usize,usize)) -> WorldCamera{
        WorldCamera{x_offset:0,y_offset:0, world_size: (world_size.0 as isize, world_size.1 as isize)}
    }

    pub fn move_camera(&mut self, move_x:isize, move_y:isize){
        self.x_offset = (self.x_offset + move_x) % self.world_size.0;
        self.y_offset = (self.y_offset + move_y) % self.world_size.1;
    } 

    pub fn offsets(&self) -> (isize,isize){
        (self.x_offset,self.y_offset)
    }

}