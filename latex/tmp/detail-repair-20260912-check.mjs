import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';

const fields = [];
for (const p of [2, 3, 5, 17]) {
  fields.push({name: `F${p}`, size: p,
    add: (x, y) => (x + y) % p,
    mul: (x, y) => x * y % p,
    neg: x => (p - x) % p});
}
for (const [size, modulus] of [[4, 7], [8, 11], [16, 19]]) {
  fields.push({name: `F${size}`, size,
    add: (x, y) => x ^ y, neg: x => x,
    mul: (x, y) => {
      let result = 0;
      while (y) {
        if (y & 1) result ^= x;
        y >>= 1;
        x <<= 1;
        if (x & size) x ^= modulus;
      }
      return result;
    }});
}

let exactCases = 0, scaledCases = 0, oddsCases = 0;
let degenerateCases = 0, terminalCases = 0;
for (const f of fields) {
  const sub = (x, y) => f.add(x, f.neg(y));
  const inv = x => {
    for (let j = 1; j < f.size; j++) if (f.mul(x, j) === 1) return j;
    throw new Error(`No inverse for ${x} in ${f.name}`);
  };
  const tensor = pairs => pairs.reduce(
    (acc, pair) => acc.flatMap(x => pair.map(y => f.mul(x, y))), [1]);
  const ts = values => tensor(values.map(x => [sub(1, x), x]));
  const dot = (x, y) => x.reduce((acc, v, j) => f.add(acc, f.mul(v, y[j])), 0);
  for (let z = 0; z < f.size; z++) for (let y = 0; y < f.size; y++) {
    const product = ts([z]).map((x, j) => f.mul(x, ts([y])[j]));
    const first = f.mul(sub(1, z), sub(1, y));
    const second = f.mul(z, y);
    assert.deepEqual(product, [first, second]);
    exactCases++;
    const normalizer = f.add(first, second);
    if (normalizer !== 0) {
      const query = f.mul(second, inv(normalizer));
      assert.deepEqual(product, ts([query]).map(x => f.mul(normalizer, x)));
      scaledCases++;
    } else degenerateCases++;
    if (first !== 0) {
      const odds = f.mul(f.mul(z, inv(sub(1, z))), f.mul(y, inv(sub(1, y))));
      assert.deepEqual(product, [first, f.mul(first, odds)]);
      oddsCases++;
    }
  }
  for (let sample = 0; sample < 100; sample++) {
    // Column-major input, arbitrary two-entry row factors, 4-by-4 matrix.
    const A = Array.from({length: 16}, (_, j) => (j * 7 + sample) % f.size);
    const B = Array.from({length: 16}, (_, j) => (j * 3 + sample * 2) % f.size);
    const factors = [[sample % f.size, (sample + 1) % f.size],
      [(sample + 2) % f.size, (2 * sample + 1) % f.size]];
    const y = [(sample + 3) % f.size, (sample + 5) % f.size];
    const yy = [(sample + 7) % f.size, (sample + 11) % f.size];
    const chi = ts(y), l = tensor(factors), columns = ts(yy);
    const weighted = l.map((x, j) => f.mul(x, chi[j]));
    const resultFactors = factors.map((pair, j) => [
      f.mul(pair[0], sub(1, y[j])), f.mul(pair[1], y[j])]);
    assert.deepEqual(weighted, tensor(resultFactors));
    const va = Array.from({length: 4}, (_, j) => dot(A.slice(j * 4, j * 4 + 4), chi));
    const vb = Array.from({length: 4}, (_, j) => dot(B.slice(j * 4, j * 4 + 4), weighted));
    const queryA = columns.flatMap(x => chi.map(v => f.mul(x, v)));
    const queryB = columns.flatMap(x => weighted.map(v => f.mul(x, v)));
    assert.equal(dot(va, columns), dot(A, queryA));
    assert.equal(dot(vb, columns), dot(B, queryB));
    terminalCases++;
  }
}

const root = process.cwd();
const read = name => fs.readFileSync(path.join(root, name), 'utf8');
const strip = text => text.split(/\r?\n/).map(line => line.replace(/(?<!\\)%.*/, '')).join('\n');
const active = new Map();
function collect(name) {
  if (active.has(name)) return;
  const source = strip(read(name));
  active.set(name, source);
  for (const match of source.matchAll(/\\(?:input|include)\s*\{([^}]+)\}/g)) {
    const child = match[1].endsWith('.tex') ? match[1] : `${match[1]}.tex`;
    if (!fs.existsSync(path.join(root, child))) throw new Error(`Missing input ${child}`);
    collect(child);
  }
}
collect('main.tex');
const labels = new Map(), citations = new Set(), refs = new Set(), errors = [];
for (const [name, text] of active) {
  let depth = 0;
  for (let j = 0; j < text.length; j++) {
    if (text[j] === '\\') { j++; continue; }
    if (text[j] === '{') depth++;
    if (text[j] === '}') depth--;
    if (depth < 0) { errors.push(`${name}: unmatched closing brace`); break; }
  }
  if (depth !== 0) errors.push(`${name}: brace depth ${depth}`);
  const envs = [];
  for (const match of text.matchAll(/\\(begin|end)\s*\{([^}]+)\}/g)) {
    if (match[1] === 'begin') envs.push(match[2]);
    else if (envs.pop() !== match[2]) errors.push(`${name}: environment mismatch ${match[2]}`);
  }
  if (envs.length) errors.push(`${name}: unclosed environments ${envs.join(',')}`);
  for (const match of text.matchAll(/\\label\s*\{([^}]+)\}/g)) {
    if (labels.has(match[1])) errors.push(`Duplicate label ${match[1]}`);
    labels.set(match[1], name);
  }
  for (const match of text.matchAll(/\\(?:eqref|ref|autoref|cref|Cref)\s*\{([^}]+)\}/g)) {
    for (const key of match[1].split(',')) refs.add(key.trim());
  }
  for (const match of text.matchAll(/\\cite\w*\*?(?:\[[^\]]*\])*\s*\{([^}]+)\}/g)) {
    for (const key of match[1].split(',')) citations.add(key.trim());
  }
}
const bibkeys = new Set([...read('ref.bib').matchAll(/@\w+\s*\{\s*([^,]+),/g)].map(m => m[1].trim()));
for (const ref of refs) if (!labels.has(ref) && !ref.includes('#')) errors.push(`Missing reference ${ref}`);
for (const key of citations) if (!bibkeys.has(key)) errors.push(`Missing citation ${key}`);
const contributions = text => text.slice(text.indexOf('\\subsection{Our Contributions}'));
assert.equal(contributions(read('1-intro.tex')),
  contributions(read('tmp/detail-repair-20260912-before/1-intro.tex')));
for (const name of ['alg/alg-permutation.tex', '6-together.tex', 'ref.bib']) {
  assert.equal(read(name), read(`tmp/detail-repair-20260912-before/${name}`));
}
console.log(JSON.stringify({fields: fields.map(f => f.name), exactCases, scaledCases,
  oddsCases, degenerateCases, terminalCases, activeFiles: active.size,
  labels: labels.size, citations: citations.size, contributionsUnchanged: true,
  permutationAlgorithmAndCircuitTheoremsUnchanged: true, errors}, null, 2));
if (errors.length) process.exitCode = 1;
