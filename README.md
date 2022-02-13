# Thanks Contributors!

This little script accesses GitHub's API to get all contributors and their PRs between two distinct points in the history of commits. This is helpful for changelogs where you'd want to list all contributions for that release (so e.g. changes between v1 and v1.1).

## Usage

```shell
npx thanks-contributors <base> <head> [owner] [repo]
```

First it get's the list of commits between `base...head` (equivalent to `git log
base..head`), then parses their authors and creates a markdown list of each
contributor and their contribution. By default it excludes the members of the
(owner) organization. Saves the result into an "output" folder.

```shell
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

## Development

### Build

After `yarn build/npm run build` command, you can see `package-template.[darwin|win32|linux].node` file in project root. This is the native addon built from [lib.rs](./src/lib.rs).

### CI

With GitHub actions, every commits and pull request will be built and tested automatically in [`node@12`, `node@14`, `@node16`] x [`macOS`, `Linux`, `Windows`] matrix. You will never be afraid of the native addon broken in these platforms.

### Release

Release native package is very difficult in old days. Native packages may ask developers who use its to install `build toolchain` like `gcc/llvm` , `node-gyp` or something more.

With `GitHub actions`, we can easily prebuild `binary` for major platforms. And with `N-API`, we should never afraid of **ABI Compatible**.

The other problem is how to deliver prebuild `binary` to users. Download it in `postinstall` script is a common way which most packages do it right now. The problem of this solution is it introduced many other packages to download binary which has not been used by `runtime codes`. The other problem is some user may not easily download the binary from `GitHub/CDN` if they are behind private network (But in most case, they have a private NPM mirror).

In this package we choose a better way to solve this problem. We release different `npm packages` for different platform. And add it to `optionalDependencies` before release the `Major` package to npm.

`NPM` will choose which native package should download from `registry` automatically. You can see [npm](./npm) dir for details. And you can also run `yarn add @napi-rs/package-template` to see how it works.

### Develop requirements

- Install latest `Rust`
- Install `Node.js@10+` which fully supported `Node-API`
- Install `yarn@1.x`

### Test in local

- yarn
- yarn build

### Release package

Ensure you have set you **NPM_TOKEN** in `GitHub` project setting.

In `Settings -> Secrets`, add **NPM_TOKEN** into it.

When you want release package:

```
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.
