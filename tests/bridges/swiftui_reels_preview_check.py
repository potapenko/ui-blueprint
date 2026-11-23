"""Adversarial saved-result checks for the R11 fixture experiment; no live IO."""
import copy
from pathlib import Path
import random
import sys
import unittest

sys.dont_write_bytecode=True
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/"experiments/native/reels-preview"))
from check_capture import compare
from capture_boundaries import require_fixture


def sample():
    def key(i): return {"namespace":"swiftui.ag_cache.layout","key":str(i)}
    target={"id":"macos-pid-123","generation":"123:5"}
    surface={"id":"native-window-9","generation":"observation"}
    context={"target":target,"environment_revision":"observation"}
    nodes=[];oracle={"pid":123,"window_id":9,"display_scale":2,"cells":{}}
    for i,name,children in ((0,"LazyVGrid",[1]),(1,"ResettableLazyLayoutRoot",[2]),(2,"Tree",list(range(3,24)))):
        nodes.append(dict(key=key(i),native_role={"value":{"value":name}},children=[key(k) for k in children]))
    for i in range(21):
        x,y=20+(i%7)*70,30+(i//7)*70
        space={"id":"local-"+str(i),"kind":"local","units":"pt","origin":"top_left"}
        mapping={"from":space,"to":{"id":"win","kind":"window","units":"pt","origin":"top_left"},
                 "target":target,"surface":surface,"environment_revision":"observation",
                 "affine":[1,0,0,1,0,32],"evidence":{"method":"source.NSView.convertPoint"}}
        g=dict(frame_kind="layout_bounds",coordinate_space=space,shape={"shape":"rect","value":dict(x=x,y=y,width=64,height=64)},
               transform={"status":"known","transform":mapping})
        nodes.append(dict(key=key(i+3),surface=surface,native_role={"value":{"value":"HiddenForReuseEffect"}},children=[],
            properties=[dict(field="layout_bounds",evidence={"provenance":"reported"},state={"availability":"known","value":{"value":g}})]))
        oracle["cells"][str(i)]={"window":[x,y+32,64,64]}
    manifest=dict(status="ready",cleanup="detached_restored",selection_provenance={"source_type":"R11PreviewFixture.FeatureGateSubscribePreview","selection_source":"body_owner"},
                  cache_audit={"model_range_reads":0,"evaluated_swiftui_or_graph":False})
    return {"artifact":{"data":{"context":context,"nodes":nodes}}},manifest,oracle,{"pid":123,"window_id":9}


class R11Checks(unittest.TestCase):
    def test_order_is_not_identity(self):
        d,m,o,r=sample();random.Random(13).shuffle(d["artifact"]["data"]["nodes"][2]["children"])
        result=compare(d,m,o,r)
        self.assertEqual(len(result["cells"]),21);self.assertTrue(result["exact_equality"])
        self.assertFalse(result["donor_admission"])

    def test_missing_or_duplicate_cells_refuse(self):
        for duplicate in (False,True):
            d,m,o,r=sample();children=d["artifact"]["data"]["nodes"][2]["children"]
            if duplicate:children[-1]=children[0]
            else:children.pop()
            with self.assertRaises(ValueError):compare(d,m,o,r)

    def test_dirty_layout_and_unknown_transform_refuse(self):
        for layout in (False,True):
            d,m,o,r=sample();p=d["artifact"]["data"]["nodes"][3]["properties"][0]
            if layout:p["state"]={"availability":"unknown","reason":"cache_value_dirty"}
            else:p["state"]["value"]["value"]["transform"]={"status":"unknown"}
            with self.assertRaises(ValueError):compare(d,m,o,r)

    def test_stale_window_or_target_refuse(self):
        for mode in ("pid","window","mapping"):
            d,m,o,r=sample()
            if mode=="pid":o["pid"]+=1
            elif mode=="window":o["window_id"]+=1
            else:d["artifact"]["data"]["nodes"][3]["properties"][0]["state"]["value"]["value"]["transform"]["transform"]["target"]={"id":"foreign"}
            with self.assertRaises(ValueError):compare(d,m,o,r)

    def test_wrong_grid_and_ambiguous_oracle_refuse(self):
        for mode in ("grid","oracle"):
            d,m,o,r=sample()
            if mode=="grid":d["artifact"]["data"]["nodes"].append(copy.deepcopy(d["artifact"]["data"]["nodes"][0]))
            else:o["cells"]["20"]=o["cells"]["0"]
            with self.assertRaises(ValueError):compare(d,m,o,r)

    def test_false_success_and_model_reads_refuse(self):
        for mode in ("hold","model","evaluation"):
            d,m,o,r=sample()
            if mode=="hold":m["cleanup"]="waiting_owner"
            elif mode=="model":m["cache_audit"]["model_range_reads"]=1
            else:m["cache_audit"]["evaluated_swiftui_or_graph"]=True
            with self.assertRaises(ValueError):compare(d,m,o,r)

    def test_pixel_rounding_is_bounded_and_not_exact(self):
        d,m,o,r=sample();o["cells"]["0"]["window"][0]+=.25
        result=compare(d,m,o,r);self.assertFalse(result["exact_equality"])
        self.assertEqual(result["max_abs_difference_pt"],.25)
        o["cells"]["0"]["window"][0]+=1
        with self.assertRaises(ValueError):compare(d,m,o,r)

    def test_fixture_harness_never_accepts_a_donor(self):
        scope={"type":"R11PreviewFixture.FeatureGateSubscribePreview","containing_type":"SwiftUI.LazyVGrid"}
        for bundle in ("me.playphraseme.reference.reels.fgb7o3fd","me.uiblueprint.external-layout.canary.unrelated"):
            with self.assertRaises(ValueError):require_fixture({"expected_bundle":bundle,"scope":scope})


if __name__=="__main__":unittest.main()
