/** @type {import('tailwindcss').Config} */
export default {
  content: ["./templates/**/*.{js,html,hbs,json}"],
  theme: {
    extend: {
      colors: {
        primary: "#c60558",
        "primary-content": "#fecce2",
        "primary-dark": "#940442",
        "primary-light": "#f8066e",

        secondary: "#b8c605",
        "secondary-content": "#000000",
        "secondary-dark": "#8a9404",
        "secondary-light": "#e6f806",

        background: "#f2edef",
        foreground: "#fcfbfb",
        border: "#e5dade",

        copy: "#2d2025",
        "copy-light": "#785464",
        "copy-lighter": "#a07889",

        success: "#05c605",
        warning: "#c6c605",
        error: "#c60505",

        "success-content": "#ccfecc",
        "warning-content": "#000000",
        "error-content": "#fecccc",
      },
    },
  },
  plugins: [],
};
// https://www.hover.dev/css-color-palette-generator