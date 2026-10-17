//! Data sources from the concept (docs/konzept.md, "Öffentliche Datenquellen").
//! Metadata only; the data itself is never stored in the repository.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    /// The app loads data directly from the source.
    Online,
    /// The person downloads a file and imports it.
    FileImport,
    /// Use only after written permission.
    PendingPermission,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub url: &'static str,
    pub license: &'static str,
    pub access: Access,
    /// The app can already use this source.
    pub implemented: bool,
}

pub const SOURCES: &[SourceInfo] = &[
    SourceInfo {
        id: "off",
        name: "Open Food Facts",
        url: "https://world.openfoodfacts.org",
        license: "ODbL 1.0",
        access: Access::Online,
        implemented: true,
    },
    SourceInfo {
        id: "bls",
        name: "Bundeslebensmittelschlüssel (BLS) 4.0",
        url: "https://blsdb.de/",
        license: "kostenfrei, Nutzung in Apps erlaubt (BLS-FAQ)",
        access: Access::FileImport,
        implemented: false,
    },
    SourceInfo {
        id: "usda-ods-purine",
        name: "USDA/ODS-NIH Purin-Datenbank",
        url: "https://library.mcmaster.ca/data/usda-and-ods-nih-database-purine-content-foods",
        license: "US Open Data",
        access: Access::FileImport,
        implemented: false,
    },
    SourceInfo {
        id: "gi-2021",
        name: "GI-Tabellen 2021 (Atkinson et al.)",
        url: "https://nutrition.org/ajcn-publishes-international-tables-of-glycemic-index-and-glycemic-load-values-2021-a-systematic-review/",
        license: "Fachartikel, Nachnutzung zu klären",
        access: Access::FileImport,
        implemented: false,
    },
    SourceInfo {
        id: "usda-fdc",
        name: "USDA FoodData Central",
        url: "https://fdc.nal.usda.gov/api-guide",
        license: "Public Domain",
        access: Access::Online,
        implemented: false,
    },
    SourceInfo {
        id: "ciqual",
        name: "CIQUAL (ANSES)",
        url: "https://www.anses.fr/en/content/ciqual-nutritional-composition-table",
        license: "Open Data, Quellenangabe Pflicht",
        access: Access::FileImport,
        implemented: false,
    },
    SourceInfo {
        id: "harvard-oxalate",
        name: "Harvard-Oxalattabelle",
        // The concept names no URL; the file is published by the Harvard Department of Nutrition.
        url: "",
        license: "ungeklärt, Anfrage offen",
        access: Access::FileImport,
        implemented: false,
    },
    SourceInfo {
        id: "sighi",
        name: "SIGHI-Liste (Histamin)",
        url: "https://histaminintoleranz.ch/en/downloads.html",
        license: "Erlaubnis einholen",
        access: Access::PendingPermission,
        implemented: false,
    },
];

pub fn by_id(id: &str) -> Option<&'static SourceInfo> {
    SOURCES.iter().find(|s| s.id == id)
}
