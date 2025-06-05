use crate::metadata;

use metadata::Metadata;
use napi_derive::napi;

#[napi]
/// List all the files in the cache.
///
/// @param cache - The cache directory.
/// @returns A list of files in the cache.
///
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::list_sync("cache");
/// ```
pub fn list_sync(cache: String) -> Vec<Metadata> {
  let list = cacache::list_sync(cache);

  let mut metadata_list = vec![];
  for metadata in list {
    if let Ok(metadata) = metadata {
      let metadata = Metadata {
        key: metadata.key,
        integrity: metadata.integrity.to_string(),
        size: metadata.size.to_string(),
        metadata: serde_json::to_string(&metadata.metadata).unwrap_or_default(),
        time: metadata.time.into(),
        raw_metadata: metadata.raw_metadata,
      };

      metadata_list.push(metadata);
    };
  }

  metadata_list
}
