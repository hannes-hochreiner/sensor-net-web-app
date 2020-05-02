var express = require('express');
var history = require('connect-history-api-fallback');
var app = express();
const port = process.env.PORT || 8888;
app.use(history());
app.use(express.static('dist'));
app.listen(port, () => console.log(`Express listening at http://localhost:${port}`))
