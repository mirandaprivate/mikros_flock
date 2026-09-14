"""One-time split: keep compact dispatch out of the dense SIMD inner loop."""
from pathlib import Path
p=Path(__file__).resolve().parent/'src/fast_sumcheck.rs'
s=p.read_text(encoding='utf-8')
where='fn block_message(input: Input'
start=s.index(where)
body=s.index('    let mut sums = [F256Unreduced::ZERO; 3];', start)
end=s.index('\nfn message(',body)
old=s[body:end]
replacement='''#[derive(Clone, Copy)]
enum DenseInput<'a> {
    Interleaved(&'a [F128]),
    Separate(&'a [F128], &'a [F128]),
}
impl DenseInput<'_> {
    #[inline]
    fn at(self, i: usize) -> (F128, F128) {
        match self {
            Self::Interleaved(x) => (x[2*i], x[2*i+1]),
            Self::Separate(a,b) => (a[i], b[i]),
        }
    }
}
#[inline(never)]
fn compact_block_message(src: CompactLayer<'_>, start: usize, weights: &[F128]) -> [F128; 3] {
    let mut values = [F128::ZERO; 1024];
    let mut result = [F128::ZERO; 3];
    for (block, w) in weights.chunks(256).enumerate() {
        let out = &mut values[..4*w.len()];
        src.read(4*(start+block*256), out);
        result = add(result, dense_block_message(DenseInput::Interleaved(out), 0, w));
    }
    result
}
fn block_message(input: Input<'_>, start: usize, weights: &[F128]) -> [F128; 3] {
    match input {
        Input::Interleaved(x) => dense_block_message(DenseInput::Interleaved(x), start, weights),
        Input::Separate(a,b) => dense_block_message(DenseInput::Separate(a,b), start, weights),
        Input::Compact(src) => compact_block_message(src,start,weights),
    }
}
fn dense_block_message(input: DenseInput<'_>, start: usize, weights: &[F128]) -> [F128; 3] {
'''+old
s=s[:start]+replacement+s[end:]
p.write_text(s,encoding='utf-8')
