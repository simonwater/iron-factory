use byteorder::{LittleEndian, ReadBytesExt};
use crc::crc32;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, SeekFrom};
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
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        None
    }

    pub fn insert(&mut self, key: &str, value: &str) -> bool {
        true
    }

    pub fn update(&mut self, key: &str, value: &str) -> bool {
        true
    }

    pub fn delete(&mut self, key: &str) -> bool {
        true
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
