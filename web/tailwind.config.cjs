module.exports = {
	content: {
		files: ['*.html', './src/**/*.ts', './src/**/*.svelte']
	},
	theme: {
		colors: {
			bg: '#fcecbc',
			fg: '#fafbff',
			mid: '#f3d584',
			contrast: '#04152b',
			attention: '#a519ff',
			info: '#09488a',
			error: '#971d00',

			color: '#04152b',
			'color-inverted': '#fafbff'
		},
		extend: {
			fontFamily: {
				noto: ['Noto Serif', 'sans-serif']
			},
			fontWeight: {
				thin: '300',
				regular: '400',
				bold: '600',
				black: '800'
			},
			fontSize: {
				xs: '0.6rem',
				s: '0.8rem',
				m: '1rem',
				l: '1.2rem',
				xl: '1.5rem',
				'2xl': '2.0rem'
			}
		}
	},
	plugins: []
};
