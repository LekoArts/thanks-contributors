#!/usr/bin/env node
const { run } = require('./index')

run().catch((e) => {
  // eslint-disable-next-line no-console
  console.warn(e)
})
