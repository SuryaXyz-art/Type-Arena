/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        emerald: {
          light: '#34d399',
          DEFAULT: '#10b981',
          dark: '#059669',
        },
        ruby: {
          DEFAULT: '#f43f5e',
          dark: '#e11d48',
        }
      }
    },
  },
  plugins: [],
}
