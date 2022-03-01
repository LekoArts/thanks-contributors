#!/usr/bin/env node

const cli = require("./index")
const arguments = process.argv.slice(2)

cli.run(arguments)
  .catch((e) => {
    console.error(e)
    process.exit(1)
  })