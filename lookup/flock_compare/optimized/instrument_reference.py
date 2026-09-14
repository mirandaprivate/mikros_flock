"""Instrument frozen reference loops; preserve their arithmetic and allocations."""
from pathlib import Path
p = Path(__file__).resolve().parent / "src"
def replace_once(s,a,b):
    assert s.count(a)==1,a
    return s.replace(a,b)
t = (p / "reference/product_tree.rs").read_text()
start=t.index("#[derive(Clone, Copy, Debug")
end=t.index("/// Consecutive words")
t=t[:start]+'use crate::product_tree::{Options, Strategy, TreeStats, Tree};\nuse crate::profile::{Timings, elapsed};\nuse std::{time::Instant, borrow::Cow};\n'+t[end:]
t=t.replace("use serde::{Deserialize, Serialize};\n","")
t=replace_once(t,"z: F128, options: Options)","z: F128, options: Options, p: &mut Timings)")
t=replace_once(t,"            let mut leaves = Words::new", "            let phase = Instant::now();\n            let mut leaves = Words::new")
t=replace_once(t,"            compact.push(leaves);", "            compact.push(leaves);\n            p.decode_ms += elapsed(phase);\n            let phase = Instant::now();")
t=replace_once(t,"            stats.switch_level =", "            p.compact_build_ms += elapsed(phase);\n            stats.switch_level =")
t=replace_once(t,"            let first = Self::evaluate_words", "            let phase = Instant::now();\n            let first = Self::evaluate_words")
t=replace_once(t,"            compact.pop();", "            p.coefficient_eval_ms += elapsed(phase);\n            compact.pop();")
t=replace_once(t,"            (0..len)\n", "            let phase = Instant::now();\n            let out = (0..len)\n")
t=replace_once(t,"                .collect()\n        };", "                .collect();\n            p.coefficient_eval_ms += elapsed(phase);\n            out\n        };")
t=replace_once(t,"        let mut extension = vec![first];", "        let phase = Instant::now();\n        let mut extension = vec![first];")
t=replace_once(t,"        stats.extension_bytes =", "        p.extension_build_ms += elapsed(phase);\n        stats.extension_bytes =")
t+='''
impl Tree for ProductTree {
    fn root(&self)->F128 {self.root()}
    fn log_len(&self)->usize {self.log_len}
    fn layer(&self,level:usize)->Cow<'_,[F128]> {Cow::Owned(self.layer(level))}
}
'''
(p / "reference/product_tree.rs").write_text(t)
s=(p / "reference/sumcheck.rs").read_text()
s=s.replace('use crate::{Error, product_tree::ProductTree};','use crate::{product_tree::Tree, profile::{Timings, elapsed}};\nuse std::time::Instant;')
s=s.replace('use serde::{Deserialize, Serialize};','use crate::sumcheck::{LayerProof, TreeProof, LeafClaim};')
start=s.index('#[derive(Clone, Debug, PartialEq')
end=s.index('fn evaluate(')
s=s[:start]+s[end:]
s=s[:s.index('pub(crate) fn verify')]
s=s.replace('tree: &ProductTree, c: &mut C','tree: &impl Tree, c: &mut C, p: &mut Timings')
s=s.replace('tree.log_len','tree.log_len()')
s=replace_once(s,'        let children = tree.layer(level);','        let phase = Instant::now();\n        let children = tree.layer(level);')
s=replace_once(s,'        let mut e = eq_table','        p.layer_materialize_ms += elapsed(phase);\n        let phase = Instant::now();\n        let mut e = eq_table')
s=replace_once(s,'        let mut rounds = Vec::new();','        p.equality_ms += elapsed(phase);\n        let mut rounds = Vec::new();')
s=replace_once(s,'            let mut g = [F128::ZERO; 3];','            let phase = Instant::now();\n            let mut g = [F128::ZERO; 3];')
s=replace_once(s,'            c.observe_f128_slice(&g);','            p.messages_ms += elapsed(phase);\n            c.observe_f128_slice(&g);')
s=replace_once(s,'            fold_low(&mut a, t);','            let phase = Instant::now();\n            fold_low(&mut a, t);')
s=replace_once(s,'            fold_low(&mut e, t);','            fold_low(&mut e, t);\n            p.folds_ms += elapsed(phase);')
(p / "reference/sumcheck.rs").write_text(s)
print('Instrumented reference tree and sumcheck')
