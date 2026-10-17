import { expect, test } from "@playwright/test";
import { acceptNotice } from "./helpers";

test("first start shows the basic notice and needs a confirmation", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("html")).toHaveAttribute("lang", "de");
  await expect(page).toHaveTitle("Healthy Teller");
  await expect(page.getByRole("heading", { level: 1, name: "Willkommen bei Healthy Teller" })).toBeVisible();
  await expect(page.locator('[data-notice="basic"]')).toContainText(
    "Die App stellt keine Diagnose und ersetzt keine ärztliche Beratung.",
  );
  const next = page.getByRole("button", { name: "Weiter" });
  await expect(next).toBeDisabled();
  await page.getByLabel("Ich habe den Hinweis gelesen und verstanden.").check();
  await next.click();
  await expect(page.getByRole("link", { name: /Lebensmittel prüfen/ })).toBeVisible();
  await expect(page.getByRole("link", { name: /Teller ansehen/ })).toBeVisible();
});

test("the confirmation is remembered", async ({ page }) => {
  await acceptNotice(page);
  await page.reload();
  await expect(page.getByRole("link", { name: /Lebensmittel prüfen/ })).toBeVisible();
});

test("the basic notice stays available under About", async ({ page }) => {
  await acceptNotice(page);
  await page.getByRole("link", { name: "Einstellungen" }).click();
  await page.getByRole("link", { name: "Über die App und Grundhinweis" }).click();
  await expect(page.locator('[data-notice="basic"]')).toBeVisible();
});

test("language can be switched to English and back", async ({ page }) => {
  await acceptNotice(page);
  await page.getByRole("link", { name: "Einstellungen" }).click();
  await page.getByLabel("English").check();
  await expect(page.getByRole("heading", { level: 1, name: "Settings" })).toBeVisible();
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  await page.goto("/");
  await expect(page.getByRole("link", { name: /Check food/ })).toBeVisible();
  await page.getByRole("link", { name: "Settings" }).click();
  await page.getByLabel("Deutsch").check();
  await expect(page.getByRole("heading", { level: 1, name: "Einstellungen" })).toBeVisible();
  await expect(page.locator("html")).toHaveAttribute("lang", "de");
});
