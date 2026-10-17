import { expect, test } from "@playwright/test";
import { acceptNotice, createProfile, mockOpenFoodFacts } from "./helpers";

test.beforeEach(async ({ page }) => {
  await mockOpenFoodFacts(page);
  await acceptNotice(page);
});

test("set up my foods from the start page and put one on the plate", async ({ page }) => {
  const card = page.locator("#home-my-foods").locator("..");
  await card.getByRole("link", { name: "Meine Lebensmittel einrichten" }).click();

  await page.getByLabel("Lebensmittel suchen").fill("Quark");
  await page.getByRole("button", { name: "Suchen" }).click();
  await page.getByRole("button", { name: "Testquark mager hinzufügen" }).click();
  await expect(page.getByRole("status")).toContainText("Testquark mager steht jetzt in Meine Lebensmittel");
  await expect(page.locator("#my-foods-add").locator("..")).toContainText("schon in der Liste");

  await page.getByLabel("Barcode-Nummer").fill("4000000000017");
  await page.getByRole("button", { name: "Hinzufügen", exact: true }).click();
  await expect(page.locator("#my-foods-list").locator("..")).toContainText("Testjoghurt natur");

  await page.goto("/");
  await expect(card).toContainText("Testquark mager");
  await card.getByRole("button", { name: /Testjoghurt natur/ }).click();
  await expect(card.getByRole("status")).toContainText("Testjoghurt natur liegt auf dem Teller.");
  await expect(page.getByRole("link", { name: /Teller ansehen/ })).toContainText("Heute 1 Lebensmittel eingetragen");
});

test("profile stores the date of birth and shows the age", async ({ page }) => {
  await page.getByRole("link", { name: "Profil anlegen" }).click();
  await page.getByLabel("Name", { exact: true }).fill("Test Person");
  await page.getByLabel("Geburtsdatum").fill("1950-01-01");
  await expect(page.getByText(/Heute \d+ Jahre alt/)).toBeVisible();
  await page.getByRole("button", { name: "Weiter" }).click();
  await page.getByRole("button", { name: "Weiter" }).click();
  await page.getByRole("button", { name: "Speichern" }).click();
  await page.getByRole("link", { name: "Wechseln" }).click();
  await page.getByRole("link", { name: "Ändern" }).click();
  await expect(page.getByLabel("Geburtsdatum")).toHaveValue("1950-01-01");
});
