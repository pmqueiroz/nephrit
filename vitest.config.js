import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    coverage: {
      all: false,
      thresholds: {
        100: true,
      },
      provider: 'v8',
      include: ['src/**/*.t{s,sx}'],
      exclude: ['**/*.{types,spec,test}.t{s,sx}', '**/assets/*'],
    },
  },
})
