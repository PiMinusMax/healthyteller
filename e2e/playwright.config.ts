import { defineConfig, devices } from "@playwright/test";
import * as path from "path";

// Output of `dx build --platform web --release` in crates/app.
const webRoot = "../target/dx/healthyteller-app/release/web/public";
const port = 4173;

export default defineConfig({
  testDir: "./tests",
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? [["github"], ["html", { open: "never" }]] : "list",
  use: {
    baseURL: `http://127.0.0.1:${port}`,
    // With a service worker in control, WebKit does not pass requests to page.route,
    // so the Open Food Facts fixtures would not apply. Only the offline test allows it.
    serviceWorkers: "block",
  },
  webServer: {
    // -P: unknown paths fall back to index.html, so deep links work like in the app.
    command: `npx http-server ${webRoot} -p ${port} -a 127.0.0.1 -c-1 --silent -P http://127.0.0.1:${port}?`,
    url: `http://127.0.0.1:${port}`,
    reuseExistingServer: !process.env.CI,
  },
  projects: [
    { name: "chromium", use: { ...devices["Desktop Chrome"] }, testIgnore: /scan\.spec\.ts/ },
    { name: "firefox", use: { ...devices["Desktop Firefox"] }, testIgnore: /scan\.spec\.ts/ },
    { name: "webkit", use: { ...devices["Desktop Safari"] }, testIgnore: /scan\.spec\.ts/ },
    {
      // Only Chromium can play a video file as camera picture.
      name: "chromium-camera",
      testMatch: /scan\.spec\.ts/,
      use: {
        ...devices["Desktop Chrome"],
        permissions: ["camera"],
        launchOptions: {
          args: [
            "--use-fake-ui-for-media-stream",
            "--use-fake-device-for-media-stream",
            `--use-file-for-fake-video-capture=${path.join(__dirname, "fixtures/barcode-4000000000013.y4m")}`,
          ],
        },
      },
    },
  ],
});
