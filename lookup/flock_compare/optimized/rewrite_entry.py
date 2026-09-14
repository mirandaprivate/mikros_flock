from pathlib import Path
p=Path(__file__).resolve().parent/'src/permutation.rs'
s=p.read_text()
s=s.replace('    product_tree::ProductTree,','    product_tree::{ProductTree, Tree},\n    profile::{Engine, Timings, elapsed},')
s=s.replace('use serde::{Deserialize, Serialize};','use serde::{Deserialize, Serialize};\nuse std::time::Instant;')
a=s.index('pub(crate) fn prove_encoded')
b=s.index('pub(crate) fn verify_encoded')
s=s[:a]+'''fn prove_trees<C: Challenger>(a: &impl Tree, b: &impl Tree, z: F128, c: &mut C, engine: Engine, p: &mut Timings) -> Result<(Proof, [LeafClaim; 2]), Error> {
    let roots = [a.root(), b.root()];
    if roots[0] != roots[1] {return Err(Error::ProductMismatch);}
    c.observe_f128_slice(&roots);
    c.observe_label(b"left-tree");
    let (left, mut ca) = if engine == Engine::Fused {crate::fast_sumcheck::prove(a,c,p)} else {crate::reference::sumcheck::prove(a,c,p)};
    c.observe_label(b"right-tree");
    let (right, mut cb) = if engine == Engine::Fused {crate::fast_sumcheck::prove(b,c,p)} else {crate::reference::sumcheck::prove(b,c,p)};
    ca.value += z; cb.value += z;
    Ok((Proof{roots,left,right},[ca,cb]))
}
''' + s[b:]
a=s.index('pub fn prove<C: Challenger>')
b=s.index('pub fn verify<C: Challenger>')
s=s[:a]+'''pub fn prove<C: Challenger>(statement:&Statement,left:&PackedBits,right:&PackedBits,options:Options,c:&mut C)->Result<(Proof,Claims,Stats),Error> {
    prove_profiled(statement,left,right,options,Engine::Fused,c).map(|(a,b,c,_)|(a,b,c))
}
pub fn prove_profiled<C: Challenger>(statement:&Statement,left:&PackedBits,right:&PackedBits,options:Options,engine:Engine,c:&mut C)->Result<(Proof,Claims,Stats,Timings),Error> {
    let mut p=Timings::default();
    let t=Instant::now();
    if engine==Engine::Reference {statement.left.check(left)?;statement.right.check(right)?;}
    else {check_source_parallel(&statement.left,left)?;check_source_parallel(&statement.right,right)?;}
    statement.bind(c)?;options.validate()?;p.source_checks_ms=elapsed(t);
    c.observe_label(b"mikros-permutation-grand-product-v1");let z=c.sample_f128();
    let (proof,leaves,stats)=if engine==Engine::Reference {
        let t=Instant::now();
        let va:Vec<_>=(0..left.rows()).map(|i|left.row(i)).collect();
        let vb:Vec<_>=(0..right.rows()).map(|i|right.row(i)).collect();p.decode_ms+=elapsed(t);
        let a=crate::reference::product_tree::ProductTree::new(&va,left.width(),z,options,&mut p)?;
        let b=crate::reference::product_tree::ProductTree::new(&vb,right.width(),z,options,&mut p)?;
        let (proof,leaves)=prove_trees(&a,&b,z,c,engine,&mut p)?;
        (proof,leaves,Stats{left:a.stats,right:b.stats})
    } else {
        let a=ProductTree::from_bits(left,z,options,engine,&mut p)?;
        let b=ProductTree::from_bits(right,z,options,engine,&mut p)?;
        let (proof,leaves)=prove_trees(&a,&b,z,c,engine,&mut p)?;
        (proof,leaves,Stats{left:a.stats,right:b.stats})
    };
    Ok((proof,source_claims(statement,leaves)?,stats,p))
}
pub fn check_source_parallel(spec:&OracleSpec,bits:&PackedBits)->Result<(),Error> {
    spec.validate()?;bits.validate()?;
    if spec.rows!=bits.rows() || spec.width!=bits.width() || spec.digest!=bits.digest_parallel() {return Err(Error::Source);}
    Ok(())
}
''' +s[b:]
needle='impl Statement {\n'
assert s.count(needle)==1
s=s.replace(needle,needle+'''    pub fn new_parallel(left:&PackedBits,right:&PackedBits)->Result<Self,Error> {
        left.validate()?;right.validate()?;
        let result=Self {
            left:OracleSpec{rows:left.rows(),width:left.width(),digest:left.digest_parallel()},
            right:OracleSpec{rows:right.rows(),width:right.width(),digest:right.digest_parallel()},
        };
        result.validate()?;Ok(result)
    }
''')
p.write_text(s)
