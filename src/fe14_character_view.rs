use crate::utils::{read_u16_vec, write_u16_vec};
use mila::{BinArchiveReader, BinArchiveWriter};

type Result<T> = std::result::Result<T, mila::ArchiveError>;

pub struct FE14CharacterView {
    pub address: usize,
    pub bitflags: Vec<u8>,
    pub pid: String,
    pub fid: Option<String>,
    pub aid: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub pointers: Vec<u8>,
    pub id: u16,
    pub support_route: u8,
    pub army: u8,
    pub replacing: u16,
    pub parent: u16,
    pub classes: Vec<u16>,
    pub support_id: i16,
    pub level: i8,
    pub internal_level: i8,
    pub enemy_flag: u8,
    pub unknown: Vec<u8>,
    pub bases: Vec<u8>,
    pub growths: Vec<u8>,
    pub modifiers: Vec<u8>,
    pub penalties: Vec<u8>,
    pub bonuses: Vec<u8>,
    pub weapon_ranks: Vec<u8>,
    pub skills: Vec<u16>,
    pub skill_flags: Vec<u8>,
    pub personal_skills: Vec<u16>,
    pub birthday: Vec<u8>,
    pub reclass_1: Vec<u16>,
    pub parent_id: u16,
    pub child_id: u16,
    pub support_index: i16,
    pub level_cap: u8,
    pub body_type: u8,
    pub combat_music: Option<String>,
    pub voice: Option<String>,
    pub recruit_equip_weapon: u16,
    pub special_shop_item: u16,
    pub padding: Vec<u8>,
}

impl FE14CharacterView {
    pub fn read(reader: &mut BinArchiveReader) -> Result<Self> {
        Ok(FE14CharacterView {
            address: reader.tell(),
            bitflags: reader.read_bytes(8)?,
            pid: reader
                .read_string()?
                .ok_or_else(|| mila::ArchiveError::OtherError("Null PID.".to_string()))?,
            fid: reader.read_string()?,
            aid: reader.read_string()?,
            name: reader.read_string()?,
            description: reader.read_string()?,
            pointers: reader.read_bytes(8)?,
            id: reader.read_u16()?,
            support_route: reader.read_u8()?,
            army: reader.read_u8()?,
            replacing: reader.read_u16()?,
            parent: reader.read_u16()?,
            classes: read_u16_vec(reader, 2)?,
            support_id: reader.read_i16()?,
            level: reader.read_i8()?,
            internal_level: reader.read_i8()?,
            enemy_flag: reader.read_u8()?,
            unknown: reader.read_bytes(3)?,
            bases: reader.read_bytes(8)?,
            growths: reader.read_bytes(8)?,
            modifiers: reader.read_bytes(8)?,
            penalties: reader.read_bytes(8)?,
            bonuses: reader.read_bytes(8)?,
            weapon_ranks: reader.read_bytes(8)?,
            skills: read_u16_vec(reader, 5)?,
            skill_flags: reader.read_bytes(2)?,
            personal_skills: read_u16_vec(reader, 3)?,
            birthday: reader.read_bytes(2)?,
            reclass_1: read_u16_vec(reader, 2)?,
            parent_id: reader.read_u16()?,
            child_id: reader.read_u16()?,
            support_index: reader.read_i16()?,
            level_cap: reader.read_u8()?,
            body_type: reader.read_u8()?,
            combat_music: reader.read_string()?,
            voice: reader.read_string()?,
            recruit_equip_weapon: reader.read_u16()?,
            special_shop_item: reader.read_u16()?,
            padding: reader.read_bytes(4)?,
        })
    }

    pub fn write(&self, writer: &mut BinArchiveWriter) -> Result<()> {
        writer.seek(self.address);
        writer.write_bytes(&self.bitflags)?;
        writer.write_string(Some(&self.pid))?;
        writer.write_string(self.fid.as_deref())?;
        writer.write_string(self.aid.as_deref())?;
        writer.write_string(self.name.as_deref())?;
        writer.write_string(self.description.as_deref())?;
        writer.write_bytes(&self.pointers)?;
        writer.write_u16(self.id)?;
        writer.write_u8(self.support_route)?;
        writer.write_u8(self.army)?;
        writer.write_u16(self.replacing)?;
        writer.write_u16(self.parent)?;
        write_u16_vec(writer, &self.classes)?;
        writer.write_i16(self.support_id)?;
        writer.write_i8(self.level)?;
        writer.write_i8(self.internal_level)?;
        writer.write_u8(self.enemy_flag)?;
        writer.write_bytes(&self.unknown)?;
        writer.write_bytes(&self.bases)?;
        writer.write_bytes(&self.growths)?;
        writer.write_bytes(&self.modifiers)?;
        writer.write_bytes(&self.penalties)?;
        writer.write_bytes(&self.bonuses)?;
        writer.write_bytes(&self.weapon_ranks)?;
        write_u16_vec(writer, &self.skills)?;
        writer.write_bytes(&self.skill_flags)?;
        write_u16_vec(writer, &self.personal_skills)?;
        writer.write_bytes(&self.birthday)?;
        write_u16_vec(writer, &self.reclass_1)?;
        writer.write_u16(self.parent_id)?;
        writer.write_u16(self.child_id)?;
        writer.write_i16(self.support_index)?;
        writer.write_u8(self.level_cap)?;
        writer.write_u8(self.body_type)?;
        writer.write_string(self.combat_music.as_deref())?;
        writer.write_string(self.voice.as_deref())?;
        writer.write_u16(self.recruit_equip_weapon)?;
        writer.write_u16(self.special_shop_item)?;
        writer.write_bytes(&self.padding)?;
        Ok(())
    }
}
