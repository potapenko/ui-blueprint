# F01 saved form and popup observation

Guide: UIB.DRAWING@1.1. Purpose: Document.

## Identity, audience, approval, owner and retention

```json
{
  "document_id": "F01-D05-OBSERVED",
  "revision": "1",
  "title": "F01 saved form and popup observation",
  "audience": "Клиент и разработчик",
  "language": "ru",
  "date": "2026-10-06",
  "owner": "E01 fixture maintainer",
  "retention": "Until fixture contract is superseded",
  "specification_refs": [
    "UIB.DRAWING@1.1",
    "F01 D05 Web sample fcf48be"
  ],
  "approval": {
    "status": "draft",
    "named_record": null
  },
  "page_format": "A3 proportions",
  "output_size": "3840 x 2160 output pixels; UI units unchanged"
}
```

## Scope, coverage, environment, named states, source, components, relations and unknowns

```json
{
  "guide": "UIB.DRAWING@1.1",
  "views": [
    {
      "snapshot_ref": "REF0001",
      "snapshot_revision": 1,
      "id": "observed",
      "title": "F01 form and popup",
      "source_kind": "observed",
      "source": "F01 D05 Web sample fcf48be; original channel SHA256 11e2a77cf6460636050e90ed0f6f63bcf3412c062bb3eb917d677be035858180",
      "state": "overlay-on",
      "scope": "f01-d05-form-popup; all 32 returned source nodes; partial selected scope and fields",
      "environment": "historical own F01 fixture; Chromium; 800 x 600 CSS viewport; DPR 1; external semantics; not PlayPhrase.me",
      "coverage": {
        "status": "partial",
        "scope_id": "REF0034",
        "fields": [
          "role",
          "accessibility_name",
          "layout_bounds",
          "hit_region",
          "visible_region"
        ],
        "omitted_count": 0,
        "unknown_count": null
      },
      "surfaces": [
        {
          "identity": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "native_owner": {
            "availability": "known",
            "value": {
              "type": "identity",
              "value": {
                "id": "REF0002",
                "generation": "REF0032"
              }
            }
          },
          "initiated_by": null,
          "anchor": null,
          "evidence": {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0033",
            "uncertainty": null
          }
        }
      ],
      "observations": [
        {
          "id": "REF0006",
          "source_namespace": "REF0004",
          "channel": "external_semantics",
          "start": 1443.29475,
          "end": 1463.6775,
          "clock_domain": "REF0035",
          "time_unit": "milliseconds",
          "freshness_basis": "live_read",
          "consistency_reason": "REF0036",
          "answer_source": "live",
          "freshness": "current",
          "last_verified": 1463.6775,
          "consistency": "unknown",
          "coverage": {
            "status": "partial",
            "scope_id": "REF0034",
            "fields": [
              "role",
              "accessibility_name",
              "layout_bounds",
              "hit_region",
              "visible_region"
            ],
            "omitted_count": 0,
            "unknown_count": null
          }
        },
        {
          "id": "REF0015",
          "source_namespace": "REF0014",
          "channel": "external_semantics",
          "start": 1463.677625,
          "end": 1474.420625,
          "clock_domain": "REF0035",
          "time_unit": "milliseconds",
          "freshness_basis": "live_read",
          "consistency_reason": "REF0036",
          "answer_source": "live",
          "freshness": "current",
          "last_verified": 1474.420625,
          "consistency": "unknown",
          "coverage": {
            "status": "partial",
            "scope_id": "REF0034",
            "fields": [
              "role",
              "accessibility_name",
              "layout_bounds",
              "hit_region",
              "visible_region"
            ],
            "omitted_count": 0,
            "unknown_count": null
          }
        }
      ],
      "not_depicted": [
        "Hidden or uncollected UI; partial coverage, unknown_count unknown",
        "Source identifiers pseudonymized; geometry and observation intervals preserved",
        "No screenshot, no fresh measurement, no real-site acceptance",
        "Pixels and unreviewed text are excluded; stored observation, not a new measurement"
      ],
      "components": [
        {
          "id": "N000",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0005"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "BUTTON"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "button"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 477.234375,
                        "y": 111.0,
                        "width": 97.296875,
                        "height": 32.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N001",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0005"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "button"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "button"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": "Open options"
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N002",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0017"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "DIV"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "dialog"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 400.0,
                        "y": 290.0,
                        "width": 200.0,
                        "height": 60.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N003",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0017"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "dialog"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "dialog"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": "Options"
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N004",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0018"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "BUTTON"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "button"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 400.0,
                        "y": 290.0,
                        "width": 98.78125,
                        "height": 32.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N005",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0018"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "button"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "button"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": "Close options"
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N006",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0019"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "BUTTON"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "button"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 40.0,
                        "y": 60.0,
                        "width": 120.0,
                        "height": 40.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N007",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0019"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "button"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "button"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": "Apply"
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N008",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0020"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "DIV"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 40.0,
                        "y": 60.0,
                        "width": 120.0,
                        "height": 40.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N009",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0020"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "generic"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": ""
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N010",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0021"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "DIV"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 40.0,
                        "y": 400.0,
                        "width": 100.0,
                        "height": 40.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N011",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0021"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "generic"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": ""
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N012",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0022"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "DIV"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 120.0,
                        "y": 400.0,
                        "width": 60.0,
                        "height": 30.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N013",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0022"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "generic"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": ""
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N014",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0023"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "SECTION"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 380.0,
                        "y": 20.0,
                        "width": 360.0,
                        "height": 123.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N015",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0023"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "region"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": "Fixture form"
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N016",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0024"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "INPUT"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 380.0,
                        "y": 42.0,
                        "width": 188.0,
                        "height": 21.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N017",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0024"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "combobox"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": "City"
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N018",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0025"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "DIV"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 380.0,
                        "y": 67.0,
                        "width": 360.0,
                        "height": 22.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N019",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0025"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "listbox"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": ""
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N020",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0026"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "DIV"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 380.0,
                        "y": 89.0,
                        "width": 360.0,
                        "height": 22.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N021",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0026"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "status"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": ""
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N022",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0027"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "BUTTON"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "button"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 380.0,
                        "y": 111.0,
                        "width": 87.625,
                        "height": 32.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N023",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0027"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "button"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "button"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": "Commit city"
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N024",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0028"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "OUTPUT"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 477.234375,
                        "y": 117.5,
                        "width": 0.0,
                        "height": 18.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N025",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0028"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "status"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": ""
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N026",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0029"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "BUTTON"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "button"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 584.140625,
                        "y": 111.0,
                        "width": 146.9375,
                        "height": 32.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N027",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0029"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "button"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "button"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": "Unexpected transition"
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N028",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0030"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "DIV"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 660.0,
                        "y": 120.0,
                        "width": 100.0,
                        "height": 30.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N029",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0030"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "generic"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": ""
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N030",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0004",
            "key": "REF0031"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "DIV"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0009",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "geometry",
                  "value": {
                    "frame_kind": "layout_bounds",
                    "coordinate_space": {
                      "id": "REF0010",
                      "kind": "viewport",
                      "units": "css_px",
                      "origin": "top_left"
                    },
                    "shape": {
                      "shape": "rect",
                      "value": {
                        "x": 40.0,
                        "y": 480.0,
                        "width": 32.0,
                        "height": 16.0
                      }
                    },
                    "transform": {
                      "status": "local_only"
                    }
                  }
                }
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0006",
                "source_namespace": "REF0004",
                "provenance": "reported",
                "method": "REF0007",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        },
        {
          "id": "N031",
          "surface": {
            "id": "REF0002",
            "generation": "REF0003"
          },
          "source_key": {
            "namespace": "REF0014",
            "key": "REF0031"
          },
          "native_role": {
            "availability": "known",
            "value": {
              "type": "text",
              "value": "generic"
            }
          },
          "parent": null,
          "children": [],
          "role": null,
          "label": null,
          "properties": [
            {
              "selection": "requested",
              "field": "role",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "role",
                  "value": "unknown"
                }
              }
            },
            {
              "selection": "requested",
              "field": "accessibility_name",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "known",
                "value": {
                  "type": "text",
                  "value": ""
                }
              }
            },
            {
              "selection": "requested",
              "field": "layout_bounds",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "hit_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            },
            {
              "selection": "requested",
              "field": "visible_region",
              "sensitivity": "public",
              "evidence": {
                "observation_id": "REF0015",
                "source_namespace": "REF0014",
                "provenance": "reported",
                "method": "REF0016",
                "uncertainty": null
              },
              "state": {
                "availability": "unknown",
                "reason": "REF0008"
              }
            }
          ],
          "geometry": null,
          "declarations": [
            {
              "namespace": "REF0011",
              "name": "REF0012",
              "state": {
                "availability": "redacted"
              },
              "sensitivity": "public",
              "source": "REF0013"
            }
          ],
          "extensions": [],
          "state_and_actions": null
        }
      ],
      "relations": [
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0005"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0005"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0017"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0017"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0018"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0018"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0019"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0019"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0020"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0020"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0021"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0021"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0022"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0022"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0023"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0023"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0024"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0024"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0025"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0025"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0026"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0026"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0027"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0027"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0028"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0028"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0029"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0029"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0030"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0030"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "corresponds_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0031"
          },
          "to": {
            "namespace": "REF0014",
            "key": "REF0031"
          },
          "evidence": {
            "observation_id": "REF0015",
            "source_namespace": "REF0014",
            "provenance": "reported",
            "method": "REF0037",
            "uncertainty": null
          }
        },
        {
          "kind": "anchored_to",
          "from": {
            "namespace": "REF0004",
            "key": "REF0017"
          },
          "to": {
            "namespace": "REF0004",
            "key": "REF0005"
          },
          "evidence": {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0038",
            "uncertainty": null
          }
        }
      ],
      "mappings": [],
      "focus": {
        "keyboard": {
          "status": "not_requested"
        },
        "accessibility": {
          "status": "not_requested"
        },
        "active_descendant": {
          "status": "not_requested"
        },
        "text_selection": null,
        "composition_state": {
          "selection": "not_requested",
          "field": "value"
        }
      },
      "requirements": [],
      "unknowns": [
        "Unavailable properties retain unknown, unsupported, redacted or not_requested. Padding, radius and baseline are unknown unless explicitly reported."
      ]
    }
  ],
  "flow": [],
  "comparisons": []
}
```

## Dimensions, anchors, units, evidence and derived chains

```json
[
  {
    "view": "observed",
    "dimensions": [
      {
        "id": "M0001",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N000",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N000",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0002",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N000",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N000",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0003",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N002",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N002",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0004",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N002",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N002",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0005",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N004",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N004",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0006",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N004",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N004",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0007",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N006",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N006",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0008",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N006",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N006",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0009",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N008",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N008",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0010",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N008",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N008",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0011",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N010",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N010",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0012",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N010",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N010",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0013",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N012",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N012",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0014",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N012",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N012",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0015",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N014",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N014",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0016",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N014",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N014",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0017",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N016",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N016",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0018",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N016",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N016",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0019",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N018",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N018",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0020",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N018",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N018",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0021",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N020",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N020",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0022",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N020",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N020",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0023",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N022",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N022",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0024",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N022",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N022",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0025",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N024",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N024",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0026",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N024",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N024",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0027",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N026",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N026",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0028",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N026",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N026",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0029",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N028",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N028",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0030",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N028",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N028",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0031",
        "label": "Width LayoutBounds",
        "anchors": [
          {
            "component": "N030",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N030",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      },
      {
        "id": "M0032",
        "label": "Height LayoutBounds",
        "anchors": [
          {
            "component": "N030",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N030",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "REF0010",
              "kind": "viewport",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": null,
        "units": "css_px",
        "source_kind": "observed",
        "evidence": [
          {
            "observation_id": "REF0006",
            "source_namespace": "REF0004",
            "provenance": "reported",
            "method": "REF0009",
            "uncertainty": null
          }
        ],
        "requirement_ref": null,
        "unknown_reason": "unstable_state",
        "check_tolerance": null
      }
    ],
    "chains": []
  }
]
```

## Sheet plan

```json
[
  {
    "id": "G01",
    "view": "observed",
    "title": "F01 form and popup",
    "kind": "general",
    "parent_view": null,
    "components": [
      "N000",
      "N001",
      "N002",
      "N003",
      "N004",
      "N005",
      "N006",
      "N007",
      "N008",
      "N009",
      "N010",
      "N011",
      "N012",
      "N013",
      "N014",
      "N015",
      "N016",
      "N017",
      "N018",
      "N019",
      "N020",
      "N021",
      "N022",
      "N023",
      "N024",
      "N025",
      "N026",
      "N027",
      "N028",
      "N029",
      "N030",
      "N031"
    ],
    "placement": "central full scope; external dimension margins; legend; title block bottom right",
    "state": "overlay-on",
    "units": [
      "css_px"
    ]
  },
  {
    "id": "D01",
    "view": "observed",
    "title": "Scope detail; preserve every listed object",
    "kind": "detail",
    "parent_view": "G01",
    "components": [
      "N000",
      "N001",
      "N002",
      "N003",
      "N004",
      "N005",
      "N006",
      "N007",
      "N008",
      "N009",
      "N010",
      "N011"
    ],
    "placement": "separate enlarged view; reference to general view is not a user action",
    "state": "overlay-on",
    "units": [
      "css_px"
    ]
  },
  {
    "id": "D02",
    "view": "observed",
    "title": "Scope detail; preserve every listed object",
    "kind": "detail",
    "parent_view": "G01",
    "components": [
      "N012",
      "N013",
      "N014",
      "N015",
      "N016",
      "N017",
      "N018",
      "N019",
      "N020",
      "N021",
      "N022",
      "N023"
    ],
    "placement": "separate enlarged view; reference to general view is not a user action",
    "state": "overlay-on",
    "units": [
      "css_px"
    ]
  },
  {
    "id": "D03",
    "view": "observed",
    "title": "Scope detail; preserve every listed object",
    "kind": "detail",
    "parent_view": "G01",
    "components": [
      "N024",
      "N025",
      "N026",
      "N027",
      "N028",
      "N029",
      "N030",
      "N031"
    ],
    "placement": "separate enlarged view; reference to general view is not a user action",
    "state": "overlay-on",
    "units": [
      "css_px"
    ]
  }
]
```

## Style, exact labels and forbidden changes

Blue Engineering: flat #0B5E9E background, white contours and text, subordinate grid, orthogonal front view, line hierarchy and title block. Use exact quoted labels as data. Preserve every selected object, relation, source, frame kind, unit and unknown. No invented controls, radius, padding, baseline, tolerance or geometry from pixels. Requirements remain proposed. Scale: schematic. Размеры по подписям; не измерять по изображению.

## Verification

Local references, arithmetic, units and explicit public-text policy checked. validation_status=unverified for the ungenerated image; approval is a separate supplied record. Generation is a separate user action. Review every ID, number, anchor, state, source, scope, readability, coverage and privacy against these files before marking an image checked. Never replace an accepted baseline with current runtime. No CAD-scale guarantee, model call, pixel reference, or runtime capture.
