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
    if (/^https:\/\/search\.openfoodfacts\.(net|org)\//.test(r.url())) {
      searched ||= new URL(r.url()).searchParams.get("q") ?? "";
    }
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

test("Enter starts the online search", async ({ page }) => {
  await page.getByLabel("Lebensmittel suchen").fill("Quark");
  await page.getByLabel("Lebensmittel suchen").press("Enter");
  await expect(page.getByRole("button", { name: /Testquark mager/ })).toBeVisible();
});

test("falls back to the older search when Search-a-licious is down", async ({ page }) => {
  await page.unrouteAll();
  await mockOpenFoodFacts(page, "product-found.json", { searchDown: true });
  await page.getByLabel("Lebensmittel suchen").fill("Quark");
  await page.getByRole("button", { name: "Online nach Name oder Marke suchen" }).click();
  await expect(page.getByRole("button", { name: /Testquark Vanille/ })).toBeVisible();
});

test("an overloaded source shows a clear message", async ({ page }) => {
  await page.unrouteAll();
  await page.route(/openfoodfacts\.(org|net)/, (route) =>
    route.fulfill({ status: 503, headers: { "access-control-allow-origin": "*" }, body: "<html></html>" }),
  );
  await page.getByLabel("Lebensmittel suchen").fill("Quark");
  await page.getByRole("button", { name: "Online nach Name oder Marke suchen" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Open Food Facts antwortet gerade nicht. Bitte versuchen Sie es später noch einmal.",
  );
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
