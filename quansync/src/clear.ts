import { quansync, type QuansyncFn } from 'quansync'
import { clearAsync, clearSync } from 'cacache-napi'

/**
 * Defines the 'clear' constant, which is a quansync instance for handling both synchronous and asynchronous clearing operations.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { clear } from 'cacache-napi/clear'
 *
 * clear.clearSync('path/to/directory')
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { clear } from 'cacache-napi/clear'
 *
 * await clear.clearAsync('path/to/directory')
 * ```
 */
export const clear: QuansyncFn<void, [cache: string]> = quansync({
  sync: clearSync,
  async: clearAsync,
})
