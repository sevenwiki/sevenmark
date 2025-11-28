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
          text: "Text Formatting",
          items: [
            { text: "Text Styles", link: "/grammar/text-styles" },
            { text: "Headers", link: "/grammar/headers" },
            { text: "Horizontal Line", link: "/grammar/horizontal-line" },
            { text: "Comments", link: "/grammar/comments" },
            { text: "Escape Characters", link: "/grammar/escape" },
          ],
        },
        {
          text: "Block Elements",
          items: [
            { text: "Blockquotes", link: "/grammar/blockquote" },
            { text: "Code Blocks", link: "/grammar/code" },
            { text: "Fold (Collapsible)", link: "/grammar/fold" },
            { text: "Lists", link: "/grammar/lists" },
            { text: "Tables", link: "/grammar/tables" },
          ],
        },
        {
          text: "Advanced Elements",
          items: [
            { text: "TeX Math", link: "/grammar/tex" },
            { text: "Literal Blocks", link: "/grammar/literal" },
            { text: "Ruby Text", link: "/grammar/ruby" },
            { text: "Footnotes", link: "/grammar/footnote" },
            { text: "Styled Elements", link: "/grammar/styled" },
          ],
        },
        {
          text: "Media & Macros",
          items: [
            { text: "Media", link: "/grammar/media" },
            { text: "Macros", link: "/grammar/macros" },
            { text: "Conditionals", link: "/grammar/conditionals" },
            { text: "Parameters", link: "/grammar/parameters" },
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
