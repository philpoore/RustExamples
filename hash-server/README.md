# Hash Server

Example HTTP Hash Server.

Very simple interface.

GET value
```
GET /key
=> 404 not found
=> 200 [data]
```

SET value
```
POST /key
[data]
=> 200 Okay
```