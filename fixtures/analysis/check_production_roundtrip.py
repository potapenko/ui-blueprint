"""Regression proof against production rlibs, never the dev/test feature graph.

Builds the schema library with locked dependencies, compiles a temporary caller
against those exact artifacts, and reports fixed float edge vectors. No Cargo
feature is selected by this driver. Nonzero exit means actual fidelity failure.
"""
import json
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SOURCE = r'''
use uiblueprint_schema::{analysis::*,model::*};
fn main() {
    let path=std::env::args().nth(1).unwrap();
    let template=std::fs::read(path).unwrap();
    let space=Space{id:Id("float-space".into()),kind:SpaceKind::Local,units:Unit::Px,origin:Origin::TopLeft};
    let key=SourceKey{namespace:Id("authored.float".into()),key:Id("node".into())};
    // First three are the exact production counterexamples; remaining values
    // exercise zero sign, subnormal/minimum, fraction and large finite metadata.
    let values=[f64::from_bits(0x3fee100db2d7d206),f64::from_bits(0x3fefde3f09756650),
        f64::from_bits(0x3fed7e42512b1d0b),f64::from_bits(1),f64::MIN_POSITIVE,
        0.0,-0.0,0.1,0.5,1.0,f64::MAX];
    let mut failures=Vec::new();
    for value in values {
        let large=value>1.0;
        let query=GeometryQuery{id:Id("float-query".into()),scope_id:Id("float-scope".into()),
            targets:vec![key.clone()],operation:GeometryRelation::Width,
            anchors:vec![Anchor{element:key.clone(),frame_kind:FrameKind::LayoutBounds,
                coordinate_space:space.clone(),fraction:if large {0.5}else{value},axis:Id("x".into())}],
            quantity_kind:QuantityKind::Length,units:Unit::Px,
            applies_when:ContextConditions{platform:None,input_mode:None,text_scale:if large{Some(value)}else{None}}};
        let doc=AnalysisDocument{schema_version:AnalysisVersion::CURRENT,artifact:AnalysisArtifact::GeometryQuery(Box::new(query))};
        let bytes=serde_json::to_vec(&doc).unwrap();
        let AnalysisArtifact::GeometryQuery(query)=AnalysisDocument::from_json(&bytes,bytes.len()).unwrap().artifact else {unreachable!()};
        let analysis=if large{query.applies_when.text_scale.unwrap()}else{query.anchors[0].fraction};
        let mut core=Document::from_json(&template,template.len()).unwrap();
        let Artifact::Geometry(case)=&mut core.artifact else{unreachable!()};
        let Shape::Rect(rect)=&mut case.geometry.shape else{unreachable!()};rect.x=value;
        let bytes=serde_json::to_vec(&core).unwrap();
        let Artifact::Geometry(case)=Document::from_json(&bytes,bytes.len()).unwrap().artifact else{unreachable!()};
        let Shape::Rect(rect)=case.geometry.shape else{unreachable!()};
        for (format,actual) in [("analysis0.2",analysis),("core0.1",rect.x)] {
            if value.to_bits()!=actual.to_bits(){failures.push(serde_json::json!({
                "format":format,"literal":serde_json::to_string(&value).unwrap(),
                "before_bits":format!("{:016x}",value.to_bits()),"after_bits":format!("{:016x}",actual.to_bits())}));}
        }
    }
    println!("{}",serde_json::json!({"cases":values.len()*2,"failures":failures}));
}
'''


def main():
    result = subprocess.run([
        "cargo", "+1.96.0", "build", "--locked", "--offline", "-p", "uiblueprint-schema",
        "--lib", "--message-format=json",
    ], cwd=ROOT, capture_output=True, text=True, check=True)
    artifacts = {}
    for line in result.stdout.splitlines():
        item = json.loads(line)
        if item.get("reason") == "compiler-artifact":
            artifacts[item["target"]["name"]] = item
    def rlib(name):
        return next(path for path in artifacts[name]["filenames"] if path.endswith(".rlib"))
    with tempfile.TemporaryDirectory(prefix="uiblueprint-analysis-prod-") as temporary:
        path = Path(temporary)
        (path / "probe.rs").write_text(SOURCE)
        subprocess.run([
            "rustc", "+1.96.0", "--edition=2024", str(path / "probe.rs"),
            "--extern", "uiblueprint_schema=" + rlib("uiblueprint_schema"),
            "--extern", "serde_json=" + rlib("serde_json"),
            "-L", "dependency=" + str(Path(rlib("serde_json")).parent),
            "-o", str(path / "probe"),
        ], cwd=ROOT, capture_output=True, text=True, check=True)
        output = subprocess.run([str(path / "probe"), str(ROOT / "fixtures/golden/GEO-TRANSFORM.json")],
            capture_output=True, text=True, timeout=20, check=True)
        proof = json.loads(output.stdout)
    proof["serde_json_features"] = artifacts["serde_json"]["features"]
    proof["serde_json_package"] = artifacts["serde_json"]["package_id"]
    proof["schema_features"] = artifacts["uiblueprint_schema"]["features"]
    print(json.dumps(proof, indent=2))
    return int(bool(proof["failures"]))


if __name__ == "__main__":
    raise SystemExit(main())
