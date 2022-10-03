import { invokeCli } from "../vitest.utils"

describe(`cli`, () => {
  it(`should show help command`, () => {
    const { exitCode, stdout } = invokeCli([`--help`])

    expect(stdout).toMatchInlineSnapshot(`
      "This little script accesses GitHub's API to get all contributors and their PRs between two distinct
      points in the history of commits. This is helpful for changelogs where you'd want to list all
      contributions for that release (so e.g. changes between v1 and v1.1).

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
                Print help information (use \`-h\` for a summary)"
    `)
    expect(exitCode).toBe(0)
  })
})