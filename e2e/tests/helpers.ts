import { Page, expect } from "@playwright/test";
import * as fs from "fs";
import * as path from "path";

/** Product fixture with invented values (fixtures/open-food-facts). */
export const FIXTURE_BARCODE = "4000000000017";
export const FIXTURE_NAME = "Testjoghurt natur";

export const FIXTURES = path.join(__dirname, "../../fixtures/open-food-facts");

/**
 * Answers every Open Food Facts request with a fixture, so tests never hit the network.
 * Products from the search fixture are also served by barcode. With `searchDown`,
 * both Search-a-licious indexes answer 503 and the app has to fall back to the older search.
 */
export async function mockOpenFoodFacts(
  page: Page,
  file = "product-found.json",
  options: { searchDown?: boolean } = {},
) {
  const product = fs.readFileSync(path.join(FIXTURES, file), "utf8");
  const search = fs.readFileSync(path.join(FIXTURES, "search-results.json"), "utf8");
  const searchALicious = fs.readFileSync(path.join(FIXTURES, "search-a-licious-results.json"), "utf8");
  const byCode = new Map<string, unknown>(
    JSON.parse(search).products.filter((p: { code?: string }) => p.code).map((p: { code: string }) => [p.code, p]),
  );
  const json = { contentType: "application/json", headers: { "access-control-allow-origin": "*" } };
  await page.route("https://world.openfoodfacts.org/**", (route) => {
    const url = route.request().url();
    const code = url.match(/\/api\/v2\/product\/(\d+)\.json/)?.[1];
    const found = code ? byCode.get(code) : undefined;
    const body = url.includes("/cgi/search.pl")
      ? search
      : found
        ? JSON.stringify({ status: 1, product: found })
        : product;
    return route.fulfill({ status: 200, ...json, body });
  });
  // Search-a-licious: staging index (.net) and production index (.org).
  await page.route(/^https:\/\/search\.openfoodfacts\.(net|org)\//, (route) =>
    options.searchDown
      ? route.fulfill({ status: 503, ...json, body: "{}" })
      : route.fulfill({ status: 200, ...json, body: searchALicious }),
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
  options: { conditions?: string[]; allergies?: string[]; medications?: string[]; saltLimit?: string } = {},
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
  for (const m of options.medications ?? []) {
    await page.getByLabel(m).check();
  }
  await page.getByRole("button", { name: "Weiter" }).click();
  if (options.saltLimit) {
    await page.getByLabel("Salz").fill(options.saltLimit);
    await page.getByLabel("Salz").blur();
  }
  await page.getByRole("button", { name: "Speichern" }).click();
  await expect(page.getByText("Test Person")).toBeVisible();
}

/** Types a barcode on the search screen; on the web the number entry sits behind the scan button. */
export async function typeBarcode(page: Page, code: string) {
  const entry = page.getByText("Nummer eintippen");
  if (await entry.isVisible()) await entry.click();
  await page.getByLabel("Barcode-Nummer").fill(code);
  await page.getByRole("button", { name: "Prüfen", exact: true }).click();
}

/** Loads the fixture product by barcode and opens the food card. */
export async function checkFixtureFood(page: Page, grams = "150 g") {
  await page.getByRole("link", { name: /Lebensmittel prüfen/ }).click();
  await typeBarcode(page, FIXTURE_BARCODE);
  await expect(page.getByRole("heading", { level: 1, name: FIXTURE_NAME })).toBeVisible();
  await page.getByRole("button", { name: grams }).click();
  await page.getByRole("link", { name: "Bewerten" }).click();
  await expect(page.getByText("Ergebnis")).toBeVisible();
}
