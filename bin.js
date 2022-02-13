#!/usr/bin/env node

const cli = require("./index")
const arguments = process.argv.slice(2)

cli.run(arguments)
  .then((code) => {
    if (code !== 0) {
      process.exit(code)
    }
  })
  .catch((e) => {
    console.error(e)
    process.exit(1)
  })