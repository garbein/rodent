# rodent

## Installation

Clone the repo and cd into the repo:

```shell
git clone https://github.com/garbein/rodent.git
cd rodent
```

Edit the .env file:

```shell
vim .env
```

Import sql/schema.sql to database

```shell
mysql -uroot -p < sql/schema.sql
```

Start the server:

```shell
cargo run
```

## Client

### Is Running?

```shell
curl -X GET http://127.0.0.1:8080 -w '\n'
```
#### Response

drop the world

### Create

```shell
curl -X POST http://127.0.0.1:8080/frontend -w '\n' \
-H 'Content-Type: application/json' \
-d '{"name":"test","title":"a test title","content":"{\"setting_name\":\"test\",\"setting_title\":\"a setting title\"}"}' 
```

#### Response

```json
{"code":1,"message":"","data":null}
```

### GET

```shell
curl -X GET http://127.0.0.1:8080/frontend/test -w '\n' 
```

#### Response

```json
{"code":1,"message":"","data":{"setting_name":"test","setting_title":"a setting title"}}
```

### Update

```shell
curl -X PUT http://127.0.0.1:8080/frontend/test -w '\n' \
-H 'Content-Type: application/json' \
-d '{"name":"test","title":"a test title","content":"{\"setting_name\":\"test\",\"setting_title\":\"a setting title\"}"}' 
```

#### Response

```json
{"code":1,"message":"","data":null}
```

### Delete

```shell
curl -X DELETE http://127.0.0.1:8080/frontend/test -w '\n' \
-H 'Content-Type: application/json' 
```

#### Response

```json
{"code":1,"message":"","data":null}
```

