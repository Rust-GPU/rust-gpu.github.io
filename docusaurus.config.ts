import { githubDark } from "./src/theme/prism";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";
import fetchAuthorImages from "./src/plugins/fetchAuthorImages";
import fetchChangelogPlugin from "./src/plugins/fetchChangelog";

const config: Config = {
  title: "Rust GPU",
  tagline: "The future of GPU programming",
  // favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: "https://rust-gpu.github.io/",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: "/",

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "rust-gpu", // Usually your GitHub org/user name.
  projectName: "https://rust-gpu.github.io/", // Usually your repo name.

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  plugins: [
    fetchAuthorImages,
    fetchChangelogPlugin,
    "@gracefullight/docusaurus-plugin-tailwind",
  ],
  presets: [
    [
      "classic",
      {
        // For now just link out
        /*
        docs: {
          sidebarPath: "./sidebars.ts",
          editUrl: "https://github.com/Rust-GPU/rust-gpu.github.io/tree/main/",
        },
        */
        blog: {
          showReadingTime: true,
          feedOptions: {
            type: ["rss", "atom"],
            xslt: true,
          },
          editUrl: "https://github.com/Rust-GPU/rust-gpu.github.io/tree/main/",
          onInlineTags: "throw",
          onInlineAuthors: "throw",
          onUntruncatedBlogPosts: "throw",
        },
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    // image: 'img/docusaurus-social-card.jpg',
    navbar: {
      title: "Rust GPU",
      // logo: {
      //   alt: 'Rust GPU Logo',
      //   src: 'img/logo.svg',
      // },
      items: [
        { href: "https://rust-gpu.github.io/rust-gpu/book/", label: "Docs", position: "right" },
        { to: "/blog", label: "Blog", position: "right" },
        { to: "/ecosystem", label: "Ecosystem", position: "right" },
        { to: "/changelog", label: "Changelog", position: "right" },
        {
          href: "https://github.com/rust-gpu/rust-gpu",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    prism: {
      theme: githubDark,
      darkTheme: githubDark,
      defaultLanguage: "rust",
    },
    colorMode: {
      disableSwitch: true,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
