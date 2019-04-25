const Rusha = require('rusha');

function sum(a, b) {
  return a + b;
}

function sha1(data) {
  const hash = Rusha.createHash();
  hash.update(data);
  return hash.digest('hex');
}

module.exports = {
  sum,
  sha1,
};
