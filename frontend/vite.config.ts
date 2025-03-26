import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { fileURLToPath, URL } from 'node:url';

const API_BASE_URL = 'https://the-elder-scrobz.hoohoot.org';

export default defineConfig({
	plugins: [vue()],
	resolve: {
		alias: {
			'@': fileURLToPath(new URL('./src', import.meta.url)),
		},
	},
	server: {
		proxy: {
			'/api/v1': {
				target: API_BASE_URL,
				changeOrigin: true,
				secure: false,
			},
		},
	},
});
