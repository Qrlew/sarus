
pub const DATASET: &str = r#"
{
    "@type": "sarus_data_spec/sarus_data_spec.Dataset",
    "doc": "",
    "name": "Transformed",
    "properties": {},
    "spec": {
      "transformed": {
        "arguments": [
          "12f577275e57db1dced92c29dbce4c21"
        ],
        "named_arguments": {
          "attributes_budget": "de64d6785ec48bbcf716c5243861ac2a"
        },
        "transform": "b5bcf7dd7285658a083556fe94e3f42d"
      }
    },
    "uuid": "90d5acce1f9376d873c600dc29d55633"
}
"#;

pub const SCHEMA: &str = r#"
{
    "@type": "sarus_data_spec/sarus_data_spec.Schema",
    "dataset": "90d5acce1f9376d873c600dc29d55633",
    "name": "Transformed_schema",
    "properties": {
      "max_max_multiplicity": "1"
    },
    "protected": {
      "label": "data",
      "paths": [
        {
          "label": "census",
          "paths": [],
          "properties": {}
        }
      ],
      "properties": {}
    },
    "type": {
      "name": "Struct",
      "properties": {},
      "struct": {
        "fields": [
          {
            "name": "data",
            "type": {
              "name": "Union",
              "properties": {
                "sarus_is_public": "False"
              },
              "union": {
                "fields": [
                  {
                    "name": "census",
                    "type": {
                      "name": "Struct",
                      "properties": {
                        "merge_paths": "0",
                        "sarus_is_public": "False"
                      },
                      "struct": {
                        "fields": [
                          {
                            "name": "age",
                            "type": {
                              "name": "Optional",
                              "optional": {
                                "type": {
                                  "integer": {
                                    "base": "INT64",
                                    "max": "90",
                                    "min": "20",
                                    "possible_values": [
                                      "20",
                                      "21",
                                      "22",
                                      "23",
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
                                      "78",
                                      "81",
                                      "82",
                                      "83",
                                      "90"
                                    ]
                                  },
                                  "name": "Integer",
                                  "properties": {}
                                }
                              },
                              "properties": {}
                            }
                          },
                          {
                            "name": "workclass",
                            "type": {
                              "name": "Text UTF-8",
                              "properties": {
                                "max_length": "16",
                                "min_length": "1"
                              },
                              "text": {
                                "encoding": "UTF-8",
                                "possible_values": [
                                  "?",
                                  "Federal-gov",
                                  "Local-gov",
                                  "Private",
                                  "Self-emp-inc",
                                  "Self-emp-not-inc",
                                  "State-gov"
                                ]
                              }
                            }
                          },
                          {
                            "name": "fnlwgt",
                            "type": {
                              "name": "Text UTF-8",
                              "properties": {
                                "max_length": "6",
                                "min_length": "5"
                              },
                              "text": {
                                "encoding": "UTF-8",
                                "possible_values": [
                                  "100820",
                                  "101978",
                                  "102308",
                                  "102346",
                                  "102359",
                                  "102942",
                                  "105252",
                                  "105478",
                                  "106175",
                                  "107164",
                                  "107276",
                                  "107287",
                                  "110380",
                                  "110862",
                                  "114580",
                                  "115066",
                                  "115806",
                                  "118902",
                                  "119592",
                                  "121253",
                                  "121362",
                                  "121441",
                                  "122066",
                                  "123011",
                                  "124137",
                                  "124449",
                                  "124930",
                                  "129177",
                                  "129460",
                                  "132222",
                                  "132870",
                                  "135285",
                                  "135296",
                                  "136204",
                                  "137527",
                                  "138107",
                                  "140359",
                                  "141058",
                                  "141584",
                                  "143123",
                                  "143833",
                                  "145290",
                                  "145933",
                                  "147372",
                                  "147707",
                                  "148995",
                                  "149650",
                                  "150601",
                                  "151089",
                                  "152307",
                                  "152569",
                                  "153183",
                                  "153870",
                                  "154374",
                                  "155106",
                                  "155141",
                                  "156516",
                                  "156897",
                                  "156996",
                                  "157145",
                                  "158685",
                                  "158702",
                                  "160369",
                                  "160724",
                                  "161691",
                                  "162028",
                                  "162140",
                                  "162613",
                                  "162945",
                                  "164526",
                                  "166295",
                                  "167793",
                                  "171328",
                                  "172175",
                                  "172274",
                                  "172822",
                                  "174995",
                                  "175070",
                                  "175360",
                                  "176046",
                                  "176185",
                                  "177305",
                                  "177408",
                                  "178510",
                                  "179731",
                                  "181655",
                                  "186061",
                                  "187702",
                                  "187870",
                                  "187901",
                                  "188044",
                                  "188774",
                                  "191712",
                                  "191765",
                                  "191978",
                                  "192052",
                                  "192779",
                                  "192923",
                                  "192963",
                                  "194901",
                                  "197163",
                                  "198654",
                                  "198759",
                                  "198863",
                                  "199029",
                                  "199114",
                                  "201742",
                                  "202699",
                                  "203034",
                                  "205246",
                                  "207668",
                                  "207938",
                                  "208577",
                                  "211287",
                                  "214955",
                                  "216864",
                                  "218490",
                                  "218637",
                                  "222405",
                                  "226355",
                                  "227856",
                                  "228696",
                                  "228921",
                                  "233882",
                                  "235882",
                                  "237608",
                                  "237713",
                                  "237729",
                                  "240857",
                                  "251905",
                                  "252752",
                                  "25322",
                                  "257269",
                                  "257295",
                                  "258298",
                                  "263561",
                                  "264663",
                                  "269417",
                                  "27187",
                                  "279015",
                                  "279833",
                                  "285408",
                                  "287983",
                                  "288825",
                                  "289669",
                                  "29059",
                                  "29591",
                                  "297248",
                                  "298449",
                                  "30529",
                                  "313243",
                                  "317847",
                                  "326232",
                                  "326857",
                                  "327825",
                                  "329980",
                                  "33310",
                                  "335549",
                                  "336007",
                                  "336188",
                                  "34310",
                                  "346478",
                                  "346635",
                                  "348521",
                                  "35576",
                                  "358893",
                                  "36364",
                                  "36385",
                                  "36671",
                                  "370767",
                                  "370890",
                                  "377931",
                                  "381789",
                                  "39181",
                                  "396745",
                                  "41108",
                                  "422013",
                                  "42402",
                                  "427422",
                                  "43221",
                                  "43554",
                                  "44064",
                                  "45363",
                                  "456062",
                                  "52138",
                                  "56248",
                                  "66624",
                                  "68898",
                                  "70037",
                                  "77009",
                                  "77053",
                                  "77071",
                                  "81413",
                                  "81929",
                                  "88638",
                                  "92431",
                                  "99131"
                                ]
                              }
                            }
                          },
                          {
                            "name": "education",
                            "type": {
                              "name": "Text UTF-8",
                              "properties": {
                                "max_length": "12",
                                "min_length": "3"
                              },
                              "text": {
                                "encoding": "UTF-8",
                                "possible_values": [
                                  "10th",
                                  "11th",
                                  "12th",
                                  "1st-4th",
                                  "5th-6th",
                                  "7th-8th",
                                  "9th",
                                  "Assoc-acdm",
                                  "Assoc-voc",
                                  "Bachelors",
                                  "Doctorate",
                                  "HS-grad",
                                  "Masters",
                                  "Prof-school",
                                  "Some-college"
                                ]
                              }
                            }
                          },
                          {
                            "name": "education_num",
                            "type": {
                              "integer": {
                                "base": "INT64",
                                "max": "16",
                                "min": "2",
                                "possible_values": [
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
                                  "16"
                                ]
                              },
                              "name": "Integer",
                              "properties": {}
                            }
                          },
                          {
                            "name": "marital_status",
                            "type": {
                              "name": "Optional",
                              "optional": {
                                "type": {
                                  "name": "Text UTF-8",
                                  "properties": {
                                    "max_length": "21",
                                    "min_length": "7"
                                  },
                                  "text": {
                                    "encoding": "UTF-8",
                                    "possible_values": [
                                      "Divorced",
                                      "Married-civ-spouse",
                                      "Married-spouse-absent",
                                      "Never-married",
                                      "Separated",
                                      "Widowed"
                                    ]
                                  }
                                }
                              },
                              "properties": {}
                            }
                          },
                          {
                            "name": "occupation",
                            "type": {
                              "name": "Text UTF-8",
                              "properties": {
                                "max_length": "17",
                                "min_length": "1"
                              },
                              "text": {
                                "encoding": "UTF-8",
                                "possible_values": [
                                  "?",
                                  "Adm-clerical",
                                  "Craft-repair",
                                  "Exec-managerial",
                                  "Farming-fishing",
                                  "Handlers-cleaners",
                                  "Machine-op-inspct",
                                  "Other-service",
                                  "Prof-specialty",
                                  "Protective-serv",
                                  "Sales",
                                  "Tech-support",
                                  "Transport-moving"
                                ]
                              }
                            }
                          },
                          {
                            "name": "relationship",
                            "type": {
                              "name": "Text UTF-8",
                              "properties": {
                                "max_length": "14",
                                "min_length": "4"
                              },
                              "text": {
                                "encoding": "UTF-8",
                                "possible_values": [
                                  "Husband",
                                  "Not-in-family",
                                  "Other-relative",
                                  "Own-child",
                                  "Unmarried",
                                  "Wife"
                                ]
                              }
                            }
                          },
                          {
                            "name": "race",
                            "type": {
                              "name": "Text UTF-8",
                              "properties": {
                                "max_length": "18",
                                "min_length": "5"
                              },
                              "text": {
                                "encoding": "UTF-8",
                                "possible_values": [
                                  "Asian-Pac-Islander",
                                  "Black",
                                  "White"
                                ]
                              }
                            }
                          },
                          {
                            "name": "sex",
                            "type": {
                              "name": "Text UTF-8",
                              "properties": {
                                "max_length": "6",
                                "min_length": "4"
                              },
                              "text": {
                                "encoding": "UTF-8",
                                "possible_values": [
                                  "Female",
                                  "Male"
                                ]
                              }
                            }
                          },
                          {
                            "name": "capital_gain",
                            "type": {
                              "integer": {
                                "base": "INT64",
                                "max": "0",
                                "min": "0",
                                "possible_values": [
                                  "0"
                                ]
                              },
                              "name": "Integer",
                              "properties": {}
                            }
                          },
                          {
                            "name": "capital_loss",
                            "type": {
                              "integer": {
                                "base": "INT64",
                                "max": "4356",
                                "min": "2231",
                                "possible_values": [
                                  "2231",
                                  "2238",
                                  "2246",
                                  "2258",
                                  "2267",
                                  "2282",
                                  "2339",
                                  "2352",
                                  "2377",
                                  "2392",
                                  "2415",
                                  "2444",
                                  "2457",
                                  "2467",
                                  "2472",
                                  "2489",
                                  "2547",
                                  "2559",
                                  "2603",
                                  "2754",
                                  "2824",
                                  "3004",
                                  "3683",
                                  "3770",
                                  "3900",
                                  "4356"
                                ]
                              },
                              "name": "Integer",
                              "properties": {}
                            }
                          },
                          {
                            "name": "hours_per_week",
                            "type": {
                              "integer": {
                                "base": "INT64",
                                "max": "99",
                                "min": "6",
                                "possible_values": [
                                  "6",
                                  "8",
                                  "10",
                                  "12",
                                  "15",
                                  "18",
                                  "20",
                                  "25",
                                  "26",
                                  "28",
                                  "30",
                                  "32",
                                  "35",
                                  "36",
                                  "38",
                                  "39",
                                  "40",
                                  "42",
                                  "44",
                                  "45",
                                  "48",
                                  "50",
                                  "52",
                                  "55",
                                  "60",
                                  "62",
                                  "65",
                                  "67",
                                  "70",
                                  "72",
                                  "75",
                                  "76",
                                  "80",
                                  "84",
                                  "90",
                                  "99"
                                ]
                              },
                              "name": "Integer",
                              "properties": {}
                            }
                          },
                          {
                            "name": "native_country",
                            "type": {
                              "name": "Text UTF-8",
                              "properties": {
                                "max_length": "15",
                                "min_length": "1"
                              },
                              "text": {
                                "encoding": "UTF-8",
                                "possible_values": [
                                  "?",
                                  "Canada",
                                  "China",
                                  "Greece",
                                  "India",
                                  "Mexico",
                                  "Philippines",
                                  "South",
                                  "Taiwan",
                                  "Trinadad&Tobago",
                                  "United-States",
                                  "Vietnam"
                                ]
                              }
                            }
                          },
                          {
                            "name": "income",
                            "type": {
                              "name": "Text UTF-8",
                              "properties": {
                                "max_length": "5",
                                "min_length": "4"
                              },
                              "text": {
                                "encoding": "UTF-8",
                                "possible_values": [
                                  "<=50K",
                                  ">50K"
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
            "name": "sarus_is_public",
            "type": {
              "boolean": {},
              "name": "Boolean",
              "properties": {}
            }
          },
          {
            "name": "sarus_protected_entity",
            "type": {
              "name": "Optional",
              "optional": {
                "type": {
                  "id": {
                    "base": "STRING",
                    "unique": false
                  },
                  "name": "Id",
                  "properties": {}
                }
              },
              "properties": {}
            }
          },
          {
            "name": "sarus_weights",
            "type": {
              "float": {
                "base": "FLOAT64",
                "max": 1.7976931348623157e+308,
                "min": 0.0,
                "possible_values": []
              },
              "name": "Float64",
              "properties": {}
            }
          }
        ]
      }
    },
    "uuid": "ea5000d26739ee6f1be286cd0eb17ee9"
  }
"#;

pub const SIZE: &str = r#"
{
    "@type": "sarus_data_spec/sarus_data_spec.Size",
    "dataset": "90d5acce1f9376d873c600dc29d55633",
    "name": "Transformed_sizes",
    "properties": {},
    "statistics": {
      "name": "Union",
      "properties": {},
      "union": {
        "fields": [
          {
            "name": "census",
            "statistics": {
              "name": "Struct",
              "properties": {},
              "struct": {
                "fields": [
                  {
                    "name": "age",
                    "statistics": {
                      "name": "Optional",
                      "optional": {
                        "multiplicity": 1.0,
                        "name": "",
                        "size": "202",
                        "statistics": {
                          "integer": {
                            "distribution": {
                              "integer": {
                                "max": "9223372036854775807",
                                "min": "-9223372036854775808",
                                "points": []
                              },
                              "properties": {}
                            },
                            "multiplicity": 1.0,
                            "size": "189"
                          },
                          "name": "Integer",
                          "properties": {}
                        }
                      },
                      "properties": {}
                    }
                  },
                  {
                    "name": "capital_gain",
                    "statistics": {
                      "integer": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "multiplicity": 1.0,
                        "size": "202"
                      },
                      "name": "Integer",
                      "properties": {}
                    }
                  },
                  {
                    "name": "capital_loss",
                    "statistics": {
                      "integer": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "multiplicity": 1.0,
                        "size": "202"
                      },
                      "name": "Integer",
                      "properties": {}
                    }
                  },
                  {
                    "name": "education",
                    "statistics": {
                      "name": "Text",
                      "properties": {},
                      "text": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "example": "",
                        "multiplicity": 1.0,
                        "size": "202"
                      }
                    }
                  },
                  {
                    "name": "education_num",
                    "statistics": {
                      "integer": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "multiplicity": 1.0,
                        "size": "202"
                      },
                      "name": "Integer",
                      "properties": {}
                    }
                  },
                  {
                    "name": "fnlwgt",
                    "statistics": {
                      "name": "Text",
                      "properties": {},
                      "text": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "example": "",
                        "multiplicity": 1.0,
                        "size": "202"
                      }
                    }
                  },
                  {
                    "name": "hours_per_week",
                    "statistics": {
                      "integer": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "multiplicity": 1.0,
                        "size": "202"
                      },
                      "name": "Integer",
                      "properties": {}
                    }
                  },
                  {
                    "name": "income",
                    "statistics": {
                      "name": "Text",
                      "properties": {},
                      "text": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "example": "",
                        "multiplicity": 1.0,
                        "size": "202"
                      }
                    }
                  },
                  {
                    "name": "marital_status",
                    "statistics": {
                      "name": "Optional",
                      "optional": {
                        "multiplicity": 1.0,
                        "name": "",
                        "size": "202",
                        "statistics": {
                          "name": "Text",
                          "properties": {},
                          "text": {
                            "distribution": {
                              "integer": {
                                "max": "9223372036854775807",
                                "min": "-9223372036854775808",
                                "points": []
                              },
                              "properties": {}
                            },
                            "example": "",
                            "multiplicity": 1.0,
                            "size": "167"
                          }
                        }
                      },
                      "properties": {}
                    }
                  },
                  {
                    "name": "native_country",
                    "statistics": {
                      "name": "Text",
                      "properties": {},
                      "text": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "example": "",
                        "multiplicity": 1.0,
                        "size": "202"
                      }
                    }
                  },
                  {
                    "name": "occupation",
                    "statistics": {
                      "name": "Text",
                      "properties": {},
                      "text": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "example": "",
                        "multiplicity": 1.0,
                        "size": "202"
                      }
                    }
                  },
                  {
                    "name": "race",
                    "statistics": {
                      "name": "Text",
                      "properties": {},
                      "text": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "example": "",
                        "multiplicity": 1.0,
                        "size": "202"
                      }
                    }
                  },
                  {
                    "name": "relationship",
                    "statistics": {
                      "name": "Text",
                      "properties": {},
                      "text": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "example": "",
                        "multiplicity": 1.0,
                        "size": "202"
                      }
                    }
                  },
                  {
                    "name": "sex",
                    "statistics": {
                      "name": "Text",
                      "properties": {},
                      "text": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "example": "",
                        "multiplicity": 1.0,
                        "size": "202"
                      }
                    }
                  },
                  {
                    "name": "workclass",
                    "statistics": {
                      "name": "Text",
                      "properties": {},
                      "text": {
                        "distribution": {
                          "integer": {
                            "max": "9223372036854775807",
                            "min": "-9223372036854775808",
                            "points": []
                          },
                          "properties": {}
                        },
                        "example": "",
                        "multiplicity": 1.0,
                        "size": "202"
                      }
                    }
                  }
                ],
                "multiplicity": 1.0,
                "name": "",
                "size": "202"
              }
            }
          }
        ],
        "multiplicity": 1.0,
        "name": "",
        "size": "202"
      }
    },
    "uuid": "ecf6fddd1bcc811f5f28666d9ed8a981"
  }
"#;