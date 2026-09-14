"""One-time internal plumbing for mutable, consumed tree layers."""
from pathlib import Path

base=Path(__file__).resolve().parent
p=base/'src/permutation.rs'
s=p.read_text(encoding='utf-8')
s=s.replace('a: &impl Tree,', 'a: &mut impl Tree,').replace('b: &impl Tree,','b: &mut impl Tree,')
s=s.replace('if engine == Engine::Fused {','if matches!(engine, Engine::Fused | Engine::Released) {')
s=s.replace('crate::fast_sumcheck::prove(a, c, p)','crate::fast_sumcheck::prove(a, c, p, engine == Engine::Fused)')
s=s.replace('crate::fast_sumcheck::prove(b, c, p)','crate::fast_sumcheck::prove(b, c, p, engine == Engine::Fused)')
s=s.replace('let a = crate::reference', 'let mut a = crate::reference').replace('let b = crate::reference', 'let mut b = crate::reference')
s=s.replace('let a = ProductTree::from_bits','let mut a = ProductTree::from_bits').replace('let b = ProductTree::from_bits','let mut b = ProductTree::from_bits')
s=s.replace('prove_trees(&a, &b,','prove_trees(&mut a, &mut b,')
p.write_text(s,encoding='utf-8')
p=base/'tests/differential.rs'
s=p.read_text(encoding='utf-8').replace('Engine::LookupTree,\n', 'Engine::LookupTree,\n                Engine::Released,\n')
p.write_text(s,encoding='utf-8')
