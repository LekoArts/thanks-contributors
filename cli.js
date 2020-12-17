#!/usr/bin/env node
const { run } = require('./index')

run().catch((e) => {
  console.warn(e)
})
