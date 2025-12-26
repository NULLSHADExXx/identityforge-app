/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'forge-dark': '#0f0f0f',
        'forge-darker': '#080808',
        'forge-accent': '#6366f1',
        'forge-accent-hover': '#818cf8',
        'forge-success': '#22c55e',
        'forge-warning': '#f59e0b',
        'forge-danger': '#ef4444',
      }
    },
  },
  plugins: [],
}
