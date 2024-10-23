import config from '@rocketseat/eslint-config/react.mjs'
import simpleImportSort from 'eslint-plugin-simple-import-sort'
import vitest from 'eslint-plugin-vitest'

export default [
  ...config,
  {
    files: ['tests/**/*'],
    plugins: {
      ...config.plugins,
      'simple-import-sort': simpleImportSort,
      vitest,
    },
    rules: {
      ...config.rules,
      ...vitest.configs.recommended.rules,
      'simple-import-sort/imports': 'error',
      '@stylistic/jsx-closing-bracket-location': 'error',
      '@stylistic/jsx-closing-tag-location': 'error',
      '@stylistic/jsx-first-prop-new-line': ['error', 'multiprop'],
      '@stylistic/jsx-function-call-newline': ['error', 'always'],
      '@stylistic/semi': ['error', 'always'],
      '@stylistic/jsx-quotes': ['error', 'prefer-double'],
      '@stylistic/quotes': ['error', 'double'],
      '@stylistic/jsx-max-props-per-line': ['error', {
        maximum: 1,
      }],
      '@stylistic/max-len': ['warn', 100],
      '@typescript-eslint/no-unused-vars': [
        'warn', { argsIgnorePattern: '^_', varsIgnorePattern: '^_' },
      ],
      'no-unused-vars': ['warn', {
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
      }],
    },
    settings: {
      vitest: {
        typecheck: true,
      },
    },
    languageOptions: {
      globals: {
        ...vitest.environments.env.globals,
      },
    },
  },
]
