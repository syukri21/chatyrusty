import { resolve } from 'path'

export default {
  root: resolve(__dirname, 'src'),
  build: {
    outDir: '../assets'
  },
  server: {
    port: 8080
  }
}
