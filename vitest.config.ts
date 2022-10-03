import { defineConfig } from "vitest/config"

export default defineConfig({
  test: {
    globals: true,
    include: [`__tests__/*.ts`],
    coverage: {
      reporter: [`text`, `json`, `html`],
    },
  },
})