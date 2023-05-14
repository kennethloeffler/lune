<!-- @generated with lune-cli -->

# Net


Built-in library for network access

### Example usage

```lua
local net = require("@lune/net")

-- Sending a web request
local response = net.request("https://www.google.com")
print(response.ok)
print(response.statusCode, response.statusMessage)
print(response.headers)

-- Using a JSON web API
local response = net.request({
    url = "https://dummyjson.com/products/add",
    method = "POST",
    headers = { ["Content-Type"] = "application/json" },
    body = net.jsonEncode({
        title = "Cool Pencil",
    })
})
local product = net.jsonDecode(response.body)
print(product.id, "-", product.title)

-- Starting up a webserver
net.serve(8080, function(request)
    return {
        status = 200,
        body = "Echo:\n" .. request.body,
    }
end)
```

---

## Functions

### jsonDecode

```lua
function Net.jsonDecode(encoded: string)
```

Decodes the given JSON string into a lua value.

### jsonEncode

```lua
function Net.jsonEncode(value: any, pretty: boolean?)
```

Encodes the given value as JSON.

### request

```lua
function Net.request(config: string | FetchParams)
```

Sends an HTTP request using the given url and / or parameters, and returns a dictionary that describes the response received.

Only throws an error if a miscellaneous network or I/O error occurs, never for unsuccessful status codes.

### serve

```lua
function Net.serve(port: number, handlerOrConfig: ServeHttpHandler | ServeConfig)
```

Creates an HTTP server that listens on the given `port`.

This will ***not*** block and will keep listening for requests on the given `port`
until the `stop` function on the returned `ServeHandle` has been called.

### socket

```lua
function Net.socket(url: string)
```

Connects to a web socket at the given URL.

Throws an error if the server at the given URL does not support
web sockets, or if a miscellaneous network or I/O error occurs.

### urlDecode

```lua
function Net.urlDecode(s: string, binary: boolean?)
```

Decodes the given string using URL decoding.

### urlEncode

```lua
function Net.urlEncode(s: string, binary: boolean?)
```

Encodes the given string using URL encoding.

---

## Types

### FetchParams

```lua
type FetchParams = {
	url: string,
	method: HttpMethod?,
	query: { [string]: string }?,
	headers: { [string]: string }?,
	body: string?,
}
```

Parameters for sending network requests with `net.request`.

This is a dictionary that may contain one or more of the following values:

* `url` - The URL to send a request to. This is always required
* `method` - The HTTP method verb, such as `"GET"`, `"POST"`, `"PATCH"`, `"PUT"`, or `"DELETE"`. Defaults to `"GET"`
* `query` - A table of key-value pairs representing query parameters in the request path
* `headers` - A table of key-value pairs representing headers
* `body` - The request body

### FetchResponse

```lua
type FetchResponse = {
	ok: boolean,
	statusCode: number,
	statusMessage: string,
	headers: { [string]: string },
	body: string,
}
```

Response type for sending network requests with `net.request`.

This is a dictionary containing the following values:

* `ok` - If the status code is a canonical success status code, meaning within the range 200 -> 299
* `statusCode` - The status code returned for the request
* `statusMessage` - The canonical status message for the returned status code, such as `"Not Found"` for status code 404
* `headers` - A table of key-value pairs representing headers
* `body` - The request body, or an empty string if one was not given

### HttpMethod

```lua
type HttpMethod = "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "OPTIONS" | "PATCH"
```

### ServeConfig

```lua
type ServeConfig = {
	handleRequest: ServeHttpHandler?,
	handleWebSocket: ServeWebSocketHandler?,
}
```

Configuration for `net.serve`.

This may contain one of, or both of the following callbacks:

* `handleRequest` for handling normal http requests, equivalent to just passing a function to `net.serve`
* `handleWebSocket` for handling web socket requests, which will receive a `WebSocket` object as its first and only parameter

### ServeHandle

```lua
type ServeHandle = {
	stop: () -> (),
}
```

A handle to a currently running web server, containing a single `stop` function to gracefully shut down the web server.

### ServeHttpHandler

```lua
type ServeHttpHandler = (request: ServeRequest) -> string | ServeResponse
```

### ServeRequest

```lua
type ServeRequest = {
	path: string,
	query: { [string]: string? },
	method: HttpMethod,
	headers: { [string]: string },
	body: string,
}
```

Data type for requests in `net.serve`.

This is a dictionary containing the following values:

* `path` - The path being requested, relative to the root. Will be `/` if not specified
* `query` - A table of key-value pairs representing query parameters in the request path
* `method` - The HTTP method verb, such as `"GET"`, `"POST"`, `"PATCH"`, `"PUT"`, or `"DELETE"`. Will always be uppercase
* `headers` - A table of key-value pairs representing headers
* `body` - The request body, or an empty string if one was not given

### ServeResponse

```lua
type ServeResponse = {
	status: number?,
	headers: { [string]: string }?,
	body: string?,
}
```

Response type for requests in `net.serve`.

This is a dictionary that may contain one or more of the following values:

* `status` - The status code for the request, in the range `100` -> `599`
* `headers` - A table of key-value pairs representing headers
* `body` - The response body

### ServeWebSocketHandler

```lua
type ServeWebSocketHandler = (socket: WebSocket) -> ()
```

### WebSocket

```lua
type WebSocket = {
	closeCode: number?,
	close: (code: number?) -> (),
	send: (message: string, asBinaryMessage: boolean?) -> (),
	next: () -> string?,
}
```

A reference to a web socket connection.

The web socket may be in either an "open" or a "closed" state, changing its current behavior.

When open:

* Any function on the socket such as `send`, `next` or `close` can be called without erroring
* `next` can be called to yield until the next message is received or the socket becomes closed

When closed:

* `next` will no longer return any message(s) and instead instantly return nil
* `send` will throw an error stating that the socket has been closed

Once the websocket has been closed, `closeCode` will no longer be nil, and will be populated with a close
code according to the [WebSocket specification](https://www.iana.org/assignments/websocket/websocket.xhtml).
This will be an integer between 1000 and 4999, where 1000 is the canonical code for normal, error-free closure.
