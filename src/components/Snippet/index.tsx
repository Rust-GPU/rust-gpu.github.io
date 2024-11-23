import React, { useEffect, useState } from "react";
import CodeBlock from "@theme/CodeBlock";

interface SnippetProps extends React.ComponentProps<typeof CodeBlock> {
  /**
   * A metadata string for specifying lines to include, e.g., "1-3,5,8-10".
   */
  lines?: string;
  omitted_placeholder?: string;
  strip_leading_spaces?: boolean;
  /**
   * Optional short hash of the content (first N characters of SHA-256),
   * required only when `lines` is specified.
   */
  hash?: string;
}

/**
 * A component for rendering a snippet of code, optionally filtering lines,
 * showing ellipses for omissions, stripping leading spaces, and validating hash.
 */
const Snippet: React.FC<SnippetProps> = ({
  children,
  lines,
  omitted_placeholder = "...",
  strip_leading_spaces = false,
  hash,
  ...props
}) => {
  const [error, setError] = useState<string | null>(null);

  if (typeof children !== "string") {
    throw new Error(
      "Snippet expects children to be a string containing the file content."
    );
  }

  /**
   * Utility function to compute the SHA-256 hash of a string.
   * @param content The input string
   * @returns Promise resolving to a hex-encoded hash
   */
  const computeHash = async (content: string): Promise<string> => {
    const encoder = new TextEncoder();
    const data = encoder.encode(content);
    const hashBuffer = await crypto.subtle.digest("SHA-256", data);
    return Array.from(new Uint8Array(hashBuffer))
      .map((byte) => byte.toString(16).padStart(2, "0"))
      .join("");
  };

  useEffect(() => {
    if (lines) {
      computeHash(children).then((computedHash) => {
        const shortHash = computedHash.slice(0, 7); // Use 7 characters for the short hash

        if (!hash) {
          setError(
            `The \`hash\` prop is required when \`lines\` is specified.\n` +
              `Provide the following hash as the \`hash\` prop: ${shortHash}`
          );
        } else if (shortHash !== hash) {
          setError(
            `Snippet hash mismatch.\n` +
              `Specified: ${hash}, but content is: ${shortHash} (full hash: ${computedHash}).\n` +
              `Check if the line numbers are still relevant and update the hash.`
          );
        }
      });
    }
  }, [children, lines, hash]);

  if (error) {
    throw new Error(error);
  }

  // Parse the `lines` metadata string into an array of line numbers.
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

    // Filter lines and find the minimum indentation
    const selectedLines = lines
      .map((line) => allLines[line - 1] || "")
      .filter((line) => line.trim().length > 0); // Ignore blank lines

    const minIndent = selectedLines.reduce((min, line) => {
      const indentMatch = line.match(/^(\s*)\S/);
      const indentLength = indentMatch ? indentMatch[1].length : 0;
      return Math.min(min, indentLength);
    }, Infinity);

    lines.forEach((line, index) => {
      const rawLine = allLines[line - 1] || "";
      const trimmedLine =
        rawLine.trim().length > 0 ? rawLine.slice(minIndent) : rawLine;

      if (index > 0 && lines[index - 1] < line - 1) {
        // Add placeholder for omitted lines only if within range
        includedContent.push(omitted_placeholder);
      }

      includedContent.push(trimmedLine);
    });

    return includedContent.join("\n");
  };

  const formattedContent = formatContent(children, includedLines);

  return <CodeBlock {...props}>{formattedContent}</CodeBlock>;
};

export default Snippet;
