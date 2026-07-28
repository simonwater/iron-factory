use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

pub struct KVStorage {
    f: File,
    index: HashMap<ByteString, u64>,
}

impl KVStorage {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::with_capacity(256);
        Ok(Self { f, index })
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);
        loop {
            let pos = f.seek(SeekFrom::Current(0))?;
            let maybe_record = Self::process_record(&mut f);
            let record = match maybe_record {
                Ok(rec) => rec,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => break,
                        _ => return Err(err),
                    };
                }
            };
            self.index.insert(record.key, pos);
        }
        Ok(())
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let pos = match self.index.get(key) {
            None => return Ok(None),
            Some(&pos) => pos,
        };
        let kv = self.get_at(pos)?;
        if kv.value.is_empty() {
            return Ok(None);
        }
        Ok(Some(kv.value))
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let pos = self.append(key, value)?;
        self.index.insert(key.to_vec(), pos);
        Ok(())
    }

    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }

    fn append(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let key_len = key.len();
        let val_len = value.len();
        let mut tmp = vec![0; key_len + val_len];
        tmp[0..key_len].copy_from_slice(key);
        tmp[key_len..].copy_from_slice(value);
        let checksum = crc32::checksum_ieee(&tmp);

        let mut f = BufWriter::new(&mut self.f);
        let pos = f.seek(SeekFrom::End(0))?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        f.write_all(&tmp)?;
        f.flush()?;

        Ok(pos)
    }

    fn get_at(&mut self, pos: u64) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&self.f);
        f.seek(SeekFrom::Start(pos))?;
        let kv = Self::process_record(&mut f)?;
        Ok(kv)
    }

    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let val_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);
        f.take(data_len as u64).read_to_end(&mut data)?;
        debug_assert_eq!(data_len as usize, data.len());
        let checksum = crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!(
                "data corruption encountered: {:08x} != {:08x}",
                checksum, saved_checksum
            );
        }

        let value = data.split_off(key_len as usize);
        let key = data;
        Ok(KeyValuePair { key, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let path = std::path::Path::new("abc");
        let mut store = KVStorage::open(path).unwrap();
        store.load().unwrap();

        // insert
        store
            .insert("abc1".as_bytes(), "value11".as_bytes())
            .unwrap();
        store
            .insert("abc2".as_bytes(), "value22".as_bytes())
            .unwrap();
        store
            .insert("abc3".as_bytes(), "value33".as_bytes())
            .unwrap();

        // get
        assert_eq!(
            store.get("abc1".as_bytes()).unwrap(),
            Some("value11".into())
        );
        assert_eq!(
            store.get("abc2".as_bytes()).unwrap(),
            Some("value22".into())
        );
        assert_eq!(
            store.get("abc3".as_bytes()).unwrap(),
            Some("value33".into())
        );

        // update
        store
            .update("abc1".as_bytes(), "value11111".as_bytes())
            .unwrap();
        assert_eq!(
            store.get("abc1".as_bytes()).unwrap(),
            Some("value11111".into())
        );

        // delete
        store.delete("abc1".as_bytes()).unwrap();
        assert_eq!(store.get("abc1".as_bytes()).unwrap(), None);
    }
}
