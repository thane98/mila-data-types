use crate::utils::{read_u16_vec, write_u16_vec};
use mila::{BinArchiveReader, BinArchiveWriter};

type Result<T> = std::result::Result<T, mila::ArchiveError>;

pub struct FE14JobView {
    pub address: usize,
    pub bitflags: Vec<u8>,
    pub jid: String,
    pub fid: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub id: u16,
    pub special_flags_1: u8,
    pub special_flags_2: u8,
    pub bases: Vec<u8>,
    pub player_growths: Vec<u8>,
    pub enemy_growths: Vec<u8>,
    pub max_stats: Vec<u8>,
    pub pair_up_bonuses: Vec<u8>,
    pub max_weapon_ranks: Vec<u8>,
    pub hit: i16,
    pub crit: i16,
    pub avoid: i16,
    pub dodge: i16,
    pub skills: Vec<u16>,
    pub mov_cost_table_entry: u8,
    pub mov: u8,
    pub smoke_clouds_size: u8,
    pub extra_flags: u8,
    pub movement_sound: Option<String>,
    pub advanced_classes: Vec<u16>,
    pub base_classes: Vec<u16>,
    pub unknown: Vec<u8>,
    pub gender_equivalent_class: u16,
    pub parallel_class: u16,
    pub origin: u8,
    pub unknown_2: Vec<u8>,
    pub dlc_skill_index: u8,
    pub unknown_3: Vec<u8>,
}

impl FE14JobView {
    pub fn read(reader: &mut BinArchiveReader) -> Result<Self> {
        Ok(FE14JobView {
            address: reader.tell(),
            bitflags: reader.read_bytes(8)?,
            jid: reader.read_string()?.ok_or_else(|| mila::ArchiveError::OtherError("Null JID.".to_string()))?,
            fid: reader.read_string()?,
            name: reader.read_string()?,
            description: reader.read_string()?,
            id: reader.read_u16()?,
            special_flags_1: reader.read_u8()?,
            special_flags_2: reader.read_u8()?,
            bases: reader.read_bytes(8)?,
            player_growths: reader.read_bytes(8)?,
            enemy_growths: reader.read_bytes(8)?,
            max_stats: reader.read_bytes(8)?,
            pair_up_bonuses: reader.read_bytes(8)?,
            max_weapon_ranks: reader.read_bytes(8)?,
            hit: reader.read_i16()?,
            crit: reader.read_i16()?,
            avoid: reader.read_i16()?,
            dodge: reader.read_i16()?,
            skills: read_u16_vec(reader, 4)?,
            mov_cost_table_entry: reader.read_u8()?,
            mov: reader.read_u8()?,
            smoke_clouds_size: reader.read_u8()?,
            extra_flags: reader.read_u8()?,
            movement_sound: reader.read_string()?,
            advanced_classes: read_u16_vec(reader, 2)?,
            base_classes: read_u16_vec(reader, 2)?,
            unknown: reader.read_bytes(4)?,
            gender_equivalent_class: reader.read_u16()?,
            parallel_class: reader.read_u16()?,
            origin: reader.read_u8()?,
            unknown_2: reader.read_bytes(6)?,
            dlc_skill_index: reader.read_u8()?,
            unknown_3: reader.read_bytes(4)?,
        })
    }

    pub fn write(&self, writer: &mut BinArchiveWriter) -> Result<()> {
        writer.seek(self.address);
        writer.write_bytes(&self.bitflags)?;
        writer.write_string(Some(&self.jid))?;
        writer.write_string(self.fid.as_deref())?;
        writer.write_string(self.name.as_deref())?;
        writer.write_string(self.description.as_deref())?;
        writer.write_u16(self.id)?;
        writer.write_u8(self.special_flags_1)?;
        writer.write_u8(self.special_flags_2)?;
        writer.write_bytes(&self.bases)?;
        writer.write_bytes(&self.player_growths)?;
        writer.write_bytes(&self.enemy_growths)?;
        writer.write_bytes(&self.max_stats)?;
        writer.write_bytes(&self.pair_up_bonuses)?;
        writer.write_bytes(&self.max_weapon_ranks)?;
        writer.write_i16(self.hit)?;
        writer.write_i16(self.crit)?;
        writer.write_i16(self.avoid)?;
        writer.write_i16(self.dodge)?;
        write_u16_vec(writer, &self.skills)?;
        writer.write_u8(self.mov_cost_table_entry)?;
        writer.write_u8(self.mov)?;
        writer.write_u8(self.smoke_clouds_size)?;
        writer.write_u8(self.extra_flags)?;
        writer.write_string(self.movement_sound.as_deref())?;
        write_u16_vec(writer, &self.advanced_classes)?;
        write_u16_vec(writer, &self.base_classes)?;
        writer.write_bytes(&self.unknown)?;
        writer.write_u16(self.gender_equivalent_class)?;
        writer.write_u16(self.parallel_class)?;
        writer.write_u8(self.origin)?;
        writer.write_bytes(&self.unknown_2)?;
        writer.write_u8(self.dlc_skill_index)?;
        writer.write_bytes(&self.unknown_3)?;
        Ok(())
    }
}
