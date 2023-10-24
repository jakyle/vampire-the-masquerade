import type { Config } from 'tailwindcss';
import typography from '@tailwindcss/typography';
import forms from '@tailwindcss/forms';
import plugin from 'tailwindcss/plugin';

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
	plugins: [
		typography,
		forms,
		plugin(({ addVariant, matchUtilities, theme }) => {
			addVariant('hocus', ['&:hover', '&:focus']);
			// Square utility
			matchUtilities(
				{
					square: (value) => ({
						width: value,
						height: value
					})
				},
				{
					values: theme('spacing')
				}
			);
		})
	]
} satisfies Config;
