import { quansync, type QuansyncFn } from 'quansync'
import { readAsync, readSync, readHashAsync, readHashSync } from 'cacache-napi'

/**
 * Defines the 'read' constant, which is a quansync instance for handling both synchronous and asynchronous reading operations.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { read } from "cacache-napi/read"
 *
 * read.sync(path/to/directory, "key");
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { read } from "cacache-napi/read"
 *
 * read.async(path/to/directory, "key");
 * ```
 */
export const read: QuansyncFn<number[], [cache: string, key: string]> = quansync({
  sync: readSync,
  async: readAsync,
})

/**
 * Defines the'readHash' constant, which is a quansync instance for handling both synchronous and asynchronous reading operations.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { readHash } from "cacache-napi/readHash"
 *
 * readHash.sync(path/to/directory, "sri");
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { readHash } from "cacache-napi/readHash"
 *
 * await readHash.async(path/to/directory, "sri");
 * ```
 */
export const readHash: QuansyncFn<number[], [cache: string, sri: string]> = quansync({
  sync: readHashSync,
  async: readHashAsync,
})
