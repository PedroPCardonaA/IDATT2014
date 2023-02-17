const express = require('express');
const path = require('path');
const { json, urlencoded } = require('body-parser');
const compiler = require('compilex');

const app = express();
app.use(json());
app.use(urlencoded({ extended: true }));
const option = { stats: true };
compiler.init(option);
app.get('/', (req, res) => {
  res.sendFile(path.join(__dirname, '/index.html'));
});
app.post('/compilecode', (req, res) => {
  const code = req.body.code;
  const output = req.body.output;
  const lang = req.body.lang;
  const envData = { OS: 'windows' };
  compiler.compilePython(envData, code, (data) => {
    if (data.error) {
        res.send(data.error);
      output.value = data.error;
    } else {
        res.send(data.output);
      output.value = data.output;
    }
    res.end();
  });
});
app.get('/fullStat', (req, res) => {
  compiler.fullStat((data) => {
    res.send(data);
  });
});
app.listen(3000);

compiler.flush(() => {
  console.log('All temporary files flushed!');
});
