use bitflags::bitflags;

use super::write as write_action0;
use super::super::{ActionTrait, Feature, Output, vec_list};

bitflags! {
    pub struct Flags : u8 {
    }
}

bitflags! {
    pub struct CallbackFlags : u8 {
        const SPRITE_LAYOUT = 0x02;
    }
}

pub struct StationBuildingSprite {
    sprite: u32,
    offset_x: u8,
    offset_y: u8,
    offset_z: u8,
    size_x: u8,
    size_y: u8,
    size_z: u8,
}

pub struct StationLayout {
    ground_sprite: u32,
    building_sprites: Vec<StationBuildingSprite>,
}

pub enum Station<'a> {
    Class { id: u8, label: &'a str },                                          // 08
                                                                               // 09 (depracated by 1A)
                                                                               // 0a (won't implement)
    CallbackFlags { id: u8, flags: CallbackFlags },                            // 0b
    SizeMask { id: u8, width_mask: u8, height_mask: u8 },                      // 0c, 0d
                                                                               // 0e (unused)
                                                                               // 0f (won't implement)
                                                                               // 10 (won't implement)
    TileType { id: u8, pylon_mask: u8, wire_mask: u8, platform_mask: u8 },     // 11, 14, 15
    Flags { id: u8, flags: Flags },                                            // 13
                                                                               // 16, 17, 18 (TODO)
                                                                               // 19 (unused)
    Layout { id: u8, layouts: &'a Vec<StationLayout> },                        // 1a
}

impl<'a> ActionTrait for Station<'a> {
    fn write(&self, output: &mut Output) {
        let (id, properties) = match self {
            Station::Class { id, label } => {
                (*id, vec![
                    vec_list!([0x08], label.as_bytes()),
                ])
            }
            Station::CallbackFlags { id, flags } => {
                (*id, vec![
                    vec_list!([0x0b], [flags.bits]),
                ])
            }
            Station::SizeMask { id, width_mask, height_mask } => {
                (*id, vec![
                    vec_list!([0x0c], [width_mask]),
                    vec_list!([0x0d], [height_mask]),
                ])
            }
            Station::TileType { id, pylon_mask, wire_mask, platform_mask } => {
                (*id, vec![
                    vec_list!([0x11], [pylon_mask]),
                    vec_list!([0x14], [wire_mask]),
                    vec_list!([0x15], [platform_mask]),
                ])
            }
            Station::Flags { id, flags: _ } => {
                (*id, vec![
                    vec_list!([0x13]),
                ])
            }
            Station::Layout { id, layouts } => {
                let mut data = Vec::new();

                for layout in *layouts {
                    data.extend([layout.building_sprites.len() as u8]);
                    data.extend(&layout.ground_sprite.to_le_bytes());

                    for building_sprite in &layout.building_sprites {
                        data.extend(&building_sprite.sprite.to_le_bytes());
                        data.extend([building_sprite.offset_x, building_sprite.offset_y, building_sprite.offset_z, building_sprite.size_x, building_sprite.size_y, building_sprite.size_z]);
                    }
                }

                (*id, vec![
                    vec_list!([0x1a], [layouts.len() as u8], data),
                ])
            }
        };

        write_action0(output, Feature::Stations, id, &properties);
    }
}
