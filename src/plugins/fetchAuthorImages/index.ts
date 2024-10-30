import fs from "fs/promises";
import path from "path";
import { LoadContext, Plugin } from "@docusaurus/types";
import yaml from "yaml";

// Paths and constants
const AUTHORS_FILE = path.resolve(__dirname, "../../../blog/authors.yml");
const OUTPUT_DIR = path.resolve(__dirname, "../../../static/img/authors");
const IMAGE_COMMENT =
  "# Image URL managed by the fetchAuthorImages plugin. Changes may be overwritten.";

// Fetch GitHub avatar URL
async function fetchGitHubAvatar(username: string): Promise<string | null> {
  const url = `https://api.github.com/users/${username}`;
  const response = await fetch(url);

  if (!response.ok) {
    console.error(
      `Failed to fetch data for ${username}: ${response.statusText}`
    );
    return null;
  }

  const data = await response.json();
  return data.avatar_url;
}

// Download and save image locally
async function downloadImage(url: string, filepath: string) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to download image from ${url}`);
  }
  const buffer = await response.arrayBuffer();
  await fs.writeFile(filepath, Buffer.from(buffer));
}

// Ensure directory exists
async function ensureDirectory(dir: string) {
  await fs.mkdir(dir, { recursive: true });
}

// Main function to process authors
async function processAuthors() {
  const file = await fs.readFile(AUTHORS_FILE, "utf8");
  const authors = yaml.parseDocument(file);

  await ensureDirectory(OUTPUT_DIR);

  const authorItems = (authors.contents as yaml.YAMLMap).items;

  for (const author of authorItems) {
    const authorValue = author.value as yaml.YAMLMap;
    const authorData = authorValue.toJSON();
    const username = authorData.socials?.github;

    if (username) {
      const avatarUrl = await fetchGitHubAvatar(username);
      if (avatarUrl) {
        const localImagePath = `/img/authors/${username}.png`;
        const localPath = path.join(OUTPUT_DIR, `${username}.png`);

        // Download the image and save it locally
        try {
          await downloadImage(avatarUrl, localPath);
          console.log(`Downloaded image for ${username} to ${localPath}`);
        } catch (error) {
          console.error(`Failed to download image for ${username}: ${error}`);
          continue;
        }

        // Create a YAML scalar node for the image URL with a comment
        const imageNode = new yaml.Scalar(localImagePath);
        imageNode.commentBefore = IMAGE_COMMENT;

        authorValue.set("image_url", imageNode);
      }
    }
  }

  await fs.writeFile(AUTHORS_FILE, authors.toString(), "utf8");
}

// Docusaurus plugin definition
export default function fetchAuthorImagesPlugin(
  context: LoadContext
): Plugin<void> {
  return {
    name: "fetch-author-images-plugin",

    async loadContent() {
      console.log("Fetching author images...");
      await processAuthors();
      console.log("Author images fetched and authors.yml updated.");
    },
  };
}
