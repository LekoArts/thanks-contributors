require('dotenv').config()
const fs = require('fs-extra')
const { Octokit } = require('@octokit/rest')
const {
  owner, repo, base, head,
} = require('./config')

const octokit = new Octokit({
  auth: process.env.TOKEN,
})

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

async function getExcludes(pathName) {
  let list = []
  try {
    const result = await fs.readJSON(pathName)
    list = result
  } catch (error) {
    // eslint-disable-next-line no-console
    console.warn(error)
  }

  return list
}

async function run({ useCompareAPI = false }) {
  const currentDate = new Date().toISOString()

  let relevantCommits = []

  if (!useCompareAPI) {
    const startCommit = await octokit.repos.getCommit({
      owner,
      repo,
      ref: base,
    })
    const endCommit = await octokit.repos.getCommit({
      owner,
      repo,
      ref: head,
    })

    const startDate = getDate(startCommit)
    const endDate = getDate(endCommit)

    if (!startDate || !endDate) {
      throw new Error('The function couldn\'t get either the startDate or endDate by checking the commits of the range')
    }

    relevantCommits = await octokit.paginate(octokit.repos.listCommits, {
      owner,
      repo,
      since: startDate,
      until: endDate,
    })
  } else {
    const res = await octokit.repos.compareCommits({
      owner,
      repo,
      base,
      head,
    })

    relevantCommits = res.data.commits
  }

  const prRegex = /(.*)\(#([0-9]+)\)/

  const entries = relevantCommits.map((c) => {
    const firstLine = c.commit.message.split('\n')[0] // Only get the first line
    const messageAndNumber = firstLine.match(prRegex)

    return {
      author: c.author.login,
      authorUrl: c.author.html_url,
      message: messageAndNumber ? messageAndNumber[1].trim() : undefined,
      prNumber: messageAndNumber ? messageAndNumber[2] : undefined,
    }
  })

  const excludes = await getExcludes('./excludes.json')

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

        const prLink = a.prNumber ? `[PR #${a.prNumber}](https://github.com/gatsbyjs/gatsby/pull/${a.prNumber})` : ''

        return (
          `  - ${a.message} ${prLink}
`
        )
      }).join('')}
`
      text += `
- [${authorName}](${authorUrl})
${authorEntries}`
    } else {
      const content = authorContent[0]
      if (!content.message) {
        return
      }
      const prLink = content.prNumber ? `[PR #${content.prNumber}](https://github.com/gatsbyjs/gatsby/pull/${content.prNumber})` : ''
      text += `
- [${authorName}](${authorUrl}): ${content.message} ${prLink}`
    }
  })

  await fs.outputFile(`${__dirname}/output/${currentDate}.md`, text)
}

run({ useCompareAPI: true })
