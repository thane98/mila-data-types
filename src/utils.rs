pub fn read_u16_vec(reader: &mut mila::BinArchiveReader, count: usize) -> Result<Vec<u16>, mila::ArchiveError> {
    let mut values: Vec<u16> = Vec::new();
    for _ in 0..count {
        values.push(reader.read_u16()?);
    }
    Ok(values)
}

pub fn write_u16_vec(writer: &mut mila::BinArchiveWriter, values: &[u16]) -> Result<(), mila::ArchiveError> {
    for value in values {
        writer.write_u16(*value)?;
    }
    Ok(())
}
