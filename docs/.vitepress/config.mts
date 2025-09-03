import { defineConfig } from "vitepress";

export default defineConfig({
  base: "/sevenmark/",
  title: "SevenMark",
  description: "A DSL designed for SevenWiki",

  themeConfig: {
    nav: [
      { text: "Home", link: "/" },
      { text: "Grammar", link: "/grammar/text-styles" },
      { text: "Examples", link: "/examples/basic" },
      { text: "API", link: "/api/" },
    ],

    sidebar: {
      "/grammar/": [
        {
          text: "Basic Grammar",
          items: [
            { text: "Text Styles", link: "/grammar/text-styles" },
            { text: "Block Elements", link: "/grammar/blocks" },
            { text: "Lists", link: "/grammar/lists" },
            { text: "Tables", link: "/grammar/tables" },
          ],
        },
        {
          text: "Advanced Grammar",
          items: [
            { text: "Parameters", link: "/grammar/parameters" },
            { text: "Styling", link: "/grammar/styling" },
            { text: "Macros", link: "/grammar/macros" },
          ],
        },
        {
          text: "Wiki Features",
          items: [
            { text: "Include", link: "/grammar/include" },
            { text: "Category", link: "/grammar/category" },
            { text: "Redirect", link: "/grammar/redirect" },
          ],
        },
      ],
      "/examples/": [
        {
          text: "Examples",
          items: [
            { text: "Basic Examples", link: "/examples/basic" },
            { text: "Complex Structures", link: "/examples/complex" },
            { text: "Real World Usage", link: "/examples/real-world" },
          ],
        },
      ],
      "/api/": [
        {
          text: "API",
          items: [
            { text: "Parser API", link: "/api/parser" },
            { text: "AST Structure", link: "/api/ast" },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/sevenwiki/sevenmark" },
    ],

    search: {
      provider: "local",
    },
  },
});
