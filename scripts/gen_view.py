import sys
import yaml

try:
    from yaml import CLoader as Loader, CDumper as Dumper
except ImportError:
    from yaml import Loader, Dumper

with open(sys.argv[1], "r") as f:
    info = yaml.load(f, Loader=Loader)

typename = info["name"]
fields = info["fields"]

print("use crate::utils::{read_u16_vec, write_u16_vec};")
print("use mila::{BinArchiveReader, BinArchiveWriter};")
print()
print("type Result<T> = std::result::Result<T, mila::ArchiveError>;")
print()
print(f"pub struct {typename} {{")
print("    address: usize,")
for field in fields:
    field_id = field["id"]
    print("    ", end="")
    templates = {
        "required_string": f"pub {field_id}: String,",
        "string": f"pub {field_id}: Option<String>,",
        "bytes": f"pub {field_id}: Vec<u8>,",
        "u16_vec": f"pub {field_id}: Vec<u16>,",
        "u8": f"pub {field_id}: u8,",
        "u16": f"pub {field_id}: u16,",
        "i8": f"pub {field_id}: i8,",
        "i16": f"pub {field_id}: i16,",
        "i32": f"pub {field_id}: i32,",
    }
    print(templates[field["type"]])
print("}")
print()
print(f"impl {typename} {{")
print("    pub fn read(reader: &mut BinArchiveReader) -> Result<Self> {")
print(f"        Ok({typename} {{")
print("            address: reader.tell(),")
for field in fields:
    field_id = field["id"]
    print("            ", end="")
    templates = {
        "required_string": f"{field_id}: reader.read_string()?.ok_or_else(|| mila::ArchiveError::OtherError(\"Null PID.\".to_string()))?,",
        "string": f"{field_id}: reader.read_string()?,",
        "bytes": "%s: reader.read_bytes(%s)?," % (field_id, field.get("length", 0)),
        "u16_vec": "%s: read_u16_vec(reader, %s)?," % (field_id, field.get("length", 0)),
        "u8": f"{field_id}: reader.read_u8()?,",
        "u16": f"{field_id}: reader.read_u16()?,",
        "i8": f"{field_id}: reader.read_i8()?,",
        "i16": f"{field_id}: reader.read_i16()?,",
        "i32": f"{field_id}: reader.read_i32()?,",
    }
    print(templates[field["type"]])
print("        })")
print("    }")
print()
print("    pub fn write(&self, writer: &mut BinArchiveWriter) -> Result<()> {")
print("        writer.seek(self.address);")
for field in fields:
    field_id = field["id"]
    print("        ", end="")
    templates = {
        "required_string": f"writer.write_string(Some(&self.{field_id}))?;",
        "string": f"writer.write_string(self.{field_id}.as_deref())?;",
        "bytes": f"writer.write_bytes(&self.{field_id})?;",
        "u16_vec": f"write_u16_vec(writer, &self.{field_id})?;",
        "u8": f"writer.write_u8(self.{field_id})?;",
        "u16": f"writer.write_u16(self.{field_id})?;",
        "i8": f"writer.write_i8(self.{field_id})?;",
        "i16": f"writer.write_i16(self.{field_id})?;",
        "i32": f"writer.write_i32(self.{field_id})?;",
    }
    print(templates[field["type"]])
print("        Ok(())")
print("    }")
print("}")
