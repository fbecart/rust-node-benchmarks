const Benchmark = require('benchmark');

const subjects = {
  'js': require('./subjects/js-example'),
  'neon': require('./subjects/neon-example/lib'),
  'wasm-pack': require('./subjects/wasm-pack-example/pkg'),
};

const benchedFunctions = {
  'sum': s => s.sum(1, 2),
  'sha1': s => s.sha1('pls-sha1-me'),
};

// add tests
for (const functionName of Object.keys(benchedFunctions)) {
  const suite = new Benchmark.Suite;
  for (const subjectName of Object.keys(subjects)) {
    suite.add(`${subjectName}#${functionName}`, () => benchedFunctions[functionName](subjects[subjectName]))
  }

  // add listeners
  suite
    .on('cycle', function (event) {
      console.log(String(event.target));
    })
    .on('complete', function () {
      console.log('Fastest is ' + this.filter('fastest').map('name'));
    });

  // run
  suite.run();
}
