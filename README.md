# Thanks Contributors!

This little script accesses GitHub's API to get all contributors and their PRs between two distinct points in the history of commits. This is helpful for changelogs where you'd want to list all contributions for that release (so e.g. changes between v1 and v1.1).

## Usage

```shell
npx thanks-contributors <base> <head> [owner] [repo]

First it get's the list of commits between base...head (equivalent to git log
base..head), then parses their authors out and creates a markdown list of each
contributor and their contribution. By default it excludes the members of the
(owner) organization. Saves the result into an "output" folder.

Positionals:
  base                                                                  [string]
  head                                                                  [string]
  owner                                          [string] [Standard: "gatsbyjs"]
  repo                                             [string] [Standard: "gatsby"]

Optionen:
      --version            Show version                                [boolean]
  -h, --help               Show help                                   [boolean]
  -l, --useListCommitsAPI  Use the "List commits" API
                           (https://docs.github.com/en/free-pro-team@latest/rest
                           /reference/repos#list-commits) instead of the
                           "Compare two commits" API
                           (https://docs.github.com/en/free-pro-team@latest/rest
                           /reference/repos#compare-two-commits)
                                                     [boolean] [Standard: false]
  -i, --include            Whether to include organization members from the list
                           or not                    [boolean] [Standard: false]
```

You must have an environment variable called `GITHUB_ACCESS_TOKEN` either exported in your CLI or defined inside an `.env` file in the root of the project.

The script automatically excludes the members of the organization ("owner" in this case). If you want to exclude specific users, you'll need to edit the `index.js` (at the moment).

The results are stored inside a `./output` folder.

### Example

```shell
npx thanks-contributors gatsby@2.29.0-next.0 gatsby@2.29.0 gatsbyjs gatsby
```

## Resulting output

It'll create a markdown list, grouped by user login. If a person had more than one commit, it creates a nested list. Example:

```md
- [harry](https://www.github.com/harry): Update something [PR #1](https://github.com/foobar/pull/1)
- [hermione](https://www.github.com/hermione)
  - Update something [PR #2](https://github.com/foobar/pull/2)
  - Update something more [PR #3](https://github.com/foobar/pull/3)
```

If the url can't be found only the name will be printed.

## Caveats

- Getting the PR number only works if you consistently add the number in the commit itself, e.g. in `feat: My cool feature (#123)`. This automatically happens in GitHub's UI if you use squash commits.
- The `excludes` currently can't be defined manually via the CLI
