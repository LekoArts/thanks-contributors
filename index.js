require('dotenv').config()

const yargs = require('yargs')
const fs = require('fs-extra')
const { Octokit } = require('@octokit/rest')

if (!process.env.GITHUB_ACCESS_TOKEN) {
  throw new Error('GITHUB_ACCESS_TOKEN env var not set')
}

const octokit = new Octokit({
  auth: process.env.GITHUB_ACCESS_TOKEN,
})

// eslint-disable-next-line prefer-destructuring
const argv = yargs
  .usage('Usage: $0 <base> <head> [owner] [repo]')
  .command(
    '* <base> <head> [owner] [repo]',
    'First it get\'s the list of commits between base...head (equivalent to git log base..head), then parses their authors out and creates a markdown list of each contributor and their contribution. By default it excludes the members of the (owner) organization. Saves the result into an "output" folder.',
    (commandBuilder) => commandBuilder
      .positional('base', { type: 'string', demandOption: true })
      .positional('head', { type: 'string', demandOption: true })
      .positional('owner', { type: 'string', default: 'gatsbyjs', demandOption: false })
      .positional('repo', { type: 'string', default: 'gatsby', demandOption: false }),
  )
  .help('h')
  .alias('h', 'help')
  .option('useListCommitsAPI', {
    alias: 'l',
    type: 'boolean',
    description: 'Use the "List commits" API (https://docs.github.com/en/free-pro-team@latest/rest/reference/repos#list-commits) instead of the "Compare two commits" API (https://docs.github.com/en/free-pro-team@latest/rest/reference/repos#compare-two-commits)',
    default: false,
    demandOption: false,
  })
  .option('include', {
    alias: 'i',
    type: 'boolean',
    description: 'Whether to include organization members from the list or not',
    default: false,
    demandOption: false,
  })
  .argv

function getDate(commit) {
  if (
    commit &&
    commit.data &&
    commit.data.commit &&
    commit.data.commit.author &&
    commit.data.commit.author.date
  ) {
    return commit.data.commit.author.date
  }
  return false
}

function groupByKey(list, key) {
  return list.reduce(
    (hash, obj) => ({
      ...hash,
      [obj[key]]: (hash[obj[key]] || []).concat(obj),
    }),
    {},
  )
}

async function run() {
  // Used for the filename
  const currentDate = new Date().toISOString().slice(0, 19)

  let relevantCommits = []

  if (argv.useListCommitsAPI) {
    // Necessary for > 250 commits between two points in time

    const startCommit = await octokit.repos.getCommit({
      owner: argv.owner,
      repo: argv.repo,
      ref: argv.base,
    })
    const endCommit = await octokit.repos.getCommit({
      owner: argv.owner,
      repo: argv.repo,
      ref: argv.head,
    })

    // Getting the dates is sometimes a bit inaccurate, maybe find a better solution?
    const startDate = getDate(startCommit)
    const endDate = getDate(endCommit)

    if (!startDate || !endDate) {
      throw new Error('The function couldn\'t get either the startDate or endDate by checking the commits of the range')
    }

    relevantCommits = await octokit.paginate(octokit.repos.listCommits, {
      owner: argv.owner,
      repo: argv.repo,
      since: startDate,
      until: endDate,
    })
  } else {
    const res = await octokit.repos.compareCommits({
      owner: argv.owner,
      repo: argv.repo,
      base: argv.base,
      head: argv.head,
    })

    relevantCommits = res.data.commits
  }

  if (relevantCommits.length === 0) {
    throw new Error('Couldn\'t find any relevant commits. Are you sure you used the correact head & base, and your excludes are correct?')
  }

  // Some regex, yey
  // /(.*)\(# means: Get everything before the first (# and put it into a group
  // ([0-9]+)\) means: Get every number until the next ) and put it into a group
  // So first group is message, second is number
  const prRegex = /(.*)\(#([0-9]+)\)/

  const entries = relevantCommits.map((c) => {
    const firstLine = c.commit.message.split('\n')[0] // Only get the first line
    const messageAndNumber = firstLine.match(prRegex)

    return {
      author: c.author ? c.author.login : c.commit.author.name,
      authorUrl: c.author ? c.author.html_url : undefined,
      message: messageAndNumber ? messageAndNumber[1].trim() : undefined,
      prNumber: messageAndNumber ? messageAndNumber[2] : undefined,
    }
  })

  let excludes = ['renovate[bot]']

  if (!argv.include) {
    const orgMembers = await octokit.paginate(octokit.orgs.listMembers, {
      org: argv.owner,
    })
    const listOfLoginNames = orgMembers.map((o) => o.login)
    excludes = [...excludes, ...listOfLoginNames]
  }

  const grouped = groupByKey(entries, 'author')
  const filtered = Object.entries(grouped).filter((entry) => !excludes.includes(entry[0]))

  let text = ''

  filtered.forEach((author) => {
    const authorName = author[0]
    const { authorUrl } = author[1][0]
    const authorContent = author[1]
    const hasMultipleEntries = authorContent.length > 1

    if (hasMultipleEntries) {
      const authorEntries = `${authorContent.map((a) => {
        if (!a.message) {
          return ''
        }

        const prLink = a.prNumber ? `[PR #${a.prNumber}](https://github.com/${argv.owner}/${argv.repo}/pull/${a.prNumber})` : ''

        return (
          `  - ${a.message} ${prLink}
`
        )
      }).join('')}
`
      const authorMD = authorUrl ? `[${authorName}](${authorUrl})` : authorName

      text += `
- ${authorMD}
${authorEntries}`
    } else {
      const content = authorContent[0]

      if (!content.message) {
        return
      }
      const authorMD = authorUrl ? `[${authorName}](${authorUrl})` : authorName
      const prLink = content.prNumber ? `[PR #${content.prNumber}](https://github.com/${argv.owner}/${argv.repo}/pull/${content.prNumber})` : ''

      text += `
- ${authorMD}: ${content.message} ${prLink}`
    }
  })

  const filename = `${process.cwd()}/output/${currentDate}.md`
  await fs.outputFile(filename, text)

  console.log(`Successfully created ${filename}`)
}

module.exports.run = run
