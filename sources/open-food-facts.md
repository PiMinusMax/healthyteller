# Open Food Facts

| Feld | Wert |
| --- | --- |
| ID in der App | `off` |
| URL | https://world.openfoodfacts.org |
| Schnittstelle | API v2, ein Produkt je Barcode: `https://world.openfoodfacts.org/api/v2/product/{barcode}.json` |
| Namenssuche | [Search-a-licious](https://search.openfoodfacts.org/docs): `https://search.openfoodfacts.org/search?q={begriffe}`; Rückfall auf die ältere Suche `https://world.openfoodfacts.org/cgi/search.pl` |
| Format | JSON, Nährwerte unter `nutriments` als `{name}_100g`, alle außer Energie in Gramm |
| Lizenz | Datenbank: ODbL 1.0, Inhalte: DbCL 1.0, Bilder: CC BY-SA 3.0 |
| Zitierhinweis | Open Food Facts, https://world.openfoodfacts.org, ODbL |
| Adapter | `crates/ingest/src/off.rs` |
| Datei-Import | JSON-Antwort der API für ein Produkt |
| Fixture | `fixtures/open-food-facts/` (erfundene Werte) |

Hinweise:

- Die App fragt nur den Barcode oder den Suchbegriff ab und speichert die Antwort im lokalen Cache. Der Gesamtexport wird nicht genutzt.
- Die Namenssuche liefert nur Barcode, Name und Marke. Wählt man einen Treffer, lädt die App das Produkt vollständig per Barcode, weil der Suchindex älter sein kann als die Produktdaten.
- Search-a-licious erlaubt keine Abrufe aus dem Browser (kein CORS-Header). In der Web-Version schlägt sie deshalb fehl, und die App weicht auf die ältere Suche aus.
- Open Food Facts garantiert weder Richtigkeit noch Vollständigkeit. Die App zeigt deshalb Vollständigkeit und Herkunft der Werte an.
- Nährwerte sind Herstellerangaben und tragen in der App das Kennzeichen ∑.
