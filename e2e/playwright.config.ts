import { defineConfig, devices } from "@playwright/test";

// Output of `dx build --platform web --release` in crates/app.
const webRoot = "../target/dx/healthyteller-app/release/web/public";
const port = 4173;

export default defineConfig({
  testDir: "./tests",
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? [["github"], ["html", { open: "never" }]] : "list",
  use: { baseURL: `http://127.0.0.1:${port}` },
  webServer: {
    command: `npx http-server ${webRoot} -p ${port} -a 127.0.0.1 -c-1 --silent`,
    url: `http://127.0.0.1:${port}`,
    reuseExistingServer: !process.env.CI,
  },
  projects: [
    { name: "chromium", use: { ...devices["Desktop Chrome"] } },
    { name: "firefox", use: { ...devices["Desktop Firefox"] } },
    { name: "webkit", use: { ...devices["Desktop Safari"] } },
  ],
});
