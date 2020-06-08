# rodent

## Demo

- [http://49.232.102.172/](http://49.232.102.172/)

## Installation

Clone the repo and cd into the repo:

```shell
git clone https://github.com/garbein/rodent.git
cd rodent
```

Rename .env.sample to .env and edit the .env file:

```shell
mv .env.sample .env
vim .env
```

Import sql/schema.sql to database

```shell
mysql -uroot -p < sql/schema.sql
```

Import sql/sqlite.sql to database
```
sqlite3 rodent.db
sqlite> .read sql/sqlite.sql
```

Start the server:

```shell
cargo run
```
use other db
```
cargo run --features="sqlite" --no-default-features
```

## Client

### Is Running?

```shell
curl -X GET http://127.0.0.1:8415 -w '\n'
```
#### Response

drop the world

### Create

#### Request

```shell
curl -X POST http://127.0.0.1:8415/settings -w '\n' \
-H 'Content-Type: application/json' \
-d '{"name":"test","title":"a test title","content":"{\"setting_name\":\"test\",\"setting_title\":\"a setting title\"}"}' 
```

#### Response

```json
{"code":0,"message":"","data":null}
```

### Read

#### Request

```shell
curl -X GET http://127.0.0.1:8415/settings/1 -w '\n' 
```

#### Response

```json
{"code":0,"message":"","data":{"id":1,"name":"test","title":"a test title","content":"{\"setting_name\":\"test\",\"setting_title\":\"a setting title\"}","created_at":"2020-05-15 16:01:58","updated_at":""}}
```

### Update

#### Request

```shell
curl -X PUT http://127.0.0.1:8415/settings/1 -w '\n' \
-H 'Content-Type: application/json' \
-d '{"name":"test","title":"a test title","content":"{\"setting_name\":\"test\",\"setting_title\":\"a setting title\"}"}' 
```

#### Response

```json
{"code":0,"message":"","data":null}
```

### Delete

#### Request

```shell
curl -X DELETE http://127.0.0.1:8415/settings/1 -w '\n' \
-H 'Content-Type: application/json' 
```

#### Response

```json
{"code":0,"message":"","data":null}
```

### List

#### Request

```shell
curl -X GET 'http://127.0.0.1:8415/settings?page=1&size=20' -w '\n' \
-H 'Content-Type: application/json' 
```

#### Response

```json
{"code":0,"message":"","data":{"total":1,"rows":[{"id":1,"name":"test","title":"a test title","created_at":"2020-05-15 16:01:58","updated_at":""}]}}
```

### Frontend

#### Request

```shell
curl -X GET http://127.0.0.1:8415/frontend/1 -w '\n' 
```

#### Response

```json
{"code":0,"message":"","data":{"setting_name":"test","setting_title":"a setting title"}}
```