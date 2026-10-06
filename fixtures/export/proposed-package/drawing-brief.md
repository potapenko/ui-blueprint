# Заявка — синтетический проект

Guide: UIB.DRAWING@1.1. Purpose: Propose.

## Identity, audience, approval, owner and retention

```json
{
  "document_id": "DRAWING-EXAMPLE",
  "revision": "1",
  "title": "Заявка — синтетический проект",
  "audience": "Клиент и разработчик",
  "language": "ru",
  "date": "2026-10-06",
  "owner": "E01 fixture maintainer",
  "retention": "Until fixture contract is superseded",
  "specification_refs": [
    "UIB.DRAWING@1.1",
    "UIB.DRAWING-EXAMPLE@1"
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
      "snapshot_ref": null,
      "snapshot_revision": null,
      "id": "proposal",
      "title": "Заявка",
      "source_kind": "proposed",
      "source": "UIB.DRAWING-EXAMPLE@1",
      "state": "proposed default",
      "scope": "complete proposed 1200 x 800 surface",
      "environment": "synthetic canvas-local; css_px; no runtime",
      "coverage": null,
      "surfaces": [],
      "observations": [],
      "not_depicted": [
        "Радиус неизвестен; не придумывать R8",
        "Не описывает PlayPhrase.me; не измерение runtime"
      ],
      "components": [
        {
          "id": "N00",
          "surface": null,
          "source_key": null,
          "native_role": null,
          "parent": null,
          "children": [
            "N01",
            "N02",
            "N03"
          ],
          "role": "group",
          "label": "Поверхность",
          "properties": [],
          "geometry": {
            "frame_kind": "layout_bounds",
            "coordinate_space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "shape": {
              "shape": "rect",
              "value": {
                "x": 0.0,
                "y": 0.0,
                "width": 1200.0,
                "height": 800.0
              }
            },
            "transform": {
              "status": "local_only"
            }
          },
          "declarations": [],
          "extensions": [],
          "state_and_actions": "Проект; runtime state and delivery unknown"
        },
        {
          "id": "N01",
          "surface": null,
          "source_key": null,
          "native_role": null,
          "parent": "N00",
          "children": [],
          "role": "heading",
          "label": "Заявка",
          "properties": [],
          "geometry": {
            "frame_kind": "layout_bounds",
            "coordinate_space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "shape": {
              "shape": "rect",
              "value": {
                "x": 24.0,
                "y": 24.0,
                "width": 1152.0,
                "height": 64.0
              }
            },
            "transform": {
              "status": "local_only"
            }
          },
          "declarations": [],
          "extensions": [],
          "state_and_actions": "Проект; runtime state and delivery unknown"
        },
        {
          "id": "N02",
          "surface": null,
          "source_key": null,
          "native_role": null,
          "parent": "N00",
          "children": [],
          "role": "group",
          "label": "Навигация",
          "properties": [],
          "geometry": {
            "frame_kind": "layout_bounds",
            "coordinate_space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "shape": {
              "shape": "rect",
              "value": {
                "x": 24.0,
                "y": 112.0,
                "width": 224.0,
                "height": 664.0
              }
            },
            "transform": {
              "status": "local_only"
            }
          },
          "declarations": [],
          "extensions": [],
          "state_and_actions": "Проект; runtime state and delivery unknown"
        },
        {
          "id": "N03",
          "surface": null,
          "source_key": null,
          "native_role": null,
          "parent": "N00",
          "children": [
            "N04",
            "N05",
            "N08"
          ],
          "role": "group",
          "label": "Основная область",
          "properties": [],
          "geometry": {
            "frame_kind": "layout_bounds",
            "coordinate_space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "shape": {
              "shape": "rect",
              "value": {
                "x": 272.0,
                "y": 112.0,
                "width": 904.0,
                "height": 664.0
              }
            },
            "transform": {
              "status": "local_only"
            }
          },
          "declarations": [],
          "extensions": [],
          "state_and_actions": "Проект; runtime state and delivery unknown"
        },
        {
          "id": "N04",
          "surface": null,
          "source_key": null,
          "native_role": null,
          "parent": "N03",
          "children": [],
          "role": "heading",
          "label": "Заголовок контента",
          "properties": [],
          "geometry": {
            "frame_kind": "layout_bounds",
            "coordinate_space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "shape": {
              "shape": "rect",
              "value": {
                "x": 296.0,
                "y": 136.0,
                "width": 856.0,
                "height": 32.0
              }
            },
            "transform": {
              "status": "local_only"
            }
          },
          "declarations": [],
          "extensions": [],
          "state_and_actions": "Проект; runtime state and delivery unknown"
        },
        {
          "id": "N05",
          "surface": null,
          "source_key": null,
          "native_role": null,
          "parent": "N03",
          "children": [
            "N06",
            "N07"
          ],
          "role": "form",
          "label": "Форма",
          "properties": [],
          "geometry": {
            "frame_kind": "layout_bounds",
            "coordinate_space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "shape": {
              "shape": "rect",
              "value": {
                "x": 296.0,
                "y": 192.0,
                "width": 856.0,
                "height": 456.0
              }
            },
            "transform": {
              "status": "local_only"
            }
          },
          "declarations": [],
          "extensions": [],
          "state_and_actions": "Проект; runtime state and delivery unknown"
        },
        {
          "id": "N06",
          "surface": null,
          "source_key": null,
          "native_role": null,
          "parent": "N05",
          "children": [],
          "role": "textbox",
          "label": "Имя",
          "properties": [],
          "geometry": {
            "frame_kind": "layout_bounds",
            "coordinate_space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "shape": {
              "shape": "rect",
              "value": {
                "x": 320.0,
                "y": 216.0,
                "width": 400.0,
                "height": 48.0
              }
            },
            "transform": {
              "status": "local_only"
            }
          },
          "declarations": [],
          "extensions": [],
          "state_and_actions": "Проект; runtime state and delivery unknown"
        },
        {
          "id": "N07",
          "surface": null,
          "source_key": null,
          "native_role": null,
          "parent": "N05",
          "children": [],
          "role": "textbox",
          "label": "Организация",
          "properties": [],
          "geometry": {
            "frame_kind": "layout_bounds",
            "coordinate_space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "shape": {
              "shape": "rect",
              "value": {
                "x": 728.0,
                "y": 216.0,
                "width": 400.0,
                "height": 48.0
              }
            },
            "transform": {
              "status": "local_only"
            }
          },
          "declarations": [],
          "extensions": [],
          "state_and_actions": "Проект; runtime state and delivery unknown"
        },
        {
          "id": "N08",
          "surface": null,
          "source_key": null,
          "native_role": null,
          "parent": "N03",
          "children": [
            "N09"
          ],
          "role": "group",
          "label": "Нижняя область действий",
          "properties": [],
          "geometry": {
            "frame_kind": "layout_bounds",
            "coordinate_space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "shape": {
              "shape": "rect",
              "value": {
                "x": 296.0,
                "y": 680.0,
                "width": 856.0,
                "height": 72.0
              }
            },
            "transform": {
              "status": "local_only"
            }
          },
          "declarations": [],
          "extensions": [],
          "state_and_actions": "Проект; runtime state and delivery unknown"
        },
        {
          "id": "N09",
          "surface": null,
          "source_key": null,
          "native_role": null,
          "parent": "N08",
          "children": [],
          "role": "button",
          "label": "Продолжить",
          "properties": [],
          "geometry": {
            "frame_kind": "layout_bounds",
            "coordinate_space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "shape": {
              "shape": "rect",
              "value": {
                "x": 1008.0,
                "y": 692.0,
                "width": 120.0,
                "height": 48.0
              }
            },
            "transform": {
              "status": "local_only"
            }
          },
          "declarations": [],
          "extensions": [],
          "state_and_actions": "Проект; runtime state and delivery unknown"
        }
      ],
      "relations": [],
      "mappings": [],
      "focus": null,
      "requirements": [
        "UIB.DRAWING-EXAMPLE@1 explicit target layout"
      ],
      "unknowns": [
        "radius unknown",
        "baseline unknown",
        "hit region unknown"
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
    "view": "proposal",
    "dimensions": [
      {
        "id": "N00-width",
        "label": "Целевая ширина",
        "anchors": [
          {
            "component": "N00",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N00",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 1200.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N00-height",
        "label": "Целевая высота",
        "anchors": [
          {
            "component": "N00",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N00",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": 800.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N01-width",
        "label": "Целевая ширина",
        "anchors": [
          {
            "component": "N01",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N01",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 1152.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N01-height",
        "label": "Целевая высота",
        "anchors": [
          {
            "component": "N01",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N01",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": 64.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N02-width",
        "label": "Целевая ширина",
        "anchors": [
          {
            "component": "N02",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N02",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 224.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N02-height",
        "label": "Целевая высота",
        "anchors": [
          {
            "component": "N02",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N02",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": 664.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N03-width",
        "label": "Целевая ширина",
        "anchors": [
          {
            "component": "N03",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N03",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 904.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N03-height",
        "label": "Целевая высота",
        "anchors": [
          {
            "component": "N03",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N03",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": 664.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N04-width",
        "label": "Целевая ширина",
        "anchors": [
          {
            "component": "N04",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N04",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 856.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N04-height",
        "label": "Целевая высота",
        "anchors": [
          {
            "component": "N04",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N04",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": 32.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N05-width",
        "label": "Целевая ширина",
        "anchors": [
          {
            "component": "N05",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N05",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 856.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N05-height",
        "label": "Целевая высота",
        "anchors": [
          {
            "component": "N05",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N05",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": 456.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N06-width",
        "label": "Целевая ширина",
        "anchors": [
          {
            "component": "N06",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N06",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 400.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N06-height",
        "label": "Целевая высота",
        "anchors": [
          {
            "component": "N06",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N06",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": 48.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N07-width",
        "label": "Целевая ширина",
        "anchors": [
          {
            "component": "N07",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N07",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 400.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N07-height",
        "label": "Целевая высота",
        "anchors": [
          {
            "component": "N07",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N07",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": 48.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N08-width",
        "label": "Целевая ширина",
        "anchors": [
          {
            "component": "N08",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N08",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 856.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N08-height",
        "label": "Целевая высота",
        "anchors": [
          {
            "component": "N08",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N08",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": 72.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N09-width",
        "label": "Целевая ширина",
        "anchors": [
          {
            "component": "N09",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N09",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 120.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "N09-height",
        "label": "Целевая высота",
        "anchors": [
          {
            "component": "N09",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N09",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          }
        ],
        "value": 48.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "outer-left",
        "label": "Внешний отступ",
        "anchors": [
          {
            "component": "N00",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N02",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          }
        ],
        "value": 24.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "column-gap",
        "label": "Промежуток колонок",
        "anchors": [
          {
            "component": "N02",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          },
          {
            "component": "N03",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          }
        ],
        "value": 24.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "outer-right",
        "label": "Внешний отступ",
        "anchors": [
          {
            "component": "N03",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          },
          {
            "component": "N00",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 24.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "vertical-nav",
        "label": "Вертикальный интервал",
        "anchors": [
          {
            "component": "N01",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          },
          {
            "component": "N02",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          }
        ],
        "value": 24.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "vertical-main",
        "label": "Вертикальный интервал",
        "anchors": [
          {
            "component": "N01",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "bottom"
          },
          {
            "component": "N03",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          }
        ],
        "value": 24.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "form-left",
        "label": "Проектный внутренний отступ",
        "anchors": [
          {
            "component": "N05",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          },
          {
            "component": "N06",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          }
        ],
        "value": 24.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "form-top",
        "label": "Проектный внутренний отступ",
        "anchors": [
          {
            "component": "N05",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N06",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          }
        ],
        "value": 24.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "field-gap",
        "label": "Промежуток полей",
        "anchors": [
          {
            "component": "N06",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          },
          {
            "component": "N07",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "left"
          }
        ],
        "value": 8.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "form-right",
        "label": "Проектный внутренний отступ",
        "anchors": [
          {
            "component": "N07",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          },
          {
            "component": "N05",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 24.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "button-right",
        "label": "Положение кнопки",
        "anchors": [
          {
            "component": "N09",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          },
          {
            "component": "N08",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "right"
          }
        ],
        "value": 24.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      },
      {
        "id": "button-top",
        "label": "Положение кнопки",
        "anchors": [
          {
            "component": "N08",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          },
          {
            "component": "N09",
            "frame_kind": "layout_bounds",
            "space": {
              "id": "canvas-local",
              "kind": "local",
              "units": "css_px",
              "origin": "top_left"
            },
            "edge": "top"
          }
        ],
        "value": 12.0,
        "units": "css_px",
        "source_kind": "proposed",
        "evidence": [],
        "requirement_ref": "UIB.DRAWING-EXAMPLE@1 explicit target layout",
        "unknown_reason": null,
        "check_tolerance": null
      }
    ],
    "chains": [
      {
        "terms": [
          "outer-left",
          "N02-width",
          "column-gap",
          "N03-width",
          "outer-right"
        ],
        "total": "N00-width",
        "arithmetic_tolerance": 0.0
      },
      {
        "terms": [
          "form-left",
          "N06-width",
          "field-gap",
          "N07-width",
          "form-right"
        ],
        "total": "N05-width",
        "arithmetic_tolerance": 0.0
      }
    ]
  }
]
```

## Sheet plan

```json
[
  {
    "id": "G01",
    "view": "proposal",
    "title": "Заявка",
    "kind": "general",
    "parent_view": null,
    "components": [
      "N00",
      "N01",
      "N02",
      "N03",
      "N04",
      "N05",
      "N06",
      "N07",
      "N08",
      "N09"
    ],
    "placement": "central full scope; external dimension margins; legend; title block bottom right",
    "state": "proposed default",
    "units": [
      "css_px"
    ]
  },
  {
    "id": "D01",
    "view": "proposal",
    "title": "D01 формы",
    "kind": "detail",
    "parent_view": "G01",
    "components": [
      "N05",
      "N06",
      "N07"
    ],
    "placement": "separate enlarged view; reference to general view is not a user action",
    "state": "proposed default",
    "units": [
      "css_px"
    ]
  },
  {
    "id": "D02",
    "view": "proposal",
    "title": "D02 кнопки",
    "kind": "detail",
    "parent_view": "G01",
    "components": [
      "N08",
      "N09"
    ],
    "placement": "separate enlarged view; reference to general view is not a user action",
    "state": "proposed default",
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
