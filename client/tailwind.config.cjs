/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,jsx,ts,tsx}"],
  theme: {
    extend: {
      backgroundColor: {
        primary: "rgb(0,0,0)",
        secondary: "#1f2937",
      },
      borderColor: {
        primary: "rgb(2, 14, 34)",
      }
    }
  },
  plugins: []
};
