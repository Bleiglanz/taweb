table! {
    notifications (id) {
        id -> Int4,
        meldung -> Varchar,
        auftrag -> Varchar,
        turnaround_nr_ -> Varchar,
        beschreibung -> Varchar,
        meldender -> Varchar,
        verantw_arbeitspl_ -> Varchar,
        zusaetzl__bemerk_1 -> Varchar,
        zusaetzl__bemerk_2 -> Varchar,
        zusaetzl__bemerk_3 -> Varchar,
        technische_identnummer -> Varchar,
        rui_schema_nr_ -> Varchar,
        moc_nr_ -> Varchar,
        standort_besonderheit_1 -> Varchar,
        standort_besonderheit_2 -> Varchar,
        planung_fuehrender -> Varchar,
        planung_2 -> Varchar,
        ansprechpartner_betrieb -> Varchar,
        ansprechpartner_technik -> Varchar,
        rohrleitungs_nr -> Varchar,
        psp_element -> Varchar,
        meldungsdatum -> Varchar,
        prio__planung -> Varchar,
        planung_text -> Varchar,
        anlagen_system -> Varchar,
        baufeld_gebaeude -> Varchar,
        kostenschaetzung -> Varchar,
        auftstatus -> Varchar,
        userstatus -> Varchar,
        codierung -> Varchar,
        meldsys_status -> Varchar,
        angelegt_von -> Varchar,
        angelegt_am -> Varchar,
        geaendert_am -> Text,
        geaendert_von -> Text,
        floc_nr -> Text,
        meldungsart -> Text,
        equipment -> Text,
        floc -> Text,
        anre_import_lfdnr -> Varchar,
        anre_file_datum -> Varchar,
    }
}

table! {
    systems (id) {
        id -> Int4,
        sysnr -> Varchar,
        descr -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    notifications,
    systems,
);
