# HTML Transformer RS

[![commit-test](https://github.com/heerden/html-transformer-rs/actions/workflows/test.yml/badge.svg)](https://github.com/heerden/html-transformer-rs/actions/workflows/test.yml)

A Rust library that converts the text inside HTML <p> elements to upper or lower case.

A light Axum web server is used to exposes a transform API endpoint.

The transformer library is dependent on `kuchikiki` (Brave)
which is further build on top of `html5ever` (Mozilla).

Fully test coverage is provided for the Transform Service.

# Getting started

Package was developed on macOS and tested on Ubuntu. 
Rust version `1.85.0` was used.

Prerequisites:
```shell
cargo install cargo-tarpaulin
cargo install cargo-sort
```

Use the following make commands to get started:

| Command         | Description                                                    |
|-----------------|----------------------------------------------------------------|
| `make test`     | Perform unit test                                              |
| `make coverage` | Generates a test coverage report                               |
| `make build`    | Builds a production ready package, ready to be served          |
| `make pretty`   | Automatically formats your code to Rust standards              |
| `make lint`     | Lint checks your code                                          |
| `make check`    | Perform a combined Pretty and Lint test before you commit code |
| `make serve`    | Serve the package server from the build production release     |

# Usage

Quick steps to usage the package:
```shell
make build
chmod +x ./target/release/html-transformer
make serve
```

Example request:
```shell
curl -v POST http://localhost:8080/v1/transform \
     -H "Content-Type: application/json" \
     -d '{
           "transform": "lowercase",
           "html": "<div><p>First paragraph element</p><span>Not a paragraph</span><p>This is the <strong>Second</strong> listed <em>Paragraph</em> element</p></div>"
         }'
```
With example response:
```text
* Host localhost:8080 was resolved.
* IPv6: ::1
* IPv4: 127.0.0.1
*   Trying [::1]:8080...
* connect to ::1 port 8080 from ::1 port 61200 failed: Connection refused
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080
> POST /v1/transform HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.7.1
> Accept: */*
> Content-Type: application/json
> Content-Length: 216
> 
* upload completely sent off: 216 bytes
< HTTP/1.1 200 OK
< content-type: text/plain; charset=utf-8
< vary: accept-encoding
< vary: origin, access-control-request-method, access-control-request-headers
< access-control-allow-origin: *
< access-control-expose-headers: *
< content-length: 145
< date: Mon, 16 Jun 2025 09:43:44 GMT
< 
* Connection #0 to host localhost left intact
<div><p>first paragraph element</p><span>Not a paragraph</span><p>this is the <strong>second</strong> listed <em>paragraph</em> element</p></div>
```

# Bonus tasks
- [ ] Perform load test with Grafana k6
- [ ] Compare result with an Actix implementation
- [ ] Integration test for the web servers
- [ ] Ensure fully test coverage of all sub-systems
- [ ] More robust error handling responses
- [ ] Extend for full html document return, if required, currently it only returns body fragments

# Future Recommendation

- Extend the api to transform the text case any specific html tag
- Extend the api to transform with more case types, e.g. sentence case
- `syn-rs` can be explored as an alternative to `kuchikiki`:
https://github.com/stoically/syn-rsx
