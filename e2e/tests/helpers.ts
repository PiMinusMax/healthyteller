import { Page, expect } from "@playwright/test";
import * as fs from "fs";
import * as path from "path";

/** Product fixture with invented values (fixtures/open-food-facts). */
export const FIXTURE_BARCODE = "4000000000017";
export const FIXTURE_NAME = "Testjoghurt natur";

export const FIXTURES = path.join(__dirname, "../../fixtures/open-food-facts");

/** Answers every Open Food Facts request with a fixture, so tests never hit the network. */
export async function mockOpenFoodFacts(page: Page, file = "product-found.json") {
  const product = fs.readFileSync(path.join(FIXTURES, file), "utf8");
  const search = fs.readFileSync(path.join(FIXTURES, "search-results.json"), "utf8");
  await page.route("https://world.openfoodfacts.org/**", (route) =>
    route.fulfill({
      status: 200,
      contentType: "application/json",
      headers: { "access-control-allow-origin": "*" },
      body: route.request().url().includes("/cgi/search.pl") ? search : product,
    }),
  );
}

/** First start: confirm the basic notice. */
export async function acceptNotice(page: Page) {
  await page.goto("/");
  await page.getByLabel("Ich habe den Hinweis gelesen und verstanden.").check();
  await page.getByRole("button", { name: "Weiter" }).click();
  await expect(page.getByRole("link", { name: /Lebensmittel prüfen/ })).toBeVisible();
}

/** Invented test profile (AGENTS.md rule 18). */
export async function createProfile(
  page: Page,
  options: { conditions?: string[]; allergies?: string[]; saltLimit?: string } = {},
) {
  await page.getByRole("link", { name: "Profil anlegen" }).click();
  await page.getByLabel("Name", { exact: true }).fill("Test Person");
  await page.getByRole("button", { name: "Weiter" }).click();
  for (const c of options.conditions ?? []) {
    await page.getByLabel(c).check();
  }
  for (const a of options.allergies ?? []) {
    await page.getByLabel(a, { exact: true }).check();
  }
  await page.getByRole("button", { name: "Weiter" }).click();
  if (options.saltLimit) {
    await page.getByLabel("Salz").fill(options.saltLimit);
    await page.getByLabel("Salz").blur();
  }
  await page.getByRole("button", { name: "Speichern" }).click();
  await expect(page.getByText("Test Person")).toBeVisible();
}

/** Loads the fixture product by barcode and opens the food card. */
export async function checkFixtureFood(page: Page, grams = "150 g") {
  await page.getByRole("link", { name: /Lebensmittel prüfen/ }).click();
  await page.getByLabel("Barcode-Nummer").fill(FIXTURE_BARCODE);
  await page.getByRole("button", { name: "Barcode prüfen" }).click();
  await expect(page.getByRole("heading", { level: 1, name: FIXTURE_NAME })).toBeVisible();
  await page.getByRole("button", { name: grams }).click();
  await page.getByRole("link", { name: "Bewerten" }).click();
  await expect(page.getByText("Ergebnis")).toBeVisible();
}
