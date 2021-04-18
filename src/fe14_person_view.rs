use crate::FE14CharacterView;
use mila::{
    ArchiveError, BinArchive, BinArchiveReader, BinArchiveWriter, LayeredFilesystem,
    LayeredFilesystemError,
};
use std::result::Result;

pub struct FE14PersonView {
    archive: BinArchive,
    characters: Vec<FE14CharacterView>,
}

impl FE14PersonView {
    pub fn load(archive: BinArchive) -> Result<Self, ArchiveError> {
        let mut characters: Vec<FE14CharacterView> = Vec::new();
        let mut reader = BinArchiveReader::new(&archive, 4);
        let count = reader.read_u16()?;
        reader.seek(0x10);
        for _ in 0..count {
            characters.push(FE14CharacterView::read(&mut reader)?);
        }
        Ok(FE14PersonView {
            archive,
            characters,
        })
    }

    pub fn commit(&mut self) -> Result<(), ArchiveError> {
        let mut writer = BinArchiveWriter::new(&mut self.archive, 0);
        for character in &self.characters {
            character.write(&mut writer)?;
        }
        Ok(())
    }

    pub fn save(&self, fs: &LayeredFilesystem, path: &str) -> Result<(), LayeredFilesystemError> {
        fs.write_archive(path, &self.archive, false)
    }

    pub fn characters(&self) -> &[FE14CharacterView] {
        &self.characters
    }

    pub fn characters_mut(&mut self) -> &mut [FE14CharacterView] {
        &mut self.characters
    }
}
