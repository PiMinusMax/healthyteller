import { expect, test } from "@playwright/test";
import { acceptNotice } from "./helpers";

test("web app can be installed: manifest and icons", async ({ page, request }) => {
  await page.goto("/");
  const href = await page.locator('link[rel="manifest"]').getAttribute("href");
  expect(href).toBeTruthy();
  const manifest = await (await request.get(href!)).json();
  expect(manifest.name).toBe("Healthy Teller");
  expect(manifest.display).toBe("standalone");
  for (const icon of manifest.icons) {
    const res = await request.get(new URL(icon.src, new URL(href!, page.url())).toString());
    expect(res.status(), icon.src).toBe(200);
  }
});

test("app starts offline after the first visit", async ({ page, context, browserName }) => {
  test.skip(browserName !== "chromium", "Service workers in Playwright are reliable in Chromium only");
  await acceptNotice(page);
  await page.evaluate(async () => {
    await navigator.serviceWorker.ready;
  });
  // A second load lets the service worker cache the hashed app files.
  await page.reload();
  await expect(page.getByRole("link", { name: /Lebensmittel prüfen/ })).toBeVisible();
  await context.setOffline(true);
  await page.reload();
  await expect(page.getByRole("link", { name: /Lebensmittel prüfen/ })).toBeVisible();
  await page.getByRole("link", { name: /Teller ansehen/ }).click();
  await expect(page.getByRole("heading", { level: 1, name: "Heute" })).toBeVisible();
  await context.setOffline(false);
});
