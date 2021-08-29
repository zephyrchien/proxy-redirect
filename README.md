## Proxy-Redirect
[![CI][ci-badge]][ci-url]
[![Build][build-badge]][build-url]
[![License][mit-badge]][mit-url]

[ci-badge]: https://github.com/zephyrchien/proxy-redirect/workflows/ci/badge.svg
[ci-url]: https://github.com/zephyrchien/proxy-redirect/actions

[build-badge]: https://github.com/zephyrchien/proxy-redirect/workflows/build/badge.svg
[build-url]: https://github.com/zephyrchien/proxy-redirect/actions

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/zephyrchien/proxy-redirect/blob/master/LICENSE

This tiny tool listen on a specific port and redirects traffics to other different ports, making it possible to support multiple protocols on one port.

## Usage
```shell
redirect -c example.conf
```

## Config
```
listen = 127.0.0.1:10000
socks5 = 127.0.0.1:20000
http = 127.0.0.1:30000
default = 127.0.0.1:40000
```

## License
MIT