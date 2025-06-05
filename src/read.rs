use cacache::Integrity;
use napi::Error;
use napi_derive::napi;

#[napi]
/// read a file from the cache
/// @param cache The cache directory
/// @param key The key of the file to read
/// @returns The file contents
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::read_sync('/path/to/cache','sha512-abcdef')
/// ```
pub fn read_sync(cache: String, key: String) -> Result<Vec<u8>, Error> {
  cacache::read_sync(cache, key).map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// read a file from the cache
/// @param cache The cache directory
/// @param key The key of the file to read
/// @returns The file contents
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::read_hash_sync('/path/to/cache','sha512-abcdef')
/// ```
pub fn read_hash_sync(cache: String, sri: String) -> Result<Vec<u8>, Error> {
  let sri = sri
    .parse::<Integrity>()
    .map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::read_hash_sync(cache, &sri).map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// read a file from the cache asynchronously
/// @param cache The cache directory
/// @param key The key of the file to read
/// @returns The file contents
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::read_async('/path/to/cache','sha512-abcdef')
/// ```
pub async fn read_async(cache: String, key: String) -> Result<Vec<u8>, Error> {
  cacache::read(cache, key)
    .await
    .map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// read a file from the cache asynchronously
/// @param cache The cache directory
/// @param key The key of the file to read
/// @returns The file contents
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::read_hash_async('/path/to/cache','sha512-abcdef').await
/// ```
pub async fn read_hash_async(cache: String, sri: String) -> Result<Vec<u8>, Error> {
  let sri = sri
    .parse::<Integrity>()
    .map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::read_hash(cache, &sri)
    .await
    .map_err(|err| Error::from_reason(err.to_string()))
}
