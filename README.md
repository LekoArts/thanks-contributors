# Thanks Contributors!

This little script accesses GitHub's API to get all contributors and their PRs between two distinct points in the history of commits. This is helpful for changelogs where you'd want to list all contributions for that release (so e.g. changes between v1 and v1.1).

## Usage

```shell
npx @lekoarts/thanks-contributors [OPTIONS] <BASE> <HEAD> [OWNER] [REPO]
```

First, it get's the list of commits between `base...head` (equivalent to `git log
base..head`), then parses their authors and creates a markdown list of each
contributor and their contribution.

```shell
Usage: @lekoarts/thanks-contributors [OPTIONS] <BASE> <HEAD> [OWNER] [REPO]

Arguments:
<BASE>
        Pointer from where to start looking for changes

<HEAD>
        Pointer until where to stop looking for changes

[OWNER]
        Name of the owner/user/organization of the repository

        [default: gatsbyjs]

[REPO]
        Name of the repository

        [default: gatsby]

Options:
-i, --include-org-members <INCLUDE_ORG_MEMBERS>
        Include organization members into the list [default: false]

        [possible values: true, false]

-e, --excludes <EXCLUDES>...
        List of members to exclude from the list. Usage: -e=member1,member2 [default:
        \\"renovate-bot\\", \\"renovate[bot]\\"]

-h, --help
        Print help information (use \`-h\` for a summary)
```

You must have an environment variable called `GITHUB_ACCESS_TOKEN` either exported in your CLI or defined inside an `.env` file in the directory you're running the CLI in.

The script automatically excludes the members of the organization ("owner" in this case). If you want to exclude specific users, you'll need to provide the `--excludes` flag.

The results are stored inside a `output` folder in the current directory.

### Example

```shell
npx @lekoarts/thanks-contributors gatsby@5.1.0-next.0 gatsby@5.1.0 gatsbyjs gatsby
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
