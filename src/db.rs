use std::path::Path;

use bincode::{self, Decode, Encode};
pub(crate) use rocksdb::DB;
use rocksdb::{self, BlockBasedOptions, Cache, Options};
pub(crate) type Key = [u8; 8];

pub(crate) fn open_with_cache<P: AsRef<Path>>(path: P) -> Result<DB, rocksdb::Error> {
    let cache = Cache::new_lru_cache(1024 * 1024 * 1024 * 12); // 12GiB
    let mut table_options = BlockBasedOptions::default();
    table_options.set_block_cache(&cache);
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_block_based_table_factory(&table_options);

    DB::open(&opts, path)
}

pub(crate) trait Entity: Encode + Decode {
    fn get_key(&self) -> Key;
}

#[derive(Debug)]
pub(crate) enum StructDBError {
    EncodeError(bincode::error::EncodeError),
    DecodeError(bincode::error::DecodeError),
    DBError(rocksdb::Error),
}
impl From<bincode::error::EncodeError> for StructDBError {
    fn from(error: bincode::error::EncodeError) -> Self {
        StructDBError::EncodeError(error)
    }
}
impl From<bincode::error::DecodeError> for StructDBError {
    fn from(error: bincode::error::DecodeError) -> Self {
        StructDBError::DecodeError(error)
    }
}
impl From<rocksdb::Error> for StructDBError {
    fn from(error: rocksdb::Error) -> Self {
        StructDBError::DBError(error)
    }
}

pub(crate) fn get_entity<T>(db: &DB, key: &Key) -> Option<T>
where
    T: Entity,
{
    let Some(bytes) = db.get(key).unwrap() else {
        return None;
    };
    let (entity, _): (T, _) =
        bincode::decode_from_slice(&bytes, bincode::config::standard()).unwrap();
    Some(entity)
}

pub(crate) fn get_entities<'a, T, K, I>(db: &DB, keys: I) -> impl Iterator<Item = Option<T>> + 'a
where
    K: AsRef<[u8]>,
    T: Entity,
    I: IntoIterator<Item = K>,
    <I as IntoIterator>::IntoIter: 'a,
{
    db.multi_get(keys).into_iter().map(|r| -> Option<T> {
        match r.unwrap() {
            Some(b) => {
                let (entity, _) =
                    bincode::decode_from_slice(&b, bincode::config::standard()).unwrap();
                Some(entity)
            }
            None => None,
        }
    })
}

pub(crate) fn put_entity<T>(db: &DB, entity: &T)
where
    T: Entity,
{
    let key = entity.get_key();
    let bytes = bincode::encode_to_vec(entity, bincode::config::standard()).unwrap();
    db.put(&key, &bytes).unwrap();
}
