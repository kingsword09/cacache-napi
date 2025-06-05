use cacache::Integrity;
use napi::Error;
use napi_derive::napi;

#[napi]
/// remove a file from the cache
/// @param cache The cache directory
/// @param key The key of the file to remove
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::remove_sync('/path/to/cache','sha512-abcdef')
/// ```
pub fn remove_sync(cache: String, key: String) -> Result<(), Error> {
  cacache::remove_sync(cache, key).map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// remove a file from the cache
/// @param cache The cache directory
/// @param key The key of the file to remove
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::remove_sync('/path/to/cache','sha512-abcdef')
/// ```
pub fn remove_hash_sync(cache: String, sri: String) -> Result<(), Error> {
  let sri = sri
    .parse::<Integrity>()
    .map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::remove_hash_sync(cache, &sri).map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// remove a file from the cache asynchronously
/// @param cache The cache directory
/// @param key The key of the file to remove
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::remove_async('/path/to/cache','sha512-abcdef').await
/// ```
pub async fn remove_async(cache: String, key: String) -> Result<(), Error> {
  cacache::remove(cache, key)
    .await
    .map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// remove a file from the cache asynchronously
/// @param cache The cache directory
/// @param key The key of the file to remove
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::remove_hash_async('/path/to/cache','sha512-abcdef').await
/// ```
pub async fn remove_hash_async(cache: String, sri: String) -> Result<(), Error> {
  let sri = sri
    .parse::<Integrity>()
    .map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::remove_hash(cache, &sri)
    .await
    .map_err(|err| Error::from_reason(err.to_string()))
}
