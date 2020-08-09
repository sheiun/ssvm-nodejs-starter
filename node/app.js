const { evaluate } = require("../pkg/ssvm_nodejs_starter_lib.js");

const http = require("http");
const url = require("url");
const hostname = "0.0.0.0";
const port = 3000;

const server = http.createServer((req, res) => {
  const queryObject = url.parse(req.url, true).query;
  if (!queryObject["x"] || !queryObject["y"] || !queryObject["op"]) {
    res.end(
      `Please use command curl http://${hostname}:${port}/?x=1&op=+&y=2 \n`
    );
  } else {
    let ans = evaluate(queryObject["x"], queryObject["y"], queryObject["op"]);
    console.log(ans);
    res.end(ans);
  }
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
