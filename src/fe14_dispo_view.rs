use crate::FE14SpawnView;
use mila::{
    ArchiveError, BinArchive, BinArchiveReader, BinArchiveWriter, LayeredFilesystem,
    LayeredFilesystemError,
};
use std::result::Result;

pub struct FE14DispoView {
    archive: mila::BinArchive,
    spawns: Vec<FE14SpawnView>,
}

impl FE14DispoView {
    pub fn load(archive: BinArchive) -> Result<Self, ArchiveError> {
        let mut spawns: Vec<FE14SpawnView> = Vec::new();
        let mut reader = BinArchiveReader::new(&archive, 4);
        let mut next_pointer: Option<usize> = reader.read_pointer()?;
        while let Some(ptr) = next_pointer {
            let count = reader.read_u32()?;
            let end = reader.tell() + 4; // We don't need the faction name, so skip it.

            // Read the spawn data.
            reader.seek(ptr);
            for _ in 0..count {
                spawns.push(FE14SpawnView::read(&mut reader)?);
            }

            // Return to the faction table.
            reader.seek(end);
            next_pointer = reader.read_pointer()?;
        }

        Ok(FE14DispoView {
            archive,
            spawns,
        })
    }

    pub fn commit(&mut self) -> Result<(), ArchiveError> {
        let mut writer = BinArchiveWriter::new(&mut self.archive, 0);
        for spawn in &self.spawns {
            spawn.write(&mut writer)?;
        }
        Ok(())
    }

    pub fn save(&self, fs: &LayeredFilesystem, path: &str) -> Result<(), LayeredFilesystemError> {
        fs.write_archive(path, &self.archive, false)
    }

    pub fn spawns(&self) -> &[FE14SpawnView] {
        &self.spawns
    }

    pub fn spawns_mut(&mut self) -> &mut [FE14SpawnView] {
        &mut self.spawns
    }
}
