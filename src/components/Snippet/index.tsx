import React from "react";
import CodeBlock from "@theme/CodeBlock";

interface SnippetProps extends React.ComponentProps<typeof CodeBlock> {
  /**
   * A metadata string for specifying lines to include, e.g., "1-3,5,8-10".
   */
  lines?: string;
  omitted_placeholder?: string;
  strip_leading_spaces?: boolean;
}

/**
 * A component for rendering a snippet of code, optionally filtering lines,
 * showing ellipses for omissions, and stripping all leading spaces.
 */
const Snippet: React.FC<SnippetProps> = ({
  children,
  lines,
  omitted_placeholder = "...",
  strip_leading_spaces = false,
  ...props
}) => {
  if (typeof children !== "string") {
    console.error(
      "Snippet expects children to be a string containing the file content."
    );
    return null;
  }

  // Parse the `linesToInclude` metadata string into an array of line numbers.
  const parseLineRanges = (metaString?: string): number[] => {
    if (!metaString) return [];
    return metaString.split(",").flatMap((range) => {
      const [start, end] = range.split("-").map(Number);
      if (!end) return [start]; // Single line
      return Array.from({ length: end - start + 1 }, (_, i) => start + i); // Range
    });
  };

  const includedLines = parseLineRanges(lines);

  // Extract the lines to include and insert "..." for omissions.
  const formatContent = (content: string, lines: number[]): string => {
    const allLines = content.split("\n");
    if (lines.length === 0) return content; // If no specific lines are specified, return full content.

    const includedContent: string[] = [];
    lines.forEach((line, index) => {
      if (index > 0 && lines[index - 1] < line - 1) {
        includedContent.push(omitted_placeholder); // Add placeholder for omitted lines
      }

      const rawLine = allLines[line - 1] || "";
      const formattedLine = strip_leading_spaces
        ? rawLine.trimStart()
        : rawLine;
      includedContent.push(formattedLine);
    });

    // Add placeholder if lines at the end are omitted
    if (lines[lines.length - 1] < allLines.length) {
      includedContent.push(omitted_placeholder);
    }

    return includedContent.join("\n");
  };

  const formattedContent = formatContent(children, includedLines);

  return <CodeBlock {...props}>{formattedContent}</CodeBlock>;
};

export default Snippet;
