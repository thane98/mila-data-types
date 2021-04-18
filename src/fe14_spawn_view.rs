use mila::{BinArchiveReader, BinArchiveWriter};

type Result<T> = std::result::Result<T, mila::ArchiveError>;

pub struct FE14SpawnView {
    address: usize,
    pub pid: String,
    pub unknown_1: Vec<u8>,
    pub team: u8,
    pub level: i8,
    pub coord_1: Vec<u8>,
    pub coord_2: Vec<u8>,
    pub unknown_2: Vec<u8>,
    pub spawn_flags: u8,
    pub spawn_flags_2: u8,
    pub spawn_flags_3: u8,
    pub spawn_flags_4: u8,
    pub item_1: Option<String>,
    pub item_flags_1: Vec<u8>,
    pub item_2: Option<String>,
    pub item_flags_2: Vec<u8>,
    pub item_3: Option<String>,
    pub item_flags_3: Vec<u8>,
    pub item_4: Option<String>,
    pub item_flags_4: Vec<u8>,
    pub item_5: Option<String>,
    pub item_flags_5: Vec<u8>,
    pub skill_1: Option<String>,
    pub skill_2: Option<String>,
    pub skill_3: Option<String>,
    pub skill_4: Option<String>,
    pub skill_5: Option<String>,
    pub skill_flags: Vec<u8>,
    pub ai_action: Option<String>,
    pub ai_action_parameter: Option<String>,
    pub ai_mission: Option<String>,
    pub ai_mission_parameter: Option<String>,
    pub ai_attack: Option<String>,
    pub ai_attack_parameter: Option<String>,
    pub ai_movement: Option<String>,
    pub ai_movement_parameter: Option<String>,
    pub ai_unknown_1: Vec<u8>,
    pub ai_unknown_2: Vec<u8>,
    pub ai_unknown_3: Vec<u8>,
}

impl FE14SpawnView {
    pub fn read(reader: &mut BinArchiveReader) -> Result<Self> {
        Ok(FE14SpawnView {
            address: reader.tell(),
            pid: reader.read_string()?.ok_or_else(|| mila::ArchiveError::OtherError("Null PID.".to_string()))?,
            unknown_1: reader.read_bytes(4)?,
            team: reader.read_u8()?,
            level: reader.read_i8()?,
            coord_1: reader.read_bytes(2)?,
            coord_2: reader.read_bytes(2)?,
            unknown_2: reader.read_bytes(2)?,
            spawn_flags: reader.read_u8()?,
            spawn_flags_2: reader.read_u8()?,
            spawn_flags_3: reader.read_u8()?,
            spawn_flags_4: reader.read_u8()?,
            item_1: reader.read_string()?,
            item_flags_1: reader.read_bytes(4)?,
            item_2: reader.read_string()?,
            item_flags_2: reader.read_bytes(4)?,
            item_3: reader.read_string()?,
            item_flags_3: reader.read_bytes(4)?,
            item_4: reader.read_string()?,
            item_flags_4: reader.read_bytes(4)?,
            item_5: reader.read_string()?,
            item_flags_5: reader.read_bytes(4)?,
            skill_1: reader.read_string()?,
            skill_2: reader.read_string()?,
            skill_3: reader.read_string()?,
            skill_4: reader.read_string()?,
            skill_5: reader.read_string()?,
            skill_flags: reader.read_bytes(4)?,
            ai_action: reader.read_string()?,
            ai_action_parameter: reader.read_string()?,
            ai_mission: reader.read_string()?,
            ai_mission_parameter: reader.read_string()?,
            ai_attack: reader.read_string()?,
            ai_attack_parameter: reader.read_string()?,
            ai_movement: reader.read_string()?,
            ai_movement_parameter: reader.read_string()?,
            ai_unknown_1: reader.read_bytes(8)?,
            ai_unknown_2: reader.read_bytes(8)?,
            ai_unknown_3: reader.read_bytes(8)?,
        })
    }

    pub fn write(&self, writer: &mut BinArchiveWriter) -> Result<()> {
        writer.seek(self.address);
        writer.write_string(Some(&self.pid))?;
        writer.write_bytes(&self.unknown_1)?;
        writer.write_u8(self.team)?;
        writer.write_i8(self.level)?;
        writer.write_bytes(&self.coord_1)?;
        writer.write_bytes(&self.coord_2)?;
        writer.write_bytes(&self.unknown_2)?;
        writer.write_u8(self.spawn_flags)?;
        writer.write_u8(self.spawn_flags_2)?;
        writer.write_u8(self.spawn_flags_3)?;
        writer.write_u8(self.spawn_flags_4)?;
        writer.write_string(self.item_1.as_deref())?;
        writer.write_bytes(&self.item_flags_1)?;
        writer.write_string(self.item_2.as_deref())?;
        writer.write_bytes(&self.item_flags_2)?;
        writer.write_string(self.item_3.as_deref())?;
        writer.write_bytes(&self.item_flags_3)?;
        writer.write_string(self.item_4.as_deref())?;
        writer.write_bytes(&self.item_flags_4)?;
        writer.write_string(self.item_5.as_deref())?;
        writer.write_bytes(&self.item_flags_5)?;
        writer.write_string(self.skill_1.as_deref())?;
        writer.write_string(self.skill_2.as_deref())?;
        writer.write_string(self.skill_3.as_deref())?;
        writer.write_string(self.skill_4.as_deref())?;
        writer.write_string(self.skill_5.as_deref())?;
        writer.write_bytes(&self.skill_flags)?;
        writer.write_string(self.ai_action.as_deref())?;
        writer.write_string(self.ai_action_parameter.as_deref())?;
        writer.write_string(self.ai_mission.as_deref())?;
        writer.write_string(self.ai_mission_parameter.as_deref())?;
        writer.write_string(self.ai_attack.as_deref())?;
        writer.write_string(self.ai_attack_parameter.as_deref())?;
        writer.write_string(self.ai_movement.as_deref())?;
        writer.write_string(self.ai_movement_parameter.as_deref())?;
        writer.write_bytes(&self.ai_unknown_1)?;
        writer.write_bytes(&self.ai_unknown_2)?;
        writer.write_bytes(&self.ai_unknown_3)?;
        Ok(())
    }
}
