/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './renderer/**/*.{js,ts,jsx,tsx}',
    './pages/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        fg: '#CCCCCC',
        bg: '#0d0d0d',
        primary: '#CF3704',
        green: '#00FF00'
      },
      animation: {
        appear: 'appear 400ms ease-out'
      },
      keyframes: {
        appear: {
          '0%': { transform: 'translate(100%)' },
          '100%': { transform: 'translate(0%)' },
        }
      }
    },
  },
  plugins: [],
}
