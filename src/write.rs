use cacache::Algorithm;
use napi::Error;
use napi_derive::napi;
use std::str::FromStr;

fn write_sync_internal(cache: &str, key: &str, data: Vec<u8>) -> Result<String, Error> {
  cacache::write_sync(cache, key, data)
    .map(|integrity| integrity.to_string())
    .map_err(|err| Error::from_reason(err.to_string()))
}

fn write_hash_sync(cache: &str, data: Vec<u8>) -> Result<String, Error> {
  cacache::write_hash_sync(cache, data)
    .map(|integrity| integrity.to_string())
    .map_err(|err| Error::from_reason(err.to_string()))
}

fn write_sync_with_algo(
  algo: &str,
  cache: &str,
  key: &str,
  data: Vec<u8>,
) -> Result<String, Error> {
  let algo = Algorithm::from_str(algo).map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::write_sync_with_algo(algo, cache, key, data)
    .map(|integrity| integrity.to_string())
    .map_err(|err| Error::from_reason(err.to_string()))
}

fn wirte_hash_sync_with_algo(algo: &str, cache: &str, data: Vec<u8>) -> Result<String, Error> {
  let algo = Algorithm::from_str(algo).map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::write_hash_sync_with_algo(algo, cache, data)
    .map(|integrity| integrity.to_string())
    .map_err(|err| Error::from_reason(err.to_string()))
}

async fn write_async_internal(cache: &str, key: &str, data: Vec<u8>) -> Result<String, Error> {
  cacache::write(cache, key, data)
    .await
    .map(|integrity| integrity.to_string())
    .map_err(|err| Error::from_reason(err.to_string()))
}

async fn write_hash_async(cache: &str, data: Vec<u8>) -> Result<String, Error> {
  cacache::write_hash(cache, data)
    .await
    .map(|integrity| integrity.to_string())
    .map_err(|err| Error::from_reason(err.to_string()))
}

async fn write_async_with_algo(
  algo: &str,
  cache: &str,
  key: &str,
  data: Vec<u8>,
) -> Result<String, Error> {
  let algo = Algorithm::from_str(algo).map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::write_with_algo(algo, cache, key, data)
    .await
    .map(|integrity| integrity.to_string())
    .map_err(|err| Error::from_reason(err.to_string()))
}

async fn write_hash_async_with_algo(
  algo: &str,
  cache: &str,
  data: Vec<u8>,
) -> Result<String, Error> {
  let algo = Algorithm::from_str(algo).map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::write_hash_with_algo(algo, cache, data)
    .await
    .map(|integrity| integrity.to_string())
    .map_err(|err| Error::from_reason(err.to_string()))
}

#[napi(object)]
pub struct WriteOptions {
  pub algo: Option<String>,
  pub key: Option<String>,
}

impl Default for WriteOptions {
  fn default() -> Self {
    Self {
      algo: None,
      key: None,
    }
  }
}

impl WriteOptions {
  pub fn new(algo: Option<String>, key: Option<String>) -> Self {
    Self {
      algo: algo.map(String::from),
      key: key.map(String::from),
    }
  }
}

#[napi]
/// write data to cacache
/// @param cache The cacache directory
/// @param data The data to write
/// @param ops The options
/// @returns The integrity of the data
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::write_sync('/path/to/cache', vec![], {algo: 'sha512', key: 'hello world'})
/// ```
pub fn write_sync(cache: String, data: &[u8], ops: Option<WriteOptions>) -> Result<String, Error> {
  match ops {
    Some(ops) => match (ops.algo, ops.key) {
      (Some(algo), Some(key)) => write_sync_with_algo(&algo, &cache, &key, data.to_vec()),
      (Some(algo), None) => wirte_hash_sync_with_algo(&algo, &cache, data.to_vec()),
      (None, Some(key)) => write_sync_internal(&cache, &key, data.to_vec()),
      (None, None) => write_hash_sync(&cache, data.to_vec()),
    },
    None => write_sync_internal(&cache, "", data.to_vec()),
  }
}

#[napi]
/// write data to cacache asynchronously
/// @param cache The cacache directory
/// @param data The data to write
/// @param ops The options
/// @returns The integrity of the data
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
/// 
/// cacache::write_async('/path/to/cache', vec![], {algo:'sha512', key: 'hello world'}).await
/// ```
pub async fn write_async(
  cache: String,
  data: &[u8],
  ops: Option<WriteOptions>,
) -> Result<String, Error> {
  match ops {
    Some(WriteOptions { algo, key }) => match (algo, key) {
      (Some(algo), Some(key)) => write_async_with_algo(&algo, &cache, &key, data.to_vec()).await,
      (Some(algo), None) => write_hash_async_with_algo(&algo, &cache, data.to_vec()).await,
      (None, Some(key)) => write_async_internal(&cache, &key, data.to_vec()).await,
      (None, None) => write_hash_async(&cache, data.to_vec()).await,
    },
    None => write_async_internal(&cache, "", data.to_vec()).await,
  }
}
