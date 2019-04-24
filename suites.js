const Benchmark = require('benchmark');

const subjects = {
  'js': require('./subjects/js-example'),
  'neon': require('./subjects/neon-example/lib'),
  'wasm-pack': require('./subjects/wasm-pack-example/pkg'),
};

const benchedFunctions = {
  'sum': subject => subject.sum(1, 2),
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

  // run async
  suite.run({ 'async': true });
}
