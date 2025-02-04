use super::TILE_YIELD;

#[derive(Copy, Clone,Debug)]
pub struct Tile{
    information: u8, //Probably make this an u16 to add some more meta information
                     // Like who owns the tile, if it has building or city and so on...
}

const BIOME_TO_COLOUR: [[f32;3];8] = [
    [0.0,0.0,0.0], // Void, 0
    [0.0,0.3,1.0], // Water, 1
    [0.3,1.0,0.0], // Woods, 2
    [1.0,1.0,0.2], // Plains, 3
    [0.9,0.9,0.9], // Mountain, 4
    [0.5,0.6,0.4], // Hills, 5
    [0.75,0.0,0.0], // Fog of War, 6
    [0.5,0.0,0.5]]; // Debug, 7

const IMPROVEMENT_FACTOR: u32 = 5;
const OCCUPIED_FACTOR:u32 = 2;

impl Tile{
    pub fn new(biome:u16, resource: u16) -> Tile{
        let tile = biome << 5 | resource << 1;
        Tile { information: tile as u8 }
    }

    pub fn get_biome(&self) -> u8{
        return self.information >> 5 & 7
    }

    pub fn get_biome_colour(&self) -> [f32;3]{
        return BIOME_TO_COLOUR[(self.information >> 5 & 7) as usize]
    }

    pub fn get_improved(&self) -> u8{
        return self.information >> 4 & 1
    }

    pub fn get_resource(&self) -> u8{
        return self.information >> 1 & 7
    }

    pub fn get_occupied(&self) -> u8{
        return self.information & 1
    }

    pub fn set_biome(&mut self, new_biome: u8){
        assert!(new_biome < 8);
        self.information = (self.information & 31) | (new_biome << 5);
    }

    pub fn improve(&mut self){
        self.information = (self.information & 239) | ((1 & 1) << 4);
    }

    pub fn set_improved(&mut self, improved: u8){
        self.information = (self.information & 239) | ((improved & 1) << 4);
    }

    pub fn set_resource(&mut self, resource: u8){
        assert!(resource < 8);
        self.information = (self.information & 241) | (resource << 1);
    }

    pub fn set_occupied(&mut self, occupied: u8){
        self.information = (self.information & 254) | (occupied & 1);
    }

    /*
     * 
     * Returns (biome_id, amount of resource gained) 
     * 
     * (Maybe also add turn timer or something if I want to make the tiles produce less and less resources)
     * 
     */
    pub fn harvest(&self) -> (u32, u32){
        let improve_modifier = self.get_improved() as u32 * IMPROVEMENT_FACTOR;
        let occupied_modifier = self.get_occupied() as u32 * OCCUPIED_FACTOR;
        let r#type = self.get_biome() as u32;

        return (r#type, TILE_YIELD*improve_modifier-occupied_modifier)
    }

}