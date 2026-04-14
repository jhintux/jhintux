module.exports = {
  content: ["./src/**/*.{rs,html,css}", "./assets/**/*.css"],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"DM Sans"', "ui-sans-serif", "system-ui", "sans-serif"],
        mono: ['"DM Mono"', "ui-monospace", "monospace"],
      },
    },
  },
  plugins: [],
};
