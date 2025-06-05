use napi::bindgen_prelude::BigInt;
use napi_derive::napi;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct Metadata {
  /// Key this entry is stored under.
  pub key: String,
  /// Integrity hash for the stored data. Acts as a key into {cache}/content.
  pub integrity: String,
  /// Timestamp in unix milliseconds when this entry was written.
  pub time: BigInt,
  /// Size of data associated with this entry.
  pub size: String,
  /// Arbitrary JSON  associated with this entry.
  pub metadata: String,
  /// Raw metadata in binary form. Can be different from JSON metadata.
  pub raw_metadata: Option<Vec<u8>>,
}

#[napi]
/// @param cache The cache directory
/// @param key The key of the entry to get
/// @returns Metadata
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
/// 
/// cacache::metadata_sync('/path/to/cache','abcdef')
/// ```
pub fn metadata_sync(cache: String, key: String) -> Option<Metadata> {
  if let Ok(Some(metadata)) = cacache::metadata_sync(cache, key) {
    let metadata = Metadata {
      key: metadata.key,
      integrity: metadata.integrity.to_string(),
      time: metadata.time.into(),
      size: metadata.size.to_string(),
      metadata: serde_json::to_string(&metadata.metadata).unwrap_or_default(),
      raw_metadata: metadata.raw_metadata,
    };

    Some(metadata)
  } else {
    None
  }
}

#[napi]
/// @param cache The cache directory asynchronously.
/// @param key The key of the entry to get
/// @returns Metadata
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::metadata_async('/path/to/cache','abcdef').await
/// ```
pub async fn metadata_async(cache: String, key: String) -> Option<Metadata> {
  if let Ok(Some(metadata)) = cacache::metadata(cache, key).await {
    let metadata = Metadata {
      key: metadata.key,
      integrity: metadata.integrity.to_string(),
      time: metadata.time.into(),
      size: metadata.size.to_string(),
      metadata: serde_json::to_string(&metadata.metadata).unwrap_or_default(),
      raw_metadata: metadata.raw_metadata,
    };
    Some(metadata)
  } else {
    None
  }
}
