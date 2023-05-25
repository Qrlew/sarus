
pub const DATASET: &str = r#"
{
    "uuid": "02f75893-794f-40ac-829e-73480d58a4b5",
    "name": "mimic_iii",
    "spec": {
        "sql": {"uri": "postgresql://postgres:test@localhost"}
    }
}
"#;

pub const SCHEAM: &str = r#"
{
    "uuid": "19261961-fd43-4ad1-939a-c4c89c3f73db",
    "dataset": "02f75893-794f-40ac-829e-73480d58a4b5",
    "name": "mimic_iii_schema",
    "type": {
        "name": "mimic_iii",
        "union": {
            "fields": [
                {
                    "name": "admissions",
                    "type": {
                        "struct": {
                            "fields": [
                                {"name": "row_id", "type": {"id": {"base": "INT64"}}},
                                {"name": "subject_id", "type": {"id": {"base": "INT64"}}},
                                {"name": "hadm_id", "type": {"id": {"base": "INT64"}}},
                                {"name": "admittime", "type": {"datetime": {"format": "%Y-%m-%d %H:%M:%S"}}},
                                {"name": "dischtime", "type": {"datetime": {"format": "%Y-%m-%d %H:%M:%S"}}},
                                {"name": "deathtime", "type": {"optional": {"type": {"datetime": {"format": "%Y-%m-%d %H:%M:%S", "min": "2019-12-16 00:00:00", "max": "2029-12-16 00:00:00"}}}}},
                                {"name": "admission_type", "type": {"text": {"encoding": "UTF-8"}}},
                                {"name": "admission_location", "type": {"text": {"encoding": "UTF-8"}}},
                                {"name": "discharge_location", "type": {"text": {"encoding": "UTF-8"}}},
                                {"name": "insurance", "type": {"text": {"encoding": "UTF-8"}}},
                                {"name": "language", "type": {"optional": {"type": {"text": {"encoding": "UTF-8"}}}}},
                                {"name": "religion", "type": {"optional": {"type": {"text": {"encoding": "UTF-8"}}}}},
                                {"name": "marital_status", "type": {"optional": {"type": {"text": {"encoding": "UTF-8"}}}}},
                                {"name": "ethnicity", "type": {"text": {"encoding": "UTF-8"}}},
                                {"name": "edregtime", "type": {"optional": {"type": {"datetime": {"format": "%Y-%m-%d %H:%M:%S", "min": "2019-12-16 00:00:00"}}}}},
                                {"name": "edouttime", "type": {"optional": {"type": {"datetime": {"format": "%Y-%m-%d %H:%M:%S"}}}}},
                                {"name": "diagnosis", "type": {"optional": {"type": {"text": {"encoding": "UTF-8"}}}}},
                                {"name": "hospital_expire_flag", "type": {"optional": {"type": {"integer": {}}}}},
                                {"name": "has_chartevents_data", "type": {"integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807"}}}
                            ]
                        }
                    }
                },
                {
                    "name": "patients",
                    "type": {
                        "struct": {
                            "fields": [
                                {"name": "row_id", "type": {"id": {"base": "INT64"}}},
                                {"name": "subject_id", "type": {"id": {"base": "INT64"}}},
                                {"name": "gender", "type": {"text": {"encoding": "UTF-8"}}},
                                {"name": "dob", "type": {"datetime": {"format": "%Y-%m-%d %H:%M:%S"}}},
                                {"name": "dod", "type": {"optional": {"type": {"datetime": {"format": "%Y-%m-%d %H:%M:%S"}}}}},
                                {"name": "dod_hosp", "type": {"optional": {"type": {"datetime": {"format": "%Y-%m-%d %H:%M:%S"}}}}},
                                {"name": "dod_ssn", "type": {"optional": {"type": {"datetime": {"format": "%Y-%m-%d %H:%M:%S"}}}}},
                                {"name": "expire_flag", "type": {"integer": {"base": "INT64", "min": "-9223372036854775808", "max": "9223372036854775807"}}}
                            ]
                        },
                        "properties": {
                            "size": "100"
                        }
                    }
                }
            ]
        }
    },
    "protected": {
        "label": "mimic_iii",
        "paths": [
            {
                "label": "patients",
                "paths": [
                    {
                        "label": "subject_id",
                        "paths": []
                    }
                ]
            }
        ]
    }
}
"#;
