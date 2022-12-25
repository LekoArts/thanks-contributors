# Contributing

## Development

### Build

After `yarn build/npm run build` command, you can see `thanks-contributors.[darwin|win32|linux].node` file in project root. This is the native addon built from [lib.rs](./src/lib.rs).

### CI

With GitHub actions, every commits and pull request will be built and tested automatically in [`node@14`, `@node16`] x [`macOS`, `Linux`, `Windows`] matrix. You will never be afraid of the native addon broken in these platforms.

### Release

Release native package is very difficult in old days. Native packages may ask developers who use its to install `build toolchain` like `gcc/llvm` , `node-gyp` or something more.

With `GitHub actions`, we can easily prebuild `binary` for major platforms. And with `N-API`, we should never afraid of **ABI Compatible**.

The other problem is how to deliver prebuild `binary` to users. Download it in `postinstall` script is a common way which most packages do it right now. The problem of this solution is it introduced many other packages to download binary which has not been used by `runtime codes`. The other problem is some user may not easily download the binary from `GitHub/CDN` if they are behind private network (But in most case, they have a private NPM mirror).

In this package we choose a better way to solve this problem. We release different `npm packages` for different platform. And add it to `optionalDependencies` before release the `Major` package to npm.

`NPM` will choose which native package should download from `registry` automatically. You can see [npm](./npm) dir for details. And you can also run `yarn add @napi-rs/thanks-contributors` to see how it works.

### Develop requirements

- Install latest `Rust`
- Install `Node.js@14+` which fully supported `Node-API`
- Install `yarn@1.x`

### Test in local

- `yarn`
- `yarn build:debug`
- `node bin.js --help`

### Release package

Ensure you have set you **NPM_TOKEN** in `GitHub` project setting.

In `Settings -> Secrets`, add **NPM_TOKEN** into it.

When you want release package:

```
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.
