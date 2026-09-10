PHASE 0 — Rust in Practice
Goal
Get comfortable using Rust for a relatively large systems project.
You already know the language, so this is mostly about applying it.

Learn/practice
Data structures
struct
enum
Vec
HashMap
String
&str

slices
ownership
borrowing

lifetimes where necessary
Error handling
Result
Option
custom errors
?

error propagation
Organization

modules
mod
pub

traits
implementations

Useful Rust features

iterators
closures
pattern matching
generics

smart pointers
Arc
Mutex
Systems/concurrency

Eventually:

threads
channels
synchronization
async Rust

You don't need to build anything separate.
Use these concepts directly inside the browser.

PHASE 1 — URL Parsing
Goal
Turn:

https://example.com:443/foo/bar?q=rust#section

into structured data.
Your browser needs to understand:

scheme   → https
host     → example.com
port     → 443
path     → /foo/bar
query    → q=rust
fragment → section

Learn

URL syntax

schemes
hostname
ports
paths
queries
fragments
percent encoding
default ports
absolute vs relative URLs

Build
Create:


struct Url {
    scheme: String,
    host: String,
    port: u16,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
}

Start simple.
Don't try to implement every obscure part of the WHATWG URL specification.

Milestone
Input:
https://example.com:8080/test?q=hello

Output:
scheme: https
host: example.com
port: 8080
path: /test
query: q=hello

Learn from

WHATWG URL Standard

MDN URL documentation
PHASE 2 — TCP Implementation
Goal
Actually establish network connections yourself.
Your browser should be able to do:

example.com
     │
     ▼
IP address
     │
     ▼
TCP connection
     │
     ▼
server

Learn

sockets

IP addresses

ports

TCP connections

byte streams
connect
read
write

buffering

partial reads

EOF

connection closing
Important concept
TCP does not give you messages.
It gives you:

BYTE STREAM


So you need to understand why:

write("hello")
write("world")

doesn't necessarily arrive as:

"hello"
"world"

It could arrive as:

"hel"
"lowor"
"ld"

That becomes extremely important when implementing HTTP.

Build
src/
└── net/
    └── tcp.rs

Your browser should be able to establish a connection and send raw bytes.
PHASE 3 — HTTP/1.1
Goal
Speak HTTP yourself over your TCP connection.
Instead of using:

reqwest
hyper
curl

for your initial implementation, construct the HTTP request yourself.

Learn
Request
GET / HTTP/1.1
Host: example.com
Connection: close

Response
HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 1234

<html>...

Learn specifically

HTTP methods

status codes

request line

response line

headers

header parsing

body
Content-Length
Transfer-Encoding

chunked encoding
Connection

redirects

content types

MIME types
Build
src/
└── net/
    ├── mod.rs
    ├── tcp.rs
    └── http.rs

Then:

my-browser http://example.com


should:

URL
 ↓
URL parser
 ↓
TCP
 ↓
HTTP request
 ↓
HTTP response
 ↓
HTML

Milestone 1
You have built a minimal web client.
PHASE 4 — DNS
Goal
Allow the browser to use:

example.com


instead of requiring:

93.xxx.xxx.xxx


Learn

DNS

domain names

DNS resolution

A records

AAAA records

IPv4

IPv6

UDP

DNS caching
First implementation
Use the operating system's DNS resolver.
You don't need to implement DNS packets yet.
Your flow becomes:

URL
 ↓
hostname
 ↓
DNS
 ↓
IP
 ↓
TCP
 ↓
HTTP

Later
If you want additional networking depth:

DNS client
 ↓
UDP
 ↓
DNS packet parser

PHASE 5 — HTML Tokenization
Now you start building the browser engine.

Goal
Take:


<h1>Hello</h1><p>World</p>

and convert it into tokens.
For example:

StartTag("h1")
Text("Hello")
EndTag("h1")

StartTag("p")
Text("World")
EndTag("p")

Learn

lexical analysis

tokenization

state machines

character processing

tags

end tags

attributes

text

comments

whitespace
Build
src/
└── html/
    ├── mod.rs
    ├── token.rs
    └── tokenizer.rs

Important
Don't think of HTML as simply:

split("<")


Real HTML parsing is much more complicated because HTML isn't a simple XML-style grammar.
For your first browser, however, you can intentionally support a small subset.
PHASE 6 — HTML Parser + DOM
Goal
Turn HTML tokens into a tree.
Input:


<html>
    <body>
        <h1>Hello</h1>
        <p>World</p>
    </body></html>

Output:

Document
└── html
    └── body
        ├── h1
        │   └── "Hello"
        │
        └── p
            └── "World"

Learn

parsing

tree structures

parent/child relationships

elements

text nodes

attributes

document node
Build
Something conceptually like:

Node
├── Document
├── Element
└── Text

and:

Element
├── tag_name
├── attributes
└── children

Milestone 2
Your browser now does:

URL
 ↓
DNS
 ↓
TCP
 ↓
HTTP
 ↓
HTML
 ↓
Tokenizer
 ↓
Parser
 ↓
DOM
 ↓
print DOM

This is your first real browser-engine milestone.
PHASE 7 — CSS Tokenization
Goal
Understand CSS syntax.
Given:


h1 {
    color: red;
    font-size: 32px;
}

you need to recognize:

selector
{
property
:
value
;
}

Learn

selectors

declarations

properties

values

braces

colons

semicolons

whitespace

comments
Start by supporting

h1
.title#header

Don't attempt all CSS selectors.
PHASE 8 — CSS Parsing
Goal
Turn CSS into structured rules.
For:


h1 {
    color: red;
    font-size: 32px;
}

you could produce:

Rule
├── Selector
│   └── h1
│
└── Declarations
    ├── color: red
    └── font-size: 32px

Build
src/
└── css/
    ├── mod.rs
    ├── token.rs
    ├── tokenizer.rs
    └── parser.rs

PHASE 9 — Style Resolution
Now connect HTML and CSS.
Given:


<h1 class="title">Hello</h1>


and:


h1 {
    color: red;
}

.title {
    font-size: 30px;
}

the browser needs to determine:

h1
├── color: red
└── font-size: 30px

Learn

selector matching

cascade

specificity

inheritance

computed styles

default styles
Start with
tag selectors
.class selectors
#id selectors

Then gradually add complexity.

Result
DOM
 +
CSSOM
 ↓
Styled DOM

PHASE 10 — Layout Engine
This is one of the biggest browser concepts.
You know:

WHAT


the page contains.
You know:

HOW


it should look.
But you don't yet know:

WHERE


everything goes.

Goal
Turn elements into rectangles.
For example:

body
├── h1
└── p

becomes:

h1
x = 0
y = 0
width = 800
height = 40

p
x = 0
y = 40
width = 800
height = 20

Learn
CSS box model
margin
border
padding
content

Layout

width

height

block layout

inline layout

vertical flow

margins

padding

borders
Start with block layout.
Don't immediately attempt Flexbox, Grid, tables, etc.

Result
DOM
 ↓
Style
 ↓
Layout tree
 ↓
Rectangles

Milestone 3
You have a primitive layout engine.
PHASE 11 — Painting
Goal
Turn the layout into drawing commands.
For example:

Layout box
 ↓
Draw rectangle
 ↓
Draw background
 ↓
Draw border
 ↓
Draw text

Learn

paint order

backgrounds

borders

text

clipping

stacking

coordinate systems
Initially support:

background
rectangle
border
text

You don't need a GPU renderer yet.
PHASE 12 — Window + Graphics
Now give the browser a real window.

Goal
Take:

Paint commands


and put them on the screen.

Learn

native windows

event loops

framebuffers

pixel buffers

keyboard events

mouse events

window resizing

coordinate systems
In Rust, you can use libraries such as:

winit
pixels

rather than immediately writing platform-specific Linux/Windows windowing code.

Result
┌────────────────────────────────┐
│ http://example.com              │
├────────────────────────────────┤
│                                │
│ Hello World                    │
│                                │
│ This is my browser.            │
│                                │
└────────────────────────────────┘

Milestone 4
You have a graphical browser.
PHASE 13 — Navigation
Now make it actually behave like a browser.

Implement
<a> links

clicking links

absolute URLs

relative URLs

URL resolution

navigation

reload

redirects

back

forward

history
Flow:

Click link
 ↓
Get href
 ↓
Resolve URL
 ↓
Fetch resource
 ↓
Parse HTML
 ↓
Build DOM
 ↓
Style
 ↓
Layout
 ↓
Paint

PHASE 14 — Resource Loading
Webpages aren't just HTML.
They contain:

HTML
CSS
Images
Fonts
Scripts

Create a resource-loading system.

Learn

resource URLs

MIME types

asynchronous loading

dependencies

resource caching

loading states
Architecture:

Document
 │
 ├── HTML
 ├── CSS
 ├── Images
 ├── Fonts
 └── Scripts

PHASE 15 — Images
Implement:


<img src="cat.png">


Pipeline:

HTML
 ↓
Image URL
 ↓
HTTP
 ↓
Image bytes
 ↓
Decoder
 ↓
Pixel data
 ↓
Paint

Learn

image formats

decoding

raster images

pixel formats

alpha

image dimensions
Initially use an existing image decoding library.
Don't spend months writing PNG/JPEG decoders unless you specifically want that as another systems project.
PHASE 16 — Forms
Implement basic:


<form>
    <input>
    <button></form>

Learn

form controls

input state

form submission

GET

POST

URL encoding

form data
Eventually:

User input
 ↓
DOM
 ↓
Form submission
 ↓
HTTP request
 ↓
Server

PHASE 17 — Cookies + Cache + Compression
Now you're moving from "render webpages" toward "behave like a browser."

Cookies
Learn:

Set-Cookie
Cookie

domains

paths

expiration

Secure

HttpOnly

SameSite
Cache
Learn:


caching

cache keys

freshness

validation
ETag
Last-Modified
Cache-Control
Compression
Learn:


gzip

Brotli

content encoding
Your resource pipeline becomes:

Request
 ↓
Cache?
 ↓
Network
 ↓
Response
 ↓
Decode
 ↓
Resource

PHASE 18 — JavaScript Integration
Now add JavaScript.

IMPORTANT
Do not build a JavaScript engine from scratch yet.
That's a massive project.
Instead:

Your Browser
      ↓
Existing JS Engine
      ↓
JavaScript execution

Learn

JavaScript execution

ECMAScript

execution contexts

objects

functions

promises

event loop
Then expose browser APIs.
For example:


document.querySelector(...)document.bodyelement.textContent = "Hello"element.addEventListener(...)

The important part
Your JavaScript engine isn't really the browser.
The browser provides the host environment.

JavaScript
    │
    ▼
JS Engine
    │
    ▼
Browser APIs
    │
    ▼
DOM

PHASE 19 — Events
Now connect your operating system to the browser.
Example:

Mouse click
     ↓
Window
     ↓
Browser event
     ↓
DOM element
     ↓
JavaScript
     ↓
DOM modification
     ↓
Style
     ↓
Layout
     ↓
Paint

Learn

mouse events

keyboard events

event listeners

event propagation

bubbling

capturing

timers

event loop
Milestone 5
You have an interactive browser.
PHASE 20 — TLS / HTTPS
Now support:

https://example.com


Your network stack becomes:

URL
 ↓
DNS
 ↓
TCP
 ↓
TLS
 ↓
HTTP

Learn

TLS handshake
encryption
certificates
certificate authorities
public/private keys
certificate validation
trust stores
TLS versions

Important
Don't implement cryptography yourself.
Use an established TLS library.
You're learning how TLS integrates with a browser, not trying to replace OpenSSL/rustls.

PHASE 21 — Modern HTTP
Once HTTP/1.1 works, learn:

HTTP/1.1
   ↓
HTTP/2
   ↓
HTTP/3

HTTP/2

Learn:

binary framing
streams
multiplexing
header compression
connection reuse

HTTP/3

Learn:

HTTP/3
   ↓
QUIC
   ↓
UDP

and:


QUIC

streams

connection migration

TLS integration
You don't necessarily need to implement HTTP/2/3 yourself.
Understanding the architecture is the important part initially.
PHASE 22 — Browser Security
This is where the project becomes much more serious.

Learn
Same-origin policy
Understand:

scheme + host + port


and why:

https://site-a.com


can't freely access:

https://site-b.com


CORS
Learn:


origins

preflight
Access-Control-Allow-Origin

credentials
Cookies
Understand:


SameSite

Secure

HttpOnly

domain/path restrictions
Sandboxing
Understand why browser content cannot simply:

read your filesystem
execute arbitrary programs
access every process

PHASE 23 — Browser Architecture
Now study how your single-process toy browser evolves into something closer to a modern browser.
Instead of:

Browser
└── Everything

you start thinking:

Browser Process
│
├── UI
│
├── Network
│
├── Renderer
│   ├── HTML
│   ├── CSS
│   ├── Layout
│   └── JavaScript
│
├── GPU
│
└── Utility Processes

Learn

processes
threads
IPC
sandboxing
process isolation
renderer processes
GPU processes
network processes
shared memory
synchronization

This connects directly back to the systems programming knowledge you wanted to develop.

PHASE 24 — Build Your Own JS Engine
This is optional and very advanced.
Only do this after the browser itself works.
Then:

JavaScript source
 ↓
Lexer
 ↓
Parser
 ↓
AST
 ↓
Interpreter
 ↓
Runtime

Learn

lexical analysis
parsing
ASTs
scopes
environments
closures
garbage collection
bytecode
virtual machines
interpreters
JIT compilation

Then eventually:

JavaScript
 ↓
AST
 ↓
Bytecode
 ↓
VM

This can become a completely separate portfolio project.
PHASE 25 — GPU Rendering
Once your basic renderer works, you can investigate:

CPU rendering
     ↓
GPU rendering

Learn

GPU pipeline
vertex bufferstextures
shaders
compositing
rasterization
layers
double buffering
vsync

Eventually:

Layout
 ↓
Paint
 ↓
Display list
 ↓
Rasterization
 ↓
GPU
 ↓
Screen

PHASE 26 — Advanced Browser Architecture
At the very end, investigate:

multi-process architecture
site isolation
sandboxing
IPC
GPU process
renderer process
network process
storage process
browser profiles
permissions
service workers
Web Workers
WebSockets
WebRTC
IndexedDB
localStorage
session storage

extensions
At this point you're no longer making a "toy browser."
You're studying browser engineering.

