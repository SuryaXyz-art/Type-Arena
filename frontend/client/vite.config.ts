import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@linera/client': path.resolve(__dirname, 'node_modules/@linera/client/dist/index.js'),
      '@linera/metamask': path.resolve(__dirname, 'node_modules/@linera/metamask/dist/index.js'),
    },
  },
  server: {
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin',
    },
  },
  esbuild: {
    supported: {
      'top-level-await': true,
    },
  },
});