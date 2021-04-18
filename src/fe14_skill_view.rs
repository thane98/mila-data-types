use mila::{BinArchiveReader, BinArchiveWriter};

type Result<T> = std::result::Result<T, mila::ArchiveError>;

pub struct FE14SkillView {
    address: usize,
    pub seid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub effect: Option<String>,
    pub id: u16,
    pub sort_order: i16,
    pub icon: u16,
    pub stat: u8,
    pub trigger_factor: u8,
    pub trigger_divisor: u8,
    pub unknown: u8,
    pub base_price: i16,
    pub unknown_2: u8,
    pub padding: Vec<u8>,
}

impl FE14SkillView {
    pub fn read(reader: &mut BinArchiveReader) -> Result<Self> {
        Ok(FE14SkillView {
            address: reader.tell(),
            seid: reader.read_string()?.ok_or_else(|| mila::ArchiveError::OtherError("Null SEID.".to_string()))?,
            name: reader.read_string()?,
            description: reader.read_string()?,
            effect: reader.read_string()?,
            id: reader.read_u16()?,
            sort_order: reader.read_i16()?,
            icon: reader.read_u16()?,
            stat: reader.read_u8()?,
            trigger_factor: reader.read_u8()?,
            trigger_divisor: reader.read_u8()?,
            unknown: reader.read_u8()?,
            base_price: reader.read_i16()?,
            unknown_2: reader.read_u8()?,
            padding: reader.read_bytes(3)?,
        })
    }

    pub fn write(&self, writer: &mut BinArchiveWriter) -> Result<()> {
        writer.seek(self.address);
        writer.write_string(Some(&self.seid))?;
        writer.write_string(self.name.as_deref())?;
        writer.write_string(self.description.as_deref())?;
        writer.write_string(self.effect.as_deref())?;
        writer.write_u16(self.id)?;
        writer.write_i16(self.sort_order)?;
        writer.write_u16(self.icon)?;
        writer.write_u8(self.stat)?;
        writer.write_u8(self.trigger_factor)?;
        writer.write_u8(self.trigger_divisor)?;
        writer.write_u8(self.unknown)?;
        writer.write_i16(self.base_price)?;
        writer.write_u8(self.unknown_2)?;
        writer.write_bytes(&self.padding)?;
        Ok(())
    }
}
