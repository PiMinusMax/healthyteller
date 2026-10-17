import AxeBuilder from "@axe-core/playwright";
import { expect, Page, test } from "@playwright/test";
import { acceptNotice, checkFixtureFood, createProfile, mockOpenFoodFacts } from "./helpers";

// WCAG 2.2 AA without violations (AGENTS.md rules 10 and 13).
const WCAG_TAGS = ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"];

async function expectNoViolations(page: Page, label: string) {
  const results = await new AxeBuilder({ page }).withTags(WCAG_TAGS).analyze();
  expect(results.violations, `${label}: ${JSON.stringify(results.violations, null, 2)}`).toEqual([]);
}

for (const scheme of ["light", "dark"] as const) {
  test(`all screens have no WCAG 2.2 AA violations (${scheme})`, async ({ page }) => {
    // Walks through every screen, which takes longer than a single check.
    test.setTimeout(180_000);
    await page.emulateMedia({ colorScheme: scheme });
    await mockOpenFoodFacts(page);
    await page.goto("/");
    await expectNoViolations(page, "welcome");
    await acceptNotice(page);
    await expectNoViolations(page, "home");
    await createProfile(page, { conditions: ["Bluthochdruck"], allergies: ["Milch"], saltLimit: "5" });
    await checkFixtureFood(page);
    await expectNoViolations(page, "food card");
    await page.getByRole("button", { name: "Zu Meine Lebensmittel hinzufügen" }).click();
    await page.getByRole("link", { name: "Alle Nährwerte" }).click();
    await expectNoViolations(page, "nutrient list");
    for (const route of ["/pruefen", "/teller", "/meine-lebensmittel", "/profile", "/profil/neu/2", "/einstellungen", "/ueber", "/druckliste", "/schaetzungen", "/begriff/purine"]) {
      await page.goto(route);
      await expect(page.locator("main")).toBeVisible();
      await expectNoViolations(page, route);
    }
  });
}

test("start page works with the keyboard only", async ({ page }) => {
  await acceptNotice(page);
  const check = page.getByRole("link", { name: /Lebensmittel prüfen/ });
  for (let i = 0; i < 10 && !(await check.evaluate((el) => el === document.activeElement)); i++) {
    await page.keyboard.press("Tab");
  }
  await expect(check).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(page.getByRole("heading", { level: 1, name: "Lebensmittel prüfen" })).toBeVisible();
});

test("content fits at 200 % zoom without horizontal scrolling", async ({ page }) => {
  // 640 px at 200 % zoom equals a 320 px wide viewport.
  await page.setViewportSize({ width: 320, height: 800 });
  await mockOpenFoodFacts(page);
  await acceptNotice(page);
  await createProfile(page, { conditions: ["Bluthochdruck"], saltLimit: "5" });
  await checkFixtureFood(page);
  for (const route of [null, "/teller", "/einstellungen", "/pruefen"]) {
    if (route) await page.goto(route);
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
    expect(overflow, route ?? "food card").toBeLessThanOrEqual(0);
  }
});
