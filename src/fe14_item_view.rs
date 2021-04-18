use mila::{BinArchiveReader, BinArchiveWriter};

type Result<T> = std::result::Result<T, mila::ArchiveError>;

pub struct FE14ItemView {
    pub address: usize,
    pub bitflags: Vec<u8>,
    pub iid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub id: u16,
    pub item_list_position: u16,
    pub icon: u16,
    pub weapon_category: u8,
    pub non_weapon_category: u8,
    pub required_weapon_exp: u8,
    pub base_staff_exp: u8,
    pub uses: i8,
    pub mt: i8,
    pub hit: i16,
    pub crit: i16,
    pub avoid: i16,
    pub dodge: i16,
    pub min_range: i8,
    pub max_range: i8,
    pub effective_speed_player_phase: i8,
    pub effective_speed_enemy_phase: i8,
    pub buy_price: i32,
    pub sell_price: i32,
    pub effective_damage_1: u8,
    pub effective_damage_2: u8,
    pub mov: u8,
    pub legal_player_weapon: u8,
    pub bonuses: Vec<u8>,
    pub extra_data: Vec<u8>,
    pub forge_table_index: u8,
    pub unknown: u8,
    pub skirmish_item_drop: u8,
    pub unknown_2: u8,
    pub dawn_stock: Vec<u8>,
    pub dusk_stock: Vec<u8>,
    pub rod_stock: Vec<u8>,
    pub staff_stock: Vec<u8>,
    pub hoshido_event: i8,
    pub nohr_event: i8,
    pub dawn_lottery: i8,
    pub dusk_lottery: i8,
    pub my_castle_unknowns: Vec<u8>,
    pub com: Option<String>,
    pub unknown_3: Vec<u8>,
}

impl FE14ItemView {
    pub fn read(reader: &mut BinArchiveReader) -> Result<Self> {
        Ok(FE14ItemView {
            address: reader.tell(),
            bitflags: reader.read_bytes(8)?,
            iid: reader.read_string()?.ok_or_else(|| mila::ArchiveError::OtherError("Null IID.".to_string()))?,
            name: reader.read_string()?,
            description: reader.read_string()?,
            id: reader.read_u16()?,
            item_list_position: reader.read_u16()?,
            icon: reader.read_u16()?,
            weapon_category: reader.read_u8()?,
            non_weapon_category: reader.read_u8()?,
            required_weapon_exp: reader.read_u8()?,
            base_staff_exp: reader.read_u8()?,
            uses: reader.read_i8()?,
            mt: reader.read_i8()?,
            hit: reader.read_i16()?,
            crit: reader.read_i16()?,
            avoid: reader.read_i16()?,
            dodge: reader.read_i16()?,
            min_range: reader.read_i8()?,
            max_range: reader.read_i8()?,
            effective_speed_player_phase: reader.read_i8()?,
            effective_speed_enemy_phase: reader.read_i8()?,
            buy_price: reader.read_i32()?,
            sell_price: reader.read_i32()?,
            effective_damage_1: reader.read_u8()?,
            effective_damage_2: reader.read_u8()?,
            mov: reader.read_u8()?,
            legal_player_weapon: reader.read_u8()?,
            bonuses: reader.read_bytes(8)?,
            extra_data: reader.read_bytes(8)?,
            forge_table_index: reader.read_u8()?,
            unknown: reader.read_u8()?,
            skirmish_item_drop: reader.read_u8()?,
            unknown_2: reader.read_u8()?,
            dawn_stock: reader.read_bytes(3)?,
            dusk_stock: reader.read_bytes(3)?,
            rod_stock: reader.read_bytes(3)?,
            staff_stock: reader.read_bytes(3)?,
            hoshido_event: reader.read_i8()?,
            nohr_event: reader.read_i8()?,
            dawn_lottery: reader.read_i8()?,
            dusk_lottery: reader.read_i8()?,
            my_castle_unknowns: reader.read_bytes(4)?,
            com: reader.read_string()?,
            unknown_3: reader.read_bytes(4)?,
        })
    }

    pub fn write(&self, writer: &mut BinArchiveWriter) -> Result<()> {
        writer.seek(self.address);
        writer.write_bytes(&self.bitflags)?;
        writer.write_string(Some(&self.iid))?;
        writer.write_string(self.name.as_deref())?;
        writer.write_string(self.description.as_deref())?;
        writer.write_u16(self.id)?;
        writer.write_u16(self.item_list_position)?;
        writer.write_u16(self.icon)?;
        writer.write_u8(self.weapon_category)?;
        writer.write_u8(self.non_weapon_category)?;
        writer.write_u8(self.required_weapon_exp)?;
        writer.write_u8(self.base_staff_exp)?;
        writer.write_i8(self.uses)?;
        writer.write_i8(self.mt)?;
        writer.write_i16(self.hit)?;
        writer.write_i16(self.crit)?;
        writer.write_i16(self.avoid)?;
        writer.write_i16(self.dodge)?;
        writer.write_i8(self.min_range)?;
        writer.write_i8(self.max_range)?;
        writer.write_i8(self.effective_speed_player_phase)?;
        writer.write_i8(self.effective_speed_enemy_phase)?;
        writer.write_i32(self.buy_price)?;
        writer.write_i32(self.sell_price)?;
        writer.write_u8(self.effective_damage_1)?;
        writer.write_u8(self.effective_damage_2)?;
        writer.write_u8(self.mov)?;
        writer.write_u8(self.legal_player_weapon)?;
        writer.write_bytes(&self.bonuses)?;
        writer.write_bytes(&self.extra_data)?;
        writer.write_u8(self.forge_table_index)?;
        writer.write_u8(self.unknown)?;
        writer.write_u8(self.skirmish_item_drop)?;
        writer.write_u8(self.unknown_2)?;
        writer.write_bytes(&self.dawn_stock)?;
        writer.write_bytes(&self.dusk_stock)?;
        writer.write_bytes(&self.rod_stock)?;
        writer.write_bytes(&self.staff_stock)?;
        writer.write_i8(self.hoshido_event)?;
        writer.write_i8(self.nohr_event)?;
        writer.write_i8(self.dawn_lottery)?;
        writer.write_i8(self.dusk_lottery)?;
        writer.write_bytes(&self.my_castle_unknowns)?;
        writer.write_string(self.com.as_deref())?;
        writer.write_bytes(&self.unknown_3)?;
        Ok(())
    }
}