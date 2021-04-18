mod fe14_chapter_view;
mod fe14_character_view;
mod fe14_dispo_view;
mod fe14_gamedata_view;
mod fe14_item_view;
mod fe14_job_view;
mod fe14_person_view;
mod fe14_skill_view;
mod fe14_spawn_view;
mod utils;

pub use fe14_chapter_view::FE14ChapterView;
pub use fe14_character_view::FE14CharacterView;
pub use fe14_dispo_view::FE14DispoView;
pub use fe14_gamedata_view::FE14GameDataView;
pub use fe14_item_view::FE14ItemView;
pub use fe14_job_view::FE14JobView;
pub use fe14_person_view::FE14PersonView;
pub use fe14_skill_view::FE14SkillView;
pub use fe14_spawn_view::FE14SpawnView;


fn main() {
    let fs = mila::LayeredFilesystem::new(
        vec![r"C:\Development\FE\Dumps\Fates".to_owned(), r"C:\Development\FE\FE14Test".to_owned()],
        mila::Language::EnglishNA,
        mila::Game::FE14,
    )
    .unwrap();
    
    let gd = fs.read_archive("GameData/GameData.bin.lz", false).unwrap();
    let mut view = FE14GameDataView::load(gd).unwrap();
    for skill in view.items() {
        println!("{:?}", skill.iid);
    }
    // view.commit().unwrap();
    // view.save(&fs, "GameData/GameData.bin.lz").unwrap();
}
