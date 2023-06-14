pub const DATASET: &str = r#"
{
    "@type": "sarus_data_spec/sarus_data_spec.Dataset",
    "uuid": "476edb9a9d4542986b40b7b0ca485a88",
    "name": "Transformed",
    "spec": {
        "transformed": {
        "transform": "b5bcf7dd7285658a083556fe94e3f42d",
        "arguments": [
            "8a5f4206f9db04071ec8fb06c7535c88"
        ],
        "named_arguments": {
            "attributes_budget": "f8b2fe1ffc40b3a92c8a18e1f0aa5112"
        }
        }
    },
    "properties": {
        "slugname": "jp_retail_demo"
    },
    "doc": ""
}
"#;

pub const SCHEMA: &str = r#"
{
    "@type": "sarus_data_spec/sarus_data_spec.Schema",
    "uuid": "dff9bbe9c8afb3a4e624396a8d58b590",
    "dataset": "476edb9a9d4542986b40b7b0ca485a88",
    "name": "jp_retail_demo",
    "type": {
      "struct": {
        "fields": [
          {
            "name": "data",
            "type": {
              "name": "Union",
              "properties": {
                "public_fields": "[]"
              },
              "union": {
                "fields": [
                  {
                    "name": "private",
                    "type": {
                      "name": "private",
                      "properties": {
                        "public_fields": "[]"
                      },
                      "union": {
                        "fields": [
                          {
                            "name": "campaign_descriptions",
                            "type": {
                              "name": "Struct",
                              "properties": {
                                "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSKgoEZGF0YRIiCgdwcml2YXRlEhcKFWNhbXBhaWduX2Rlc2NyaXB0aW9ucw==\"]",
                                "public_columns": "[]",
                                "merge_paths": "0"
                              },
                              "struct": {
                                "fields": [
                                  {
                                    "name": "index",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "26",
                                        "possible_values": [
                                          "0",
                                          "1",
                                          "2",
                                          "3",
                                          "4",
                                          "5",
                                          "6",
                                          "7",
                                          "8",
                                          "9",
                                          "10",
                                          "11",
                                          "12",
                                          "13",
                                          "14",
                                          "15",
                                          "16",
                                          "17",
                                          "18",
                                          "19",
                                          "20",
                                          "21",
                                          "22",
                                          "23",
                                          "24",
                                          "25",
                                          "26"
                                        ],
                                        "base": "INT64",
                                        "min": "0"
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "campaign_id",
                                    "type": {
                                      "name": "Id",
                                      "id": {
                                        "unique": true,
                                        "base": "INT64"
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "campaign_type",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "properties": {
                                        "min_length": "6",
                                        "max_length": "6"
                                      },
                                      "text": {
                                        "encoding": "UTF-8"
                                      }
                                    }
                                  },
                                  {
                                    "name": "start_date",
                                    "type": {
                                      "name": "Datetime",
                                      "datetime": {
                                        "format": "%Y-%m-%d %H:%M:%S",
                                        "min": "2016-11-14 00:00:00",
                                        "max": "2017-12-28 00:00:01",
                                        "possible_values": [
                                          "2017-09-04 00:00:00",
                                          "2017-05-31 00:00:00",
                                          "2017-12-28 00:00:00",
                                          "2017-10-30 00:00:00",
                                          "2017-11-27 00:00:00",
                                          "2016-12-28 00:00:00",
                                          "2017-11-15 00:00:00",
                                          "2016-12-06 00:00:00",
                                          "2017-12-06 00:00:00",
                                          "2017-02-08 00:00:00",
                                          "2017-04-03 00:00:00",
                                          "2017-06-28 00:00:00",
                                          "2017-04-24 00:00:00",
                                          "2017-03-03 00:00:00",
                                          "2017-07-12 00:00:00",
                                          "2017-09-20 00:00:00",
                                          "2017-03-29 00:00:00",
                                          "2017-03-08 00:00:00",
                                          "2017-10-04 00:00:00",
                                          "2017-08-08 00:00:00",
                                          "2017-03-13 00:00:00",
                                          "2016-11-14 00:00:00",
                                          "2017-10-18 00:00:00",
                                          "2017-05-08 00:00:00",
                                          "2017-04-19 00:00:00"
                                        ],
                                        "base": "INT64_NS"
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "end_date",
                                    "type": {
                                      "name": "Datetime",
                                      "datetime": {
                                        "format": "%Y-%m-%d %H:%M:%S",
                                        "min": "2017-01-16 00:00:00",
                                        "max": "2018-02-28 00:00:01",
                                        "possible_values": [
                                          "2017-12-17 00:00:00",
                                          "2017-05-07 00:00:00",
                                          "2017-08-27 00:00:00",
                                          "2018-01-07 00:00:00",
                                          "2017-07-02 00:00:00",
                                          "2017-04-09 00:00:00",
                                          "2017-09-24 00:00:00",
                                          "2017-04-30 00:00:00",
                                          "2017-05-21 00:00:00",
                                          "2017-12-24 00:00:00",
                                          "2017-11-05 00:00:00",
                                          "2017-05-28 00:00:00",
                                          "2017-11-08 00:00:00",
                                          "2017-07-30 00:00:00",
                                          "2018-02-28 00:00:00",
                                          "2017-02-05 00:00:00",
                                          "2017-01-16 00:00:00",
                                          "2017-08-13 00:00:00",
                                          "2017-11-19 00:00:00",
                                          "2018-02-04 00:00:00",
                                          "2017-03-26 00:00:00",
                                          "2017-06-25 00:00:00",
                                          "2017-02-19 00:00:00",
                                          "2017-05-08 00:00:00",
                                          "2018-02-05 00:00:00"
                                        ],
                                        "base": "INT64_NS"
                                      },
                                      "properties": {}
                                    }
                                  }
                                ]
                              }
                            }
                          },
                          {
                            "name": "campaigns",
                            "type": {
                              "name": "Struct",
                              "properties": {
                                "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSHgoEZGF0YRIWCgdwcml2YXRlEgsKCWNhbXBhaWducw==\"]",
                                "public_columns": "[]",
                                "merge_paths": "0"
                              },
                              "struct": {
                                "fields": [
                                  {
                                    "name": "index",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "6588",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "campaign_id",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "26",
                                        "possible_values": [
                                          "0",
                                          "1",
                                          "2",
                                          "3",
                                          "4",
                                          "5",
                                          "6",
                                          "7",
                                          "8",
                                          "9",
                                          "10",
                                          "11",
                                          "12",
                                          "13",
                                          "14",
                                          "15",
                                          "16",
                                          "17",
                                          "18",
                                          "19",
                                          "20",
                                          "21",
                                          "22",
                                          "23",
                                          "24",
                                          "25",
                                          "26"
                                        ],
                                        "base": "INT64",
                                        "min": "0"
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "household_id",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "1558",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  }
                                ]
                              }
                            }
                          },
                          {
                            "name": "coupon_redemptions",
                            "type": {
                              "name": "Struct",
                              "properties": {
                                "merge_paths": "0",
                                "public_columns": "[]",
                                "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSJwoEZGF0YRIfCgdwcml2YXRlEhQKEmNvdXBvbl9yZWRlbXB0aW9ucw==\"]"
                              },
                              "struct": {
                                "fields": [
                                  {
                                    "name": "household_id",
                                    "type": {
                                      "name": "Id",
                                      "id": {
                                        "reference": {
                                          "label": "data",
                                          "paths": [
                                            {
                                              "label": "private",
                                              "paths": [
                                                {
                                                  "label": "households",
                                                  "paths": [
                                                    {
                                                      "label": "household_id",
                                                      "paths": [],
                                                      "properties": {}
                                                    }
                                                  ],
                                                  "properties": {}
                                                }
                                              ],
                                              "properties": {}
                                            }
                                          ],
                                          "properties": {}
                                        },
                                        "base": "INT64",
                                        "unique": false
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "coupon_upc",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "490",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "campaign_id",
                                    "type": {
                                      "name": "Id",
                                      "id": {
                                        "reference": {
                                          "label": "data",
                                          "paths": [
                                            {
                                              "label": "private",
                                              "paths": [
                                                {
                                                  "label": "campaign_descriptions",
                                                  "paths": [
                                                    {
                                                      "label": "campaign_id",
                                                      "paths": [],
                                                      "properties": {}
                                                    }
                                                  ],
                                                  "properties": {}
                                                }
                                              ],
                                              "properties": {}
                                            }
                                          ],
                                          "properties": {}
                                        },
                                        "base": "INT64",
                                        "unique": false
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "redemption_date",
                                    "type": {
                                      "name": "Datetime",
                                      "datetime": {
                                        "format": "%Y-%m-%d %H:%M:%S",
                                        "min": "2017-01-01 00:00:00",
                                        "max": "2017-12-31 00:00:01",
                                        "possible_values": [],
                                        "base": "INT64_NS"
                                      },
                                      "properties": {}
                                    }
                                  }
                                ]
                              }
                            }
                          },
                          {
                            "name": "coupons",
                            "type": {
                              "name": "Struct",
                              "properties": {
                                "merge_paths": "0",
                                "public_columns": "[]",
                                "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSHAoEZGF0YRIUCgdwcml2YXRlEgkKB2NvdXBvbnM=\"]"
                              },
                              "struct": {
                                "fields": [
                                  {
                                    "name": "index",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "116203",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "coupon_upc",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "980",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "product_id",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "41856",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "campaign_id",
                                    "type": {
                                      "name": "Id",
                                      "id": {
                                        "reference": {
                                          "label": "data",
                                          "paths": [
                                            {
                                              "label": "private",
                                              "paths": [
                                                {
                                                  "label": "campaign_descriptions",
                                                  "paths": [
                                                    {
                                                      "label": "campaign_id",
                                                      "paths": [],
                                                      "properties": {}
                                                    }
                                                  ],
                                                  "properties": {}
                                                }
                                              ],
                                              "properties": {}
                                            }
                                          ],
                                          "properties": {}
                                        },
                                        "base": "INT64",
                                        "unique": false
                                      },
                                      "properties": {}
                                    }
                                  }
                                ]
                              }
                            }
                          },
                          {
                            "name": "demographics",
                            "type": {
                              "name": "Struct",
                              "properties": {
                                "merge_paths": "0",
                                "public_columns": "[]",
                                "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSIQoEZGF0YRIZCgdwcml2YXRlEg4KDGRlbW9ncmFwaGljcw==\"]"
                              },
                              "struct": {
                                "fields": [
                                  {
                                    "name": "household_id",
                                    "type": {
                                      "name": "Id",
                                      "id": {
                                        "reference": {
                                          "label": "data",
                                          "paths": [
                                            {
                                              "label": "private",
                                              "paths": [
                                                {
                                                  "label": "households",
                                                  "paths": [
                                                    {
                                                      "label": "household_id",
                                                      "paths": [],
                                                      "properties": {}
                                                    }
                                                  ],
                                                  "properties": {}
                                                }
                                              ],
                                              "properties": {}
                                            }
                                          ],
                                          "properties": {}
                                        },
                                        "base": "INT64",
                                        "unique": false
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "age",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "properties": {
                                        "max_length": "5",
                                        "min_length": "3"
                                      },
                                      "text": {
                                        "encoding": "UTF-8"
                                      }
                                    }
                                  },
                                  {
                                    "name": "income",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "properties": {
                                        "min_length": "5",
                                        "max_length": "9"
                                      },
                                      "text": {
                                        "encoding": "UTF-8"
                                      }
                                    }
                                  },
                                  {
                                    "name": "home_ownership",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "optional": {
                                        "type": {
                                          "name": "Text UTF-8",
                                          "properties": {
                                            "max_length": "18",
                                            "min_length": "6"
                                          },
                                          "text": {
                                            "encoding": "UTF-8"
                                          }
                                        }
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "marital_status",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "optional": {
                                        "type": {
                                          "name": "Text UTF-8",
                                          "properties": {
                                            "min_length": "7",
                                            "max_length": "9"
                                          },
                                          "text": {
                                            "encoding": "UTF-8"
                                          }
                                        }
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "household_size",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "properties": {
                                        "min_length": "1",
                                        "max_length": "2"
                                      },
                                      "text": {
                                        "encoding": "UTF-8"
                                      }
                                    }
                                  },
                                  {
                                    "name": "household_comp",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "properties": {
                                        "max_length": "16",
                                        "min_length": "12"
                                      },
                                      "text": {
                                        "encoding": "UTF-8"
                                      }
                                    }
                                  },
                                  {
                                    "name": "kids_count",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "properties": {
                                        "max_length": "2",
                                        "min_length": "1"
                                      },
                                      "text": {
                                        "encoding": "UTF-8"
                                      }
                                    }
                                  }
                                ]
                              }
                            }
                          },
                          {
                            "name": "households",
                            "type": {
                              "name": "Struct",
                              "properties": {
                                "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSHwoEZGF0YRIXCgdwcml2YXRlEgwKCmhvdXNlaG9sZHM=\"]",
                                "public_columns": "[]",
                                "merge_paths": "0"
                              },
                              "struct": {
                                "fields": [
                                  {
                                    "name": "index",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "2468",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "household_id",
                                    "type": {
                                      "name": "Id",
                                      "id": {
                                        "unique": true,
                                        "base": "INT64"
                                      },
                                      "properties": {}
                                    }
                                  }
                                ]
                              }
                            }
                          },
                          {
                            "name": "products",
                            "type": {
                              "name": "Struct",
                              "properties": {
                                "merge_paths": "0",
                                "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSHQoEZGF0YRIVCgdwcml2YXRlEgoKCHByb2R1Y3Rz\"]",
                                "public_columns": "[]"
                              },
                              "struct": {
                                "fields": [
                                  {
                                    "name": "index",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "92330",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "product_id",
                                    "type": {
                                      "name": "Id",
                                      "id": {
                                        "unique": true,
                                        "base": "INT64"
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "manufacturer_id",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "6470",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "department",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "properties": {
                                        "min_length": "4",
                                        "max_length": "16"
                                      },
                                      "text": {
                                        "encoding": "UTF-8"
                                      }
                                    }
                                  },
                                  {
                                    "name": "brand",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "properties": {
                                        "min_length": "7",
                                        "max_length": "8"
                                      },
                                      "text": {
                                        "encoding": "UTF-8"
                                      }
                                    }
                                  },
                                  {
                                    "name": "product_category",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "optional": {
                                        "type": {
                                          "name": "Text UTF-8",
                                          "properties": {
                                            "max_length": "30",
                                            "text_alphabet_name": "FullUserInput",
                                            "min_length": "4",
                                            "text_char_set": "[32, 38, 40, 41, 45, 46, 47, 58, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90]"
                                          },
                                          "text": {
                                            "encoding": "UTF-8"
                                          }
                                        }
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "product_type",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "optional": {
                                        "type": {
                                          "name": "Text UTF-8",
                                          "properties": {
                                            "text_char_set": "[32, 36, 37, 38, 40, 41, 43, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 60, 61, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90]",
                                            "text_alphabet_name": "FullUserInput",
                                            "min_length": "3",
                                            "max_length": "30"
                                          },
                                          "text": {
                                            "encoding": "UTF-8"
                                          }
                                        }
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "package_size",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "optional": {
                                        "type": {
                                          "name": "Text UTF-8",
                                          "properties": {
                                            "min_length": "1",
                                            "text_alphabet_name": "FullUserInput",
                                            "max_length": "13",
                                            "text_char_set": "[32, 40, 41, 42, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 95]"
                                          },
                                          "text": {
                                            "encoding": "UTF-8"
                                          }
                                        }
                                      },
                                      "properties": {}
                                    }
                                  }
                                ]
                              }
                            }
                          },
                          {
                            "name": "promotions",
                            "type": {
                              "name": "Struct",
                              "properties": {
                                "merge_paths": "0",
                                "public_columns": "[]",
                                "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSHwoEZGF0YRIXCgdwcml2YXRlEgwKCnByb21vdGlvbnM=\"]"
                              },
                              "struct": {
                                "fields": [
                                  {
                                    "name": "index",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "20940528",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "product_id",
                                    "type": {
                                      "name": "Id",
                                      "id": {
                                        "reference": {
                                          "label": "data",
                                          "paths": [
                                            {
                                              "label": "private",
                                              "paths": [
                                                {
                                                  "label": "products",
                                                  "paths": [
                                                    {
                                                      "label": "product_id",
                                                      "paths": [],
                                                      "properties": {}
                                                    }
                                                  ],
                                                  "properties": {}
                                                }
                                              ],
                                              "properties": {}
                                            }
                                          ],
                                          "properties": {}
                                        },
                                        "base": "INT64",
                                        "unique": false
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "store_id",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "111",
                                        "possible_values": [
                                          "0",
                                          "1",
                                          "2",
                                          "3",
                                          "4",
                                          "5",
                                          "6",
                                          "7",
                                          "8",
                                          "9",
                                          "10",
                                          "11",
                                          "12",
                                          "13",
                                          "14",
                                          "15",
                                          "16",
                                          "17",
                                          "18",
                                          "19",
                                          "20",
                                          "21",
                                          "22",
                                          "23",
                                          "24",
                                          "25",
                                          "26",
                                          "27",
                                          "28",
                                          "29",
                                          "30",
                                          "31",
                                          "32",
                                          "33",
                                          "34",
                                          "35",
                                          "36",
                                          "37",
                                          "38",
                                          "39",
                                          "40",
                                          "41",
                                          "42",
                                          "43",
                                          "44",
                                          "45",
                                          "46",
                                          "47",
                                          "48",
                                          "49",
                                          "50",
                                          "51",
                                          "52",
                                          "53",
                                          "54",
                                          "55",
                                          "56",
                                          "57",
                                          "58",
                                          "59",
                                          "60",
                                          "61",
                                          "62",
                                          "63",
                                          "64",
                                          "65",
                                          "66",
                                          "67",
                                          "68",
                                          "69",
                                          "70",
                                          "71",
                                          "72",
                                          "73",
                                          "74",
                                          "75",
                                          "76",
                                          "77",
                                          "78",
                                          "79",
                                          "80",
                                          "81",
                                          "82",
                                          "83",
                                          "84",
                                          "85",
                                          "86",
                                          "87",
                                          "88",
                                          "89",
                                          "90",
                                          "91",
                                          "92",
                                          "93",
                                          "94",
                                          "95",
                                          "96",
                                          "97",
                                          "98",
                                          "99",
                                          "100",
                                          "101",
                                          "102",
                                          "103",
                                          "104",
                                          "105",
                                          "106",
                                          "107",
                                          "108",
                                          "109",
                                          "110",
                                          "111"
                                        ],
                                        "base": "INT64",
                                        "min": "0"
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "display_location",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "properties": {
                                        "max_length": "1",
                                        "min_length": "1"
                                      },
                                      "text": {
                                        "encoding": "UTF-8"
                                      }
                                    }
                                  },
                                  {
                                    "name": "mailer_location",
                                    "type": {
                                      "name": "Text UTF-8",
                                      "properties": {
                                        "max_length": "1",
                                        "min_length": "1"
                                      },
                                      "text": {
                                        "encoding": "UTF-8"
                                      }
                                    }
                                  },
                                  {
                                    "name": "week",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "min": "1",
                                        "max": "53",
                                        "possible_values": [
                                          "1",
                                          "2",
                                          "3",
                                          "5",
                                          "6",
                                          "7",
                                          "8",
                                          "9",
                                          "10",
                                          "11",
                                          "12",
                                          "13",
                                          "14",
                                          "15",
                                          "16",
                                          "17",
                                          "18",
                                          "19",
                                          "20",
                                          "21",
                                          "22",
                                          "23",
                                          "24",
                                          "25",
                                          "26",
                                          "27",
                                          "28",
                                          "30",
                                          "31",
                                          "32",
                                          "34",
                                          "35",
                                          "36",
                                          "37",
                                          "39",
                                          "40",
                                          "41",
                                          "42",
                                          "43",
                                          "44",
                                          "45",
                                          "47",
                                          "48",
                                          "49",
                                          "51",
                                          "52",
                                          "53"
                                        ],
                                        "base": "INT64"
                                      },
                                      "properties": {}
                                    }
                                  }
                                ]
                              }
                            }
                          },
                          {
                            "name": "transactions",
                            "type": {
                              "name": "Struct",
                              "properties": {
                                "merge_paths": "0",
                                "public_columns": "[]",
                                "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSIQoEZGF0YRIZCgdwcml2YXRlEg4KDHRyYW5zYWN0aW9ucw==\"]"
                              },
                              "struct": {
                                "fields": [
                                  {
                                    "name": "household_id",
                                    "type": {
                                      "name": "Id",
                                      "id": {
                                        "reference": {
                                          "label": "data",
                                          "paths": [
                                            {
                                              "label": "private",
                                              "paths": [
                                                {
                                                  "label": "households",
                                                  "paths": [
                                                    {
                                                      "label": "household_id",
                                                      "paths": [],
                                                      "properties": {}
                                                    }
                                                  ],
                                                  "properties": {}
                                                }
                                              ],
                                              "properties": {}
                                            }
                                          ],
                                          "properties": {}
                                        },
                                        "base": "INT64",
                                        "unique": false
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "store_id",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "456",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "basket_id",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "155846",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "product_id",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "92350",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "quantity",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "max": "89638",
                                        "base": "INT64",
                                        "min": "0",
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "sales_value",
                                    "type": {
                                      "name": "Float64",
                                      "float": {
                                        "max": 840.0,
                                        "base": "FLOAT64",
                                        "min": 0.0,
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "retail_disc",
                                    "type": {
                                      "name": "Float64",
                                      "float": {
                                        "max": 130.02,
                                        "base": "FLOAT64",
                                        "min": 0.0,
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "coupon_disc",
                                    "type": {
                                      "name": "Float64",
                                      "float": {
                                        "max": 55.93,
                                        "base": "FLOAT64",
                                        "min": 0.0,
                                        "possible_values": []
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "coupon_match_disc",
                                    "type": {
                                      "name": "Float64",
                                      "float": {
                                        "max": 7.7,
                                        "possible_values": [
                                          0.0,
                                          0.0,
                                          89128.96,
                                          1.035,
                                          -1.5881868e-23,
                                          1.4499999,
                                          4.172325e-08,
                                          1.525,
                                          -1.5881868e-23,
                                          1.5749999,
                                          -71.68,
                                          1.5949999,
                                          0.0,
                                          1.625,
                                          1.2666203e-26,
                                          1.635,
                                          1.951564e-20,
                                          1.64,
                                          4.172325e-08,
                                          1.65,
                                          2.720083e+23,
                                          1.675,
                                          -7.4629786e-36,
                                          1.6899999,
                                          -1.5881868e-23,
                                          1.6999999,
                                          -2.3314683e-17,
                                          1.7049999,
                                          -71.68,
                                          1.7199999,
                                          -107374184.0,
                                          1.7249999,
                                          -4.932682e+32,
                                          1.7449999,
                                          0.0,
                                          1.75,
                                          1.2666203e-26,
                                          1.76,
                                          -1.5881868e-23,
                                          1.7624999,
                                          -3.3760442e-11,
                                          1.7724999,
                                          4.172325e-08,
                                          1.775,
                                          -107374184.0,
                                          1.7874999,
                                          126443840000.0,
                                          1.79,
                                          2.720083e+23,
                                          1.8,
                                          0.0,
                                          1.8125,
                                          -7.4629786e-36,
                                          1.8149999,
                                          -1.5881868e-23,
                                          1.8249999,
                                          4.172325e-08,
                                          1.8375,
                                          -107374184.0,
                                          1.8499999,
                                          2.720083e+23,
                                          1.8625,
                                          0.0,
                                          1.875,
                                          -107374184.0,
                                          1.8812499,
                                          -1.5881868e-23,
                                          1.8874999,
                                          4.172325e-08,
                                          1.9,
                                          0.0,
                                          1.90625,
                                          -107374184.0,
                                          1.9124999,
                                          -1.5881868e-23,
                                          1.9187499,
                                          2.720083e+23,
                                          1.925,
                                          -5.0048828e-05,
                                          1.9337499,
                                          0.0,
                                          1.9375,
                                          0.0,
                                          1.96875,
                                          -107374184.0,
                                          1.9749999,
                                          2.720083e+23,
                                          1.9875,
                                          0.0,
                                          2.0,
                                          0.0,
                                          2.03125,
                                          -1.5881868e-23,
                                          2.0562499,
                                          0.0,
                                          2.0625,
                                          -1.5881868e-23,
                                          2.0874999,
                                          4.172325e-08,
                                          2.14375,
                                          4.172325e-08,
                                          2.253125,
                                          0.0,
                                          2.28125,
                                          -107374184.0,
                                          2.3093748,
                                          -1.5881868e-23,
                                          2.3374999,
                                          -107374184.0,
                                          2.4812498
                                        ],
                                        "base": "FLOAT64",
                                        "min": 0.0
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "week",
                                    "type": {
                                      "name": "Integer",
                                      "integer": {
                                        "min": "1",
                                        "max": "53",
                                        "possible_values": [
                                          "1",
                                          "2",
                                          "3",
                                          "4",
                                          "5",
                                          "6",
                                          "7",
                                          "8",
                                          "9",
                                          "10",
                                          "11",
                                          "12",
                                          "13",
                                          "14",
                                          "15",
                                          "16",
                                          "17",
                                          "18",
                                          "19",
                                          "20",
                                          "21",
                                          "22",
                                          "23",
                                          "24",
                                          "25",
                                          "26",
                                          "27",
                                          "28",
                                          "29",
                                          "30",
                                          "31",
                                          "32",
                                          "33",
                                          "34",
                                          "35",
                                          "36",
                                          "37",
                                          "38",
                                          "39",
                                          "40",
                                          "41",
                                          "42",
                                          "43",
                                          "44",
                                          "45",
                                          "46",
                                          "47",
                                          "48",
                                          "49",
                                          "50",
                                          "51",
                                          "52",
                                          "53"
                                        ],
                                        "base": "INT64"
                                      },
                                      "properties": {}
                                    }
                                  },
                                  {
                                    "name": "transaction_timestamp",
                                    "type": {
                                      "name": "Datetime",
                                      "datetime": {
                                        "format": "%Y-%m-%d %H:%M:%S",
                                        "min": "2017-01-01 11:53:26",
                                        "max": "2018-01-01 03:50:04",
                                        "possible_values": [],
                                        "base": "INT64_NS"
                                      },
                                      "properties": {}
                                    }
                                  }
                                ]
                              }
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              }
            }
          },
          {
            "name": "sarus_weights",
            "type": {
              "name": "Integer",
              "integer": {
                "min": "-9223372036854775808",
                "max": "9223372036854775807",
                "base": "INT64",
                "possible_values": []
              },
              "properties": {}
            }
          },
          {
            "name": "sarus_is_public",
            "type": {
              "name": "Boolean",
              "boolean": {},
              "properties": {}
            }
          },
          {
            "name": "sarus_protected_entity",
            "type": {
              "name": "Id",
              "id": {
                "base": "STRING",
                "unique": false
              },
              "properties": {}
            }
          }
        ]
      },
      "name": "",
      "properties": {}
    },
    "protected": {
      "label": "data",
      "paths": [
        {
          "label": "private",
          "paths": [
            {
              "label": "campaign_descriptions",
              "paths": [],
              "properties": {}
            }
          ],
          "properties": {}
        },
        {
          "label": "private",
          "paths": [
            {
              "label": "campaigns",
              "paths": [],
              "properties": {}
            }
          ],
          "properties": {}
        },
        {
          "label": "private",
          "paths": [
            {
              "label": "coupon_redemptions",
              "paths": [],
              "properties": {}
            }
          ],
          "properties": {}
        },
        {
          "label": "private",
          "paths": [
            {
              "label": "coupons",
              "paths": [],
              "properties": {}
            }
          ],
          "properties": {}
        },
        {
          "label": "private",
          "paths": [
            {
              "label": "demographics",
              "paths": [],
              "properties": {}
            }
          ],
          "properties": {}
        },
        {
          "label": "private",
          "paths": [
            {
              "label": "households",
              "paths": [],
              "properties": {}
            }
          ],
          "properties": {}
        },
        {
          "label": "private",
          "paths": [
            {
              "label": "products",
              "paths": [],
              "properties": {}
            }
          ],
          "properties": {}
        },
        {
          "label": "private",
          "paths": [
            {
              "label": "promotions",
              "paths": [],
              "properties": {}
            }
          ],
          "properties": {}
        },
        {
          "label": "private",
          "paths": [
            {
              "label": "transactions",
              "paths": [],
              "properties": {}
            }
          ],
          "properties": {}
        }
      ],
      "properties": {}
    },
    "properties": {
      "max_max_multiplicity": "1",
      "foreign_keys": "{\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSLgoHcHJpdmF0ZRIjChJjb3Vwb25fcmVkZW1wdGlvbnMSDQoLY2FtcGFpZ25faWQ=\": \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSMQoHcHJpdmF0ZRImChVjYW1wYWlnbl9kZXNjcmlwdGlvbnMSDQoLY2FtcGFpZ25faWQ=\", \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSLwoHcHJpdmF0ZRIkChJjb3Vwb25fcmVkZW1wdGlvbnMSDgoMaG91c2Vob2xkX2lk\": \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSJwoHcHJpdmF0ZRIcCgpob3VzZWhvbGRzEg4KDGhvdXNlaG9sZF9pZA==\", \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSIwoHcHJpdmF0ZRIYCgdjb3Vwb25zEg0KC2NhbXBhaWduX2lk\": \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSMQoHcHJpdmF0ZRImChVjYW1wYWlnbl9kZXNjcmlwdGlvbnMSDQoLY2FtcGFpZ25faWQ=\", \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSKQoHcHJpdmF0ZRIeCgxkZW1vZ3JhcGhpY3MSDgoMaG91c2Vob2xkX2lk\": \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSJwoHcHJpdmF0ZRIcCgpob3VzZWhvbGRzEg4KDGhvdXNlaG9sZF9pZA==\", \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSJQoHcHJpdmF0ZRIaCgpwcm9tb3Rpb25zEgwKCnByb2R1Y3RfaWQ=\": \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSIwoHcHJpdmF0ZRIYCghwcm9kdWN0cxIMCgpwcm9kdWN0X2lk\", \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSKQoHcHJpdmF0ZRIeCgx0cmFuc2FjdGlvbnMSDgoMaG91c2Vob2xkX2lk\": \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSJwoHcHJpdmF0ZRIcCgpob3VzZWhvbGRzEg4KDGhvdXNlaG9sZF9pZA==\"}",
      "primary_keys": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSMQoHcHJpdmF0ZRImChVjYW1wYWlnbl9kZXNjcmlwdGlvbnMSDQoLY2FtcGFpZ25faWQ=\", \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSJwoHcHJpdmF0ZRIcCgpob3VzZWhvbGRzEg4KDGhvdXNlaG9sZF9pZA==\", \"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSIwoHcHJpdmF0ZRIYCghwcm9kdWN0cxIMCgpwcm9kdWN0X2lk\"]"
    }
}
"#;

pub const SIZE: &str = r#"
{
  "@type": "sarus_data_spec/sarus_data_spec.Size",
  "uuid": "f0eba4de606f6b95e658aea48fe0d63a",
  "dataset": "476edb9a9d4542986b40b7b0ca485a88",
  "name": "Transformed_sizes",
  "statistics": {
    "name": "Union",
    "properties": {
      "public_fields": "[]"
    },
    "union": {
      "fields": [
        {
          "name": "private",
          "statistics": {
            "name": "private",
            "properties": {
              "public_fields": "[]"
            },
            "union": {
              "fields": [
                {
                  "name": "campaign_descriptions",
                  "statistics": {
                    "name": "Struct",
                    "properties": {
                      "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSKgoEZGF0YRIiCgdwcml2YXRlEhcKFWNhbXBhaWduX2Rlc2NyaXB0aW9ucw==\"]",
                      "merge_paths": "0",
                      "public_columns": "[]"
                    },
                    "struct": {
                      "fields": [
                        {
                          "name": "index",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "27",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "campaign_id",
                          "statistics": {
                            "name": "Id",
                            "id": {
                              "size": "27",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "campaign_type",
                          "statistics": {
                            "name": "Text",
                            "text": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "27",
                              "example": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "start_date",
                          "statistics": {
                            "name": "Datetime",
                            "datetime": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "27",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "end_date",
                          "statistics": {
                            "name": "Datetime",
                            "datetime": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "27",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        }
                      ],
                      "size": "27",
                      "name": "",
                      "multiplicity": 0.0
                    }
                  }
                },
                {
                  "name": "campaigns",
                  "statistics": {
                    "name": "Struct",
                    "properties": {
                      "merge_paths": "0",
                      "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSHgoEZGF0YRIWCgdwcml2YXRlEgsKCWNhbXBhaWducw==\"]",
                      "public_columns": "[]"
                    },
                    "struct": {
                      "fields": [
                        {
                          "name": "index",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "6589",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "campaign_id",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "6589",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "household_id",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "6589",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        }
                      ],
                      "size": "6589",
                      "name": "",
                      "multiplicity": 0.0
                    }
                  }
                },
                {
                  "name": "coupon_redemptions",
                  "statistics": {
                    "name": "Struct",
                    "properties": {
                      "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSJwoEZGF0YRIfCgdwcml2YXRlEhQKEmNvdXBvbl9yZWRlbXB0aW9ucw==\"]",
                      "merge_paths": "0",
                      "public_columns": "[]"
                    },
                    "struct": {
                      "fields": [
                        {
                          "name": "household_id",
                          "statistics": {
                            "name": "Id",
                            "id": {
                              "size": "2100",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "coupon_upc",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "2100",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "campaign_id",
                          "statistics": {
                            "name": "Id",
                            "id": {
                              "size": "2100",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "redemption_date",
                          "statistics": {
                            "name": "Datetime",
                            "datetime": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "2100",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        }
                      ],
                      "size": "2100",
                      "name": "",
                      "multiplicity": 0.0
                    }
                  }
                },
                {
                  "name": "coupons",
                  "statistics": {
                    "name": "Struct",
                    "properties": {
                      "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSHAoEZGF0YRIUCgdwcml2YXRlEgkKB2NvdXBvbnM=\"]",
                      "public_columns": "[]",
                      "merge_paths": "0"
                    },
                    "struct": {
                      "fields": [
                        {
                          "name": "index",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "116209",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "coupon_upc",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "116209",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "product_id",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "116209",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "campaign_id",
                          "statistics": {
                            "name": "Id",
                            "id": {
                              "size": "116209",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        }
                      ],
                      "size": "116209",
                      "name": "",
                      "multiplicity": 0.0
                    }
                  }
                },
                {
                  "name": "demographics",
                  "statistics": {
                    "name": "Struct",
                    "properties": {
                      "merge_paths": "0",
                      "public_columns": "[]",
                      "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSIQoEZGF0YRIZCgdwcml2YXRlEg4KDGRlbW9ncmFwaGljcw==\"]"
                    },
                    "struct": {
                      "fields": [
                        {
                          "name": "household_id",
                          "statistics": {
                            "name": "Id",
                            "id": {
                              "size": "788",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "age",
                          "statistics": {
                            "name": "Text",
                            "text": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "788",
                              "example": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "income",
                          "statistics": {
                            "name": "Text",
                            "text": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "788",
                              "example": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "home_ownership",
                          "statistics": {
                            "name": "Text UTF-8",
                            "optional": {
                              "statistics": {
                                "name": "Text",
                                "text": {
                                  "distribution": {
                                    "integer": {
                                      "min": "-9223372036854775808",
                                      "max": "9223372036854775807",
                                      "points": []
                                    },
                                    "properties": {}
                                  },
                                  "size": "561",
                                  "example": "",
                                  "multiplicity": 0.0
                                },
                                "properties": {}
                              },
                              "size": "788",
                              "name": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "marital_status",
                          "statistics": {
                            "name": "Text UTF-8",
                            "optional": {
                              "statistics": {
                                "name": "Text",
                                "text": {
                                  "distribution": {
                                    "integer": {
                                      "min": "-9223372036854775808",
                                      "max": "9223372036854775807",
                                      "points": []
                                    },
                                    "properties": {}
                                  },
                                  "size": "662",
                                  "example": "",
                                  "multiplicity": 0.0
                                },
                                "properties": {}
                              },
                              "size": "788",
                              "name": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "household_size",
                          "statistics": {
                            "name": "Text",
                            "text": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "788",
                              "example": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "household_comp",
                          "statistics": {
                            "name": "Text",
                            "text": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "788",
                              "example": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "kids_count",
                          "statistics": {
                            "name": "Text",
                            "text": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "788",
                              "example": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        }
                      ],
                      "size": "788",
                      "name": "",
                      "multiplicity": 0.0
                    }
                  }
                },
                {
                  "name": "households",
                  "statistics": {
                    "name": "Struct",
                    "properties": {
                      "merge_paths": "0",
                      "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSHwoEZGF0YRIXCgdwcml2YXRlEgwKCmhvdXNlaG9sZHM=\"]",
                      "public_columns": "[]"
                    },
                    "struct": {
                      "fields": [
                        {
                          "name": "index",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "2471",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "household_id",
                          "statistics": {
                            "name": "Id",
                            "id": {
                              "size": "2471",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        }
                      ],
                      "size": "2471",
                      "name": "",
                      "multiplicity": 0.0
                    }
                  }
                },
                {
                  "name": "products",
                  "statistics": {
                    "name": "Struct",
                    "properties": {
                      "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSHQoEZGF0YRIVCgdwcml2YXRlEgoKCHByb2R1Y3Rz\"]",
                      "merge_paths": "0",
                      "public_columns": "[]"
                    },
                    "struct": {
                      "fields": [
                        {
                          "name": "index",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "92329",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "product_id",
                          "statistics": {
                            "name": "Id",
                            "id": {
                              "size": "92329",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "manufacturer_id",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "92329",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "department",
                          "statistics": {
                            "name": "Text",
                            "text": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "92329",
                              "example": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "brand",
                          "statistics": {
                            "name": "Text",
                            "text": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "92329",
                              "example": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "product_category",
                          "statistics": {
                            "name": "Text UTF-8",
                            "optional": {
                              "statistics": {
                                "name": "Text",
                                "text": {
                                  "distribution": {
                                    "integer": {
                                      "min": "-9223372036854775808",
                                      "max": "9223372036854775807",
                                      "points": []
                                    },
                                    "properties": {}
                                  },
                                  "size": "91791",
                                  "example": "",
                                  "multiplicity": 0.0
                                },
                                "properties": {}
                              },
                              "size": "92329",
                              "name": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "product_type",
                          "statistics": {
                            "name": "Text UTF-8",
                            "optional": {
                              "statistics": {
                                "name": "Text",
                                "text": {
                                  "distribution": {
                                    "integer": {
                                      "min": "-9223372036854775808",
                                      "max": "9223372036854775807",
                                      "points": []
                                    },
                                    "properties": {}
                                  },
                                  "size": "91802",
                                  "example": "",
                                  "multiplicity": 0.0
                                },
                                "properties": {}
                              },
                              "size": "92329",
                              "name": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "package_size",
                          "statistics": {
                            "name": "Text UTF-8",
                            "optional": {
                              "statistics": {
                                "name": "Text",
                                "text": {
                                  "distribution": {
                                    "integer": {
                                      "min": "-9223372036854775808",
                                      "max": "9223372036854775807",
                                      "points": []
                                    },
                                    "properties": {}
                                  },
                                  "size": "61747",
                                  "example": "",
                                  "multiplicity": 0.0
                                },
                                "properties": {}
                              },
                              "size": "92329",
                              "name": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        }
                      ],
                      "size": "92329",
                      "name": "",
                      "multiplicity": 0.0
                    }
                  }
                },
                {
                  "name": "promotions",
                  "statistics": {
                    "name": "Struct",
                    "properties": {
                      "public_columns": "[]",
                      "merge_paths": "0",
                      "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSHwoEZGF0YRIXCgdwcml2YXRlEgwKCnByb21vdGlvbnM=\"]"
                    },
                    "struct": {
                      "fields": [
                        {
                          "name": "index",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "4733082",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "product_id",
                          "statistics": {
                            "name": "Id",
                            "id": {
                              "size": "4733082",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "store_id",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "4733082",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "display_location",
                          "statistics": {
                            "name": "Text",
                            "text": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "4733082",
                              "example": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "mailer_location",
                          "statistics": {
                            "name": "Text",
                            "text": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "4733082",
                              "example": "",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "week",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "4733082",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        }
                      ],
                      "size": "4733082",
                      "name": "",
                      "multiplicity": 0.0
                    }
                  }
                },
                {
                  "name": "transactions",
                  "statistics": {
                    "name": "Struct",
                    "properties": {
                      "public_columns": "[]",
                      "non_zero_protected_values": "[\"CiRzYXJ1c19kYXRhX3NwZWMvc2FydXNfZGF0YV9zcGVjLlBhdGgSIQoEZGF0YRIZCgdwcml2YXRlEg4KDHRyYW5zYWN0aW9ucw==\"]",
                      "merge_paths": "0"
                    },
                    "struct": {
                      "fields": [
                        {
                          "name": "household_id",
                          "statistics": {
                            "name": "Id",
                            "id": {
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "store_id",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "basket_id",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "product_id",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "quantity",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "sales_value",
                          "statistics": {
                            "name": "Float",
                            "float": {
                              "distribution": {
                                "double": {
                                  "min": -1.7976931348623157e+308,
                                  "max": 1.7976931348623157e+308,
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "retail_disc",
                          "statistics": {
                            "name": "Float",
                            "float": {
                              "distribution": {
                                "double": {
                                  "min": -1.7976931348623157e+308,
                                  "max": 1.7976931348623157e+308,
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "coupon_disc",
                          "statistics": {
                            "name": "Float",
                            "float": {
                              "distribution": {
                                "double": {
                                  "min": -1.7976931348623157e+308,
                                  "max": 1.7976931348623157e+308,
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "coupon_match_disc",
                          "statistics": {
                            "name": "Float",
                            "float": {
                              "distribution": {
                                "double": {
                                  "min": -1.7976931348623157e+308,
                                  "max": 1.7976931348623157e+308,
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "week",
                          "statistics": {
                            "name": "Integer",
                            "integer": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        },
                        {
                          "name": "transaction_timestamp",
                          "statistics": {
                            "name": "Datetime",
                            "datetime": {
                              "distribution": {
                                "integer": {
                                  "min": "-9223372036854775808",
                                  "max": "9223372036854775807",
                                  "points": []
                                },
                                "properties": {}
                              },
                              "size": "1469283",
                              "multiplicity": 0.0
                            },
                            "properties": {}
                          }
                        }
                      ],
                      "size": "1469283",
                      "name": "",
                      "multiplicity": 0.0
                    }
                  }
                }
              ],
              "size": "6422906",
              "name": "",
              "multiplicity": 0.0
            }
          }
        }
      ],
      "size": "6422906",
      "name": "",
      "multiplicity": 0.0
    }
  },
  "properties": {}
}
"#;