use mila::{BinArchiveReader, BinArchiveWriter};

type Result<T> = std::result::Result<T, mila::ArchiveError>;

pub struct FE14ChapterView {
    address: usize,
    pub cid: String,
    pub battlefield: Option<String>,
    pub id: u8,
    pub chapter_type: u8,
    pub birthright_next_chapter: u8,
    pub conquest_next_chapter: u8,
    pub revelation_next_chapter: u8,
    pub birthright_requirement: u8,
    pub conquest_requirement: u8,
    pub revelation_requirement: u8,
    pub married_character: u16,
    pub offspring_seal_level: u8,
    pub offspring_seal_level_2: u8,
    pub route: u8,
    pub unknown: Vec<u8>,
    pub battle_prep_screen: u8,
    pub unknown_2: Vec<u8>,
}

impl FE14ChapterView {
    pub fn read(reader: &mut BinArchiveReader) -> Result<Self> {
        Ok(FE14ChapterView {
            address: reader.tell(),
            cid: reader.read_string()?.ok_or_else(|| mila::ArchiveError::OtherError("Null CID.".to_string()))?,
            battlefield: reader.read_string()?,
            id: reader.read_u8()?,
            chapter_type: reader.read_u8()?,
            birthright_next_chapter: reader.read_u8()?,
            conquest_next_chapter: reader.read_u8()?,
            revelation_next_chapter: reader.read_u8()?,
            birthright_requirement: reader.read_u8()?,
            conquest_requirement: reader.read_u8()?,
            revelation_requirement: reader.read_u8()?,
            married_character: reader.read_u16()?,
            offspring_seal_level: reader.read_u8()?,
            offspring_seal_level_2: reader.read_u8()?,
            route: reader.read_u8()?,
            unknown: reader.read_bytes(3)?,
            battle_prep_screen: reader.read_u8()?,
            unknown_2: reader.read_bytes(3)?,
        })
    }

    pub fn write(&self, writer: &mut BinArchiveWriter) -> Result<()> {
        writer.seek(self.address);
        writer.write_string(Some(&self.cid))?;
        writer.write_string(self.battlefield.as_deref())?;
        writer.write_u8(self.id)?;
        writer.write_u8(self.chapter_type)?;
        writer.write_u8(self.birthright_next_chapter)?;
        writer.write_u8(self.conquest_next_chapter)?;
        writer.write_u8(self.revelation_next_chapter)?;
        writer.write_u8(self.birthright_requirement)?;
        writer.write_u8(self.conquest_requirement)?;
        writer.write_u8(self.revelation_requirement)?;
        writer.write_u16(self.married_character)?;
        writer.write_u8(self.offspring_seal_level)?;
        writer.write_u8(self.offspring_seal_level_2)?;
        writer.write_u8(self.route)?;
        writer.write_bytes(&self.unknown)?;
        writer.write_u8(self.battle_prep_screen)?;
        writer.write_bytes(&self.unknown_2)?;
        Ok(())
    }
}