const Benchmark = require('benchmark');
const report = require('beautify-benchmark');
const _ = require('lodash');
const numeral = require('numeral');
const fs = require('fs');

const exampleToml = fs.readFileSync('./data/example.toml', 'utf8');

const subjects = {
  'js': require('./subjects/js-example'),
  'rust-ffi': require('./subjects/neon-example/lib'),
  'rust-wasm': require('./subjects/wasm-pack-example/pkg'),
  'ts-wasm': require('./subjects/assemblyscript-example'),
};

const benchedFunctions = {
  'sum': s => s.sum(1, 2),
  'sha1': s => s.sha1('pls-sha1-me'),
  'fibonacci': s => s.fibonacci(10000),
  'parse TOML': s => s.readFieldFromToml(exampleToml, 'title'),
};

for (const functionName of Object.keys(benchedFunctions)) {
  console.log(`== ${functionName} ==\n`);

  const suite = new Benchmark.Suite;
  for (const subjectName of Object.keys(subjects)) {
    suite.add(subjectName, () => benchedFunctions[functionName](subjects[subjectName]))
  }

  suite.on('cycle', event => report.add(event.target));
  suite.on('complete', () => {
    logComparison(report);
    report.log();
  });

  suite.run();
}

function logComparison(report) {
  const benches = _.sortBy(report.store.filter(bench => bench.hz), bench => -bench.hz);
  console.log(`  ${benches[0].name} was the fastest`);
  for (let i = 1; i < benches.length; i++) {
    const performanceRatio = benches[0].hz / benches[i].hz;
    console.log(`  ${benches[i].name} was ${numeral(performanceRatio).format('0a')}x slower than ${benches[0].name}`)
  }
}
