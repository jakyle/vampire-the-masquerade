/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			gridTemplateRows: {
				9: 'repeat(9, minmax(0, 1fr))'
			}
		}
	},
	plugins: []
};
