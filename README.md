# Base32 encode and decode web app

A simple web app based off [this example app](https://github.com/second-state/wasm-learning/tree/master/nodejs/quadratic) to try learning node, rust, and ssvm.

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, SSVM, and ssvmup](https://www.secondstate.io/articles/setup-rust-nodejs/).


## Build the application

```
$ docker pull secondstate/ssvm-nodejs-starter:v1
$ docker run -p 8080:8080 --rm -it -v $(pwd):/app secondstate/ssvm-nodejs-starter:v1
$ cd /app
$ ssvmup build
```


## Web app

```
$ cd node
$ node server.js
Listening at http://localhost:8080

Then point your browser to `http://hostname:8080` enter some text to be encoded to base64  

![alt text](https://github.com/mr4nk/wasm32/blob/master/wasm32/pics/encode.jpeg?raw=true)

Enter base64 encoded text to decode
![alt text](https://github.com/mr4nk/wasm32/blob/master/wasm32/pics/decode.jpeg?raw=true)
