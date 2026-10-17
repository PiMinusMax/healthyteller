import { expect, test } from "@playwright/test";
import * as path from "path";
import { FIXTURES, acceptNotice, createProfile, mockOpenFoodFacts } from "./helpers";

test.beforeEach(async ({ page }) => {
  await mockOpenFoodFacts(page);
  await acceptNotice(page);
  await createProfile(page, { allergies: ["Milch"] });
  await page.getByRole("link", { name: /Lebensmittel prüfen/ }).click();
});

test("search by name at Open Food Facts", async ({ page }) => {
  let searched = "";
  page.on("request", (r) => {
    if (r.url().includes("/cgi/search.pl")) searched = new URL(r.url()).searchParams.get("search_terms") ?? "";
  });
  await page.getByLabel("Lebensmittel suchen").fill("Quark");
  await page.getByRole("button", { name: "Online nach Name oder Marke suchen" }).click();
  await expect(page.getByRole("button", { name: /Testquark Vanille/ })).toBeVisible();
  expect(searched).toBe("Quark");
  await page.getByRole("button", { name: /Testquark Vanille/ }).click();
  await expect(page.getByRole("heading", { level: 1, name: "Testquark Vanille" })).toBeVisible();
  await page.getByRole("link", { name: "Bewerten" }).click();
  await expect(page.getByText("In den Daten gefunden: Milch.")).toBeVisible();
  // Source details are shown, but not evaluated.
  const more = page.locator("#card-more").locator("..");
  await expect(more).toContainText("Gruppe 4 von 4");
  await expect(more).toContainText("E1442, E330");
});

test("import a product file as fallback", async ({ page }) => {
  await page.getByLabel("Oder Produktdatei importieren").setInputFiles(path.join(FIXTURES, "product-found.json"));
  await expect(page.getByRole("heading", { level: 1, name: "Testjoghurt natur" })).toBeVisible();
});

test("an unreadable file shows a message", async ({ page }) => {
  await page.getByLabel("Oder Produktdatei importieren").setInputFiles({
    name: "broken.json",
    mimeType: "application/json",
    buffer: Buffer.from("{ not json"),
  });
  await expect(page.getByRole("alert")).toContainText("ließ sich nicht lesen");
});
