import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    allowedHosts: [
      'redacted.invalid',
      '.redacted.invalid'
    ]
  },
  proxy: {
    '/api': {
			target: '0.0.0.0:0000',
			changeOrigin: true,
			secure: false,
    }
  }
})
