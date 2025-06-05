import { quansync, type QuansyncFn } from 'quansync'
import { writeAsync, writeSync, type WriteOptions } from 'cacache-napi'
export type { WriteOptions } from 'cacache-napi'

/**
 * Defines the 'write' constant, which is a quansync instance for handling both synchronous and asynchronous writing operations.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { write } from "cacache-napi/write"
 *
 * write.sync(path/to/directory, Buffer.from("content"), { algo: "sha512", key: "key" });
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { write } from "cacache-napi/write"
 *
 * await write.async(path/to/directory, Buffer.from("content"), { algo: "sha512", key: "key" });
 * ```
 */
export const write: QuansyncFn<
  string,
  [cache: string, data: Uint8Array<ArrayBufferLike>, ops?: WriteOptions | null | undefined]
> = quansync({
  sync: writeSync,
  async: writeAsync,
})
