import type { PrismTheme } from "prism-react-renderer";

export const githubDark: PrismTheme = {
  plain: {
    color: "#d4d4d4",
    backgroundColor: "#000000",
  },
  styles: [
    {
      types: ["comment", "prolog", "doctype", "cdata"],
      style: {
        color: "#6a737d",
      },
    },
    {
      types: ["punctuation"],
      style: {
        color: "#d1d5da",
      },
    },
    {
      types: ["namespace"],
      style: {
        opacity: 0.7,
      },
    },
    {
      types: ["property", "tag", "boolean", "number", "constant", "symbol", "deleted"],
      style: {
        color: "#e06c75",
      },
    },
    {
      types: ["selector", "attr-name", "string", "char", "builtin", "inserted"],
      style: {
        color: "#98c379",
      },
    },
    {
      types: ["operator", "entity", "url"],
      style: {
        color: "#d1d5da",
      },
    },
    {
      types: ["atrule", "attr-value", "keyword"],
      style: {
        color: "#c678dd",
      },
    },
    {
      types: ["function", "class-name"],
      style: {
        color: "#61afef",
      },
    },
    {
      types: ["regex", "important", "variable"],
      style: {
        color: "#e5c07b",
      },
    },
    {
      types: ["important", "bold"],
      style: {
        fontWeight: "bold",
      },
    },
    {
      types: ["italic"],
      style: {
        fontStyle: "italic",
      },
    },
    {
      types: ["entity"],
      style: {
        cursor: "help",
      },
    },
  ],
};
