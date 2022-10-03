import { execaSync } from "execa"
import { join } from "path"
import strip from "strip-ansi"

type CreateLogsMatcherOutput = {
  should: {
    contain: (match: string) => void
  }
}

const createLogsMatcher = (output: string): CreateLogsMatcherOutput => {
  return {
    should: {
      contain: match => {
        expect(output).toMatch(new RegExp(match.replace(/\s+/g, `\\s+`)))
      },
    },
  }
}

const cliBinLocation = join(
  __dirname,
  `bin.js`
)

type InvokeCliResult = {
  exitCode: number
  stdout: string
  logs: CreateLogsMatcherOutput
}

export function invokeCli(args: Array<string>): InvokeCliResult {
  try {
    const results = execaSync(
      process.execPath,
      [cliBinLocation].concat(args),
      {
        cwd: __dirname,
      }
    )
    return {
      exitCode: results.exitCode,
      stdout: strip(results.stdout.toString()),
      logs: createLogsMatcher(strip(results.stdout.toString()))
    }
  } catch (err) {
    return {
      exitCode: err.exitCode,
      stdout: strip(err.stdout.toString()),
      logs: createLogsMatcher(strip(err.stdout?.toString() || ``))
    }
  }
}