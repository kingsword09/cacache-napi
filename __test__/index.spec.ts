import test from 'ava'

import { listSync } from '../index.js'

test('list_sync', (t) => {
  t.is(listSync('.').length, 0)
})
