module.exports = {
  content: ["./index.html", "./src/**/*"],
  theme: {
    extend: {
      colors: {
        red: {
          bool: "#DC2626",
          event: "#C20000",
        },
        pink: {
          string: "#DA5697",
        },
        blue: {
          exec: "#2163EB",
        },
        green: {
          pure: "#008E62",
        },
        gray: {
          graph: "#262626",
          base: "#696969",
        },
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
