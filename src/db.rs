use std::path::Path;

use bincode::{self, Decode, Encode};
pub(crate) use rocksdb::DB;
use rocksdb::{self, BlockBasedOptions, Cache, Options};
pub(crate) type Key = [u8; 8];

pub(crate) fn open_with_cache<P: AsRef<Path>>(
    path: P,
    reset_db: bool,
) -> Result<DB, rocksdb::Error> {
    if reset_db && path.as_ref().exists() {
        std::fs::remove_dir_all(&path).unwrap();
    }

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

pub(crate) fn put_entity<T>(db: &DB, entity: &T)
where
    T: Entity,
{
    let key = entity.get_key();
    let bytes = bincode::encode_to_vec(entity, bincode::config::standard()).unwrap();
    db.put(&key, &bytes).unwrap();
}
