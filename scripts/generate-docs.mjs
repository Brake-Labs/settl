#!/usr/bin/env node
/**
 * Build script: reads markdown files from docs/ and generates Astro-compatible
 * markdown pages in website/src/pages/docs/ with layout frontmatter.
 *
 * This is the single-source-of-truth approach: docs/ is canonical,
 * and the website pages are derived automatically.
 *
 * Generated pages are .md files with `layout: ../../layouts/Docs.astro`
 * frontmatter, matching the agent-of-empires pattern.
 */

import { readdir, readFile, writeFile, mkdir } from "node:fs/promises";
import { join, basename, extname } from "node:path";

const DOCS_DIR = join(import.meta.dirname, "..", "docs");
const OUTPUT_DIR = join(
  import.meta.dirname,
  "..",
  "website",
  "src",
  "pages",
  "docs"
);

async function main() {
  await mkdir(OUTPUT_DIR, { recursive: true });

  const files = (await readdir(DOCS_DIR)).filter((f) => extname(f) === ".md");

  for (const file of files) {
    const content = await readFile(join(DOCS_DIR, file), "utf-8");

    // Replace the source frontmatter with Astro layout frontmatter.
    // Keep title and description, add layout reference.
    const match = content.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/);
    if (!match) {
      console.warn(`  Skipping ${file}: no frontmatter found`);
      continue;
    }

    const frontmatter = {};
    for (const line of match[1].split("\n")) {
      const idx = line.indexOf(":");
      if (idx > 0) {
        const key = line.slice(0, idx).trim();
        const value = line.slice(idx + 1).trim();
        frontmatter[key] = value;
      }
    }

    const body = match[2];
    const title = frontmatter.title || basename(file, ".md");
    const description =
      frontmatter.description || `settl documentation: ${title}`;
    const slug = basename(file, ".md");

    const astroFrontmatter = `---
layout: ../../layouts/Docs.astro
title: "${title}"
description: "${description}"
---`;

    const output = `${astroFrontmatter}\n${body}`;

    await writeFile(join(OUTPUT_DIR, `${slug}.md`), output);
    console.log(`  Generated: docs/${slug}.md`);
  }

  console.log(`Generated ${files.length} doc pages.`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
