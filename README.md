## Instalation
...

## Usage

```
cargo run train
```
seeds faces and data of few people to db.


```
cargo run cli_trainer
```
train app by using command line interface.


```
cargo run processor path/to/photo.jpg
```
recognizes all face on given photo. Result is a vector of names and difference between closest matched face from database: `[("John Dou", 0.25761012784916366)]`


## Config

```
DATABASE_URL=./storage/db/database.sql
DATABASE_FOLDER=./storage/db/
MIGRATIONS_DIR=./migrations
```
can be set in `.env` file.

`migration` folder should include structure of db to create.


