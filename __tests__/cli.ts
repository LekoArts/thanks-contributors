import { invokeCli } from "../vitest.utils"

describe(`cli`, () => {
	it(`should show help command`, () => {
		const { exitCode, stdout } = invokeCli([`--help`])

		expect(stdout).toMatchSnapshot()
		expect(exitCode).toBe(0)
	})
	it(`should error without required arguments`, () => {
		const { exitCode, stdout } = invokeCli([])

		expect(stdout).toMatchSnapshot()
		expect(exitCode).toBe(2)
	})
  it(`should error on invalid commits range`, () => {
		const { exitCode, stdout } = invokeCli([`100.0.1`, `100.0.2`])

		expect(stdout).toMatchSnapshot()
		expect(exitCode).toBe(1)
	})
})
