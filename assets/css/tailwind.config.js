const plugin = require("tailwindcss/plugin");

module.exports = {
  content: ["./layouts/**/*.html", "./content/**/*.html"],
  mode: "jit",
  theme: {
    fontFamily: {
      serif:
        '"Iowan Old Style", Iowan, Garamond, Palatino, "Times New Roman", serif',
      mono: '"SF Mono", Consolas, monospace',
    },

    colors: {
      "background-hard": "var(--background-hard)",
      background: "var(--background)",
      "background-soft": "var(--background-soft)",
      "background-1": "var(--background-1)",
      "background-2": "var(--background-2)",
      "background-3": "var(--background-3)",
      "background-4": "var(--background-4)",

      foreground: "var(--foreground)",
      "foreground-1": "var(--foreground-1)",
      "foreground-2": "var(--foreground-2)",
      "foreground-3": "var(--foreground-3)",
      "foreground-4": "var(--foreground-4)",

      red: "var(--red)",
      green: "var(--green)",
      yellow: "var(--yellow)",
      blue: "var(--blue)",
      purple: "var(--purple)",
      aqua: "var(--aqua)",
      orange: "var(--orange)",
      gray: "var(--gray)",

      "red-dim": "var(--red-dim)",
      "green-dim": "var(--green-dim)",
      "yellow-dim": "var(--yellow-dim)",
      "blue-dim": "var(--blue-dim)",
      "purple-dim": "var(--purple-dim)",
      "aqua-dim": "var(--aqua-dim)",
      "orange-dim": "var(--orange-dim)",
      "gray-dim": "var(--gray-dim)",
    },
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("@tailwindcss/aspect-ratio"),
    require("@tailwindcss/line-clamp"),

    // Expose color palette as CSS variables (--color-xxx-yyy)
    // https://gist.github.com/Merott/d2a19b32db07565e94f10d13d11a8574
    plugin(function ({ addBase, theme }) {
      function extractColorVars(colorObj, colorGroup = "") {
        return Object.keys(colorObj).reduce((vars, colorKey) => {
          const value = colorObj[colorKey];

          const newVars =
            typeof value === "string"
              ? { [`--color${colorGroup}-${colorKey}`]: value }
              : extractColorVars(value, `-${colorKey}`);

          return { ...vars, ...newVars };
        }, {});
      }

      addBase({
        ":root": extractColorVars(theme("colors")),
      });
    }),
  ],
};
