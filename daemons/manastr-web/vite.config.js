import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  root: 'public',
  build: {
    outDir: '../dist',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: path.resolve(__dirname, 'public/index.html')
      }
    }
  },
  server: {
    port: 8080,
    host: '0.0.0.0',
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
      '/src': path.resolve(__dirname, 'src')
    },
  },
  optimizeDeps: {
    include: [
      '@cashu/cashu-ts', 
      '@nostr-dev-kit/ndk',
      'react',
      'react-dom',
      'styled-components'
    ]
  }
});