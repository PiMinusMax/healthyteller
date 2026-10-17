# ADR 0005: Eine Portion am Anteil pro Mahlzeit messen

Status: angenommen, 2026-10-17

## Kontext

Ein Tageswert (Arztvorgabe oder Regel mit `basis: day`) sagt nur, ob die Tagessumme unter dem Max bleibt. Eine Pizza mit 4,8 g Salz passt in einen Tageswert von 6 g und hieß deshalb „gut“, obwohl sie allein fast den ganzen Tag braucht. Eine eigene Schwelle pro Portion darf die App nicht setzen: Jede Zahl braucht eine Quelle (AGENTS.md Regel 2), Regeln eine fachliche Freigabe (Regel 11).

## Entscheidung

- Im Profil steht optional, wie viele Mahlzeiten die Person meist am Tag isst.
- Anteil pro Mahlzeit = Tageswert ÷ Mahlzeiten. Beide Zahlen stammen von der Person oder ihrem Arzt beziehungsweise aus der Quelle der Regel; die App rechnet nur.
- Bewertung einer Portion bei einem Tageswert:
  - Tagessumme mit dieser Portion über dem Max: **meiden**.
  - sonst Portion über dem Anteil pro Mahlzeit: **mit Maß** („mehr als Ihr Anteil pro Mahlzeit“).
  - sonst: **gut**, mit Nennung des Anteils.
  - ohne Angabe der Mahlzeiten: **unbekannt**, mit dem Hinweis, die Mahlzeiten im Profil einzutragen. „Gut“ erscheint nie nur deshalb, weil der Tag noch Platz hat.
- Die Karte zeigt zusätzlich, wie viel Gramm des Lebensmittels in den Anteil pro Mahlzeit und in den ganzen Tageswert passen (Grenze ÷ Gehalt pro Gramm).
- Listen (Suche, Meine Lebensmittel) zeigen je Dimension, was Aufmerksamkeit braucht, für die übliche Portion oder 100 g. Eine Gesamtnote gibt es weiter nicht (Regel 7).

## Folgen

- Regeln mit Tagesbasis werden in den Referenzfällen mit `meals_per_day` geprüft.
- Eine Schwelle pro 100 g aus einer zitierten Quelle (etwa für „viel Salz“) kann später als eigene Regel mit `basis: portion` dazukommen und gilt dann unabhängig von den Mahlzeiten.
