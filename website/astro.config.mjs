import { defineConfig } from "astro/config";
import tailwind from "@astrojs/tailwind";
import sitemap from "@astrojs/sitemap";

export default defineConfig({
  site: "https://mozilla-ai.github.io",
  base: "/settl",
  integrations: [tailwind(), sitemap({ changefreq: "weekly" })],
});
