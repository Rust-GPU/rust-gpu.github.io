import fs from 'fs/promises';
import path from 'path';
import { LoadContext, Plugin } from '@docusaurus/types';

const CHANGELOG_URL = 'https://raw.githubusercontent.com/Rust-GPU/rust-gpu/main/CHANGELOG.md';

const OUTPUT_PATH = path.resolve(__dirname, '../../pages/changelog.md');

const FRONTMATTER = `---
id: changelog
title: Changelog
sidebar_label: Changelog
---\n\n`;

// Fetch the changelog content from GitHub using fetch
async function fetchChangelog(): Promise<string> {
  const response = await fetch(CHANGELOG_URL);
  if (!response.ok) {
    throw new Error(`Failed to fetch changelog: ${response.statusText}`);
  }
  return await response.text();
}

// Write the changelog to the docs folder with frontmatter
async function writeChangelog(content: string) {
  const changelogWithFrontmatter = `${FRONTMATTER}${content}`;
  
  // Ensure the directory exists
  await fs.mkdir(path.dirname(OUTPUT_PATH), { recursive: true });
  
  // Write the file
  await fs.writeFile(OUTPUT_PATH, changelogWithFrontmatter, 'utf8');
}

// Docusaurus plugin definition
export default function fetchChangelogPlugin(context: LoadContext): Plugin<void> {
  return {
    name: 'fetch-changelog-plugin',

    async loadContent() {
      console.log('Fetching changelog from GitHub...');
      const changelogContent = await fetchChangelog();
      await writeChangelog(changelogContent);
      console.log('Changelog successfully written to pages/changelog.md');
    },
  };
}

