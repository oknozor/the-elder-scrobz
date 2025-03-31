import { defineConfig } from "vitepress";
import { useSidebar } from "vitepress-openapi";
import spec from "../api-docs/openapi.json" with { type: "json" };

const sidebar = useSidebar({ spec });

export default defineConfig({
  title: "The Elder Scrobz",
  description:
    "A music tracking application that helps you monitor and analyze your listening habits.",
  base: "/the-elder-scrobz/",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "API docs", link: "/api-docs" },
      { text: "Getting Started", link: "/guide/" },
    ],

    sidebar: {
      "/guide": [
        {
          items: [{ text: "Getting Started" }],
        },
      ],
      "/api-docs": [
        ...sidebar.generateSidebarGroups({
          linkPrefix: "/operations/",
        }),
      ],
    },
    search: {
      provider: "local",
      options: {
        detailedView: true,
      },
    },
    editLink: {
      pattern:
        "https://github.com/oknozor/the-elder-scrobz/edit/main/website/:path",
    },
    socialLinks: [
      { icon: "github", link: "https://github.com/oknozor/the-elder-scrobz" },
    ],
  },
});
