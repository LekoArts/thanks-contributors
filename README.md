# Thanks Contributors!

This little script accesses GitHub's API to get all contributors and their PRs between two distinct points in the history of commits. This is helpful for changelogs where you'd want to list all contributions for that release (so e.g. changes between v1 and v1.1).

**Please note: This isn't polished at all, I'll most likely create a CLI or GitHub action out of it**

## Usage

- Create a `.env` file with `TOKEN=your-github-token` so that the script can access the GitHub API
- Define a list of GitHub login IDs inside `excludes.json` whose commits should be excluded from the list.
- Define the `owner`, `repo`, `startRef`, and `endRef` inside `config.js`
- Run `node index.js` to let it generate `.md` files inside `./output`

### Resulting output

It'll create a markdown list, grouped by user login. If a person had more than one commit, it creates a nested list. Example:

```md
- [harry](https://www.github.com/harry): Update something [PR #1](https://github.com/foobar/pull/1)
- [hermione](https://www.github.com/hermione)
  - Update something [PR #2](https://github.com/foobar/pull/2)
  - Update something more [PR #3](https://github.com/foobar/pull/3)
```

## Caveats

- Getting the PR number only works if you consistently add the number in the commit itself, e.g. in `feat: My cool feature (#123)`. This automatically happens in GitHub's UI if you use squash commits.
