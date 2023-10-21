import typescript from '@rollup/plugin-typescript';

export default {
  input: 'src/index.ts',
  output: {
    file: 'dist/wp.js',
    format: 'iife'
  },
  plugins: [typescript()],
};