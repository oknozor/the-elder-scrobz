import { defineConfig, loadEnv } from 'vite';
import vue from '@vitejs/plugin-vue';
import { fileURLToPath, URL } from 'node:url';

export default ({ mode }) => {
	process.env = { ...process.env, ...loadEnv(mode, process.cwd()) };

	return defineConfig({
		plugins: [vue()],
		resolve: {
			alias: {
				'@': fileURLToPath(new URL('./src', import.meta.url)),
			},
		},
		server: {
			proxy: {
				'/api/v1': {
					target: process.env.VITE_API_BASE_URL || '/',
					secure: true,
					changeOrigin: true,
				},
			},
			port: 5173,
			host: '0.0.0.0',
		},
	});
};
