/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,js,tsx,mdx}",
    "./blog/**/*.{html,js,tsx,mdx}",
    "./docs/**/*.{html,js,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        "primary-text": "#24292e",
        "primary-text-over": "#0056b3",
        "primary-link": "#0366d6",
        secondary: "#f6f8fa",
        "secondary-text": "#586069",
        "toc-highlight-text": "#a66cdc",
        "toc-background": "#ffffff",
        code: "#24292e",
        "code-background": "#eeeeee",
        shadow: "rgba(27, 31, 35, 0.15)",
        "border-color": "#e1e4e8",
        "focus-border": "#0366d6",
      },
      fontFamily: {
        header: ['"Geist"', "sans-serif"],
        text: ['"Geist"', "sans-serif"],
        mono: ['"GeistMono"', "sans-serif"],
      },
      borderWidth: {
        thin: "max(1px, 0.0625rem)",
      },
      boxShadow: {
        normal: "inset 0 -1px 0 rgba(209, 217, 224, 0.7)",
        "link-hover": "inset 0 -1px 0 rgba(209, 217, 224, 0.7)",
      },
    },
  },
};
