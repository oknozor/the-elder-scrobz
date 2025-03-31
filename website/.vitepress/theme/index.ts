// https://vitepress.dev/guide/custom-theme
import { h } from "vue";
import type { Theme } from "vitepress";
import DefaultTheme from "vitepress/theme";
import "./style.css";
import { theme, useOpenapi, useTheme } from "vitepress-openapi/client";
import spec from "../../api-docs/openapi.json";
import "vitepress-openapi/dist/style.css";

export default {
  extends: DefaultTheme,
  Layout: () => {
    return h(DefaultTheme.Layout, null, {
      // https://vitepress.dev/guide/extending-default-theme#layout-slots
    });
  },
  enhanceApp({ app, router, siteData }) {
    const openapi = useOpenapi({
      spec,
      config: {},
    });

    useTheme({
      operation: {
        hiddenSlots: ["playground"],
        cols: 1,
      },
    });

    theme.enhanceApp({ app, openapi });
  },
} satisfies Theme;
