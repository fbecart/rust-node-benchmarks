const Rusha = require('rusha');
const tomlParser = require('toml');

function sum(a, b) {
  return a + b;
}

function sha1(data) {
  const hash = Rusha.createHash();
  hash.update(data);
  return hash.digest('hex');
}

function fibonacci(n) {
  let a = 1, b = 1;
  for (let i = 0; i < n; i++) {
    const sum = a + b;
    a = b;
    b = sum;
  }
  return b;
}

function readFieldFromToml(tomlString, fieldName) {
  const data = tomlParser.parse(tomlString);
  return data[fieldName];
}

module.exports = {
  sum,
  sha1,
  fibonacci,
  readFieldFromToml,
};
