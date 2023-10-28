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
			},
			spacing: {
				'5.5': '1.375rem',
				'128': '32rem'
			},

			minWidth: {
				'1/4': '25%',
				'1/2': '50%',
				'3/4': '75%',
				'5.5': '1.375rem',
				'6': '1.5rem',
				'7': '1.75rem',
				'8': '2rem'
			},
			maxHeight: {
				'1/4': '25%',
				'1/2': '50%',
				'3/4': '75%',
				'5.5': '1.375rem',
				'6': '1.5rem',
				'7': '1.75rem',
				'8': '2rem'
			},
		},
		data: {
			circle: 'shape~="circle"',
			square: 'shape~="square"',
			horizontal: 'direction~="horizontal"',
			vertical: 'direction~="vertical"'
		}
	},
	plugins: [
		typography,
		forms,
		plugin(({ addVariant, matchUtilities, theme }) => {
			addVariant('hocus', ['&:hover', '&:focus']);
			matchUtilities(
				{
					sq: (value) => ({
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
