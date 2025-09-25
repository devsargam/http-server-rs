# HTTP from scratch

This is my attempt at implementating http protocol in rust

### Getting Started

Basicially http request is
```
GET /path HTTP/1.1
Host: example.com
User-Agent: curl/7.81.0
Accept: */*
```

http response is
```
HTTP/1.1 200 OK
Content-Length: 13
Content-Type: text/plain

Hello, world!
```
