module.exports = {
  env: {
    commonjs: true,
    es2021: true,
    node: true,
  },
  extends: [
    'airbnb-base',
  ],
  parserOptions: {
    ecmaVersion: 12,
  },
  rules: {
    semi: 0,
    'operator-linebreak': 0,
    'import/prefer-default-export': 0,
    'no-console': 0,
  },
};
