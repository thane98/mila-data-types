use crate::{FE14ChapterView, FE14CharacterView, FE14ItemView, FE14JobView, FE14SkillView};
use mila::{
    ArchiveError, BinArchive, BinArchiveReader, BinArchiveWriter, LayeredFilesystem,
    LayeredFilesystemError,
};
use std::result::Result;

pub struct FE14GameDataView {
    archive: BinArchive,
    chapters: Vec<FE14ChapterView>,
    characters: Vec<FE14CharacterView>,
    jobs: Vec<FE14JobView>,
    items: Vec<FE14ItemView>,
    skills: Vec<FE14SkillView>,
    personal_skills: Vec<FE14SkillView>,
}

impl FE14GameDataView {
    pub fn load(archive: BinArchive) -> Result<Self, ArchiveError> {
        let mut reader = BinArchiveReader::new(&archive, 0);
        let chapters_address = reader.read_pointer()?;
        let chapters_count = reader.read_u32()?;
        let characters_address = reader.read_pointer()?;
        let jobs_address = reader.read_pointer()?;
        let skills_address = reader.read_pointer()?;
        let normal_skill_count = reader.read_u32()?;
        let total_skill_count = reader.read_u32()?;
        reader.seek(0x2C);
        let items_address = reader.read_pointer()?;
        let mut chapters: Vec<FE14ChapterView> = Vec::new();
        let mut characters: Vec<FE14CharacterView> = Vec::new();
        let mut jobs: Vec<FE14JobView> = Vec::new();
        let mut items: Vec<FE14ItemView> = Vec::new();
        let mut skills: Vec<FE14SkillView> = Vec::new();
        let mut personal_skills: Vec<FE14SkillView> = Vec::new();
        if let Some(addr) = chapters_address {
            reader.seek(addr);
            for _ in 0..chapters_count {
                chapters.push(FE14ChapterView::read(&mut reader)?);
            }
        } else {
            return Err(ArchiveError::OtherError("Bad character table pointer".to_owned()));
        }
        if let Some(addr) = characters_address {
            reader.seek(addr + 4);
            let count = reader.read_u16()?;
            reader.seek(addr + 0x10);
            for _ in 0..count {
                characters.push(FE14CharacterView::read(&mut reader)?);
            }
        } else {
            return Err(ArchiveError::OtherError("Bad character table pointer".to_owned()));
        }
        if let Some(addr) = jobs_address {
            reader.seek(addr + 6);
            let count = reader.read_u16()?;
            for _ in 0..count {
                jobs.push(FE14JobView::read(&mut reader)?);
            }
        } else {
            return Err(ArchiveError::OtherError("Bad class table pointer".to_owned()));
        }
        if let Some(addr) = items_address {
            reader.seek(addr + 6);
            let count = reader.read_u16()?;
            for _ in 0..count {
                items.push(FE14ItemView::read(&mut reader)?);
            }
        } else {
            return Err(ArchiveError::OtherError("Bad item table pointer".to_owned()));
        }
        if let Some(addr) = skills_address {
            reader.seek(addr);
            for _ in 0..normal_skill_count {
                skills.push(FE14SkillView::read(&mut reader)?);
            }
            for _ in normal_skill_count..total_skill_count {
                personal_skills.push(FE14SkillView::read(&mut reader)?);
            }
        } else {
            return Err(ArchiveError::OtherError("Bad skill table pointer".to_owned()));
        }
        Ok(FE14GameDataView {
            chapters,
            archive,
            characters,
            jobs,
            items,
            skills,
            personal_skills
        })
    }

    pub fn commit(&mut self) -> Result<(), ArchiveError> {
        let mut writer = BinArchiveWriter::new(&mut self.archive, 0);
        for chapter in &self.chapters {
            chapter.write(&mut writer)?;
        }
        for character in &self.characters {
            character.write(&mut writer)?;
        }
        for job in &self.jobs {
            job.write(&mut writer)?;
        }
        for item in &self.items {
            item.write(&mut writer)?;
        }
        for skill in &self.skills {
            skill.write(&mut writer)?;
        }
        for skill in &self.personal_skills {
            skill.write(&mut writer)?;
        }
        Ok(())
    }

    pub fn save(&self, fs: &LayeredFilesystem, path: &str) -> Result<(), LayeredFilesystemError> {
        fs.write_archive(path, &self.archive, false)
    }

    pub fn chapters(&self) -> &[FE14ChapterView] {
        &self.chapters
    }

    pub fn characters(&self) -> &[FE14CharacterView] {
        &self.characters
    }

    pub fn jobs(&mut self) -> &[FE14JobView] {
        &self.jobs
    }

    pub fn items(&mut self) -> &[FE14ItemView] {
        &self.items
    }

    pub fn skills(&mut self) -> &[FE14SkillView] {
        &self.skills
    }

    pub fn personal_skills(&mut self) -> &[FE14SkillView] {
        &self.personal_skills
    }

    pub fn chapters_mut(&mut self) -> &mut [FE14ChapterView] {
        &mut self.chapters
    }

    pub fn characters_mut(&mut self) -> &mut [FE14CharacterView] {
        &mut self.characters
    }

    pub fn jobs_mut(&mut self) -> &mut [FE14JobView] {
        &mut self.jobs
    }

    pub fn items_mut(&mut self) -> &mut [FE14ItemView] {
        &mut self.items
    }

    pub fn skills_mut(&mut self) -> &mut [FE14SkillView] {
        &mut self.skills
    }

    pub fn personal_skills_mut(&mut self) -> &mut [FE14SkillView] {
        &mut self.personal_skills
    }
}
