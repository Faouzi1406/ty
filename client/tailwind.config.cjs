/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,jsx,ts,tsx}"],
  theme: {
    extend: {
      backgroundColor: {
        primary: "#333333",
        secondary: "#333333",
        sec: "#0f0f0f"
      },
      borderColor: {
        primary: "rgb(2, 14, 34)",
      }
    }
  },
  plugins: []
};
