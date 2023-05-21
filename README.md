## Instalation
read `Dockerfile` [TODO: fix dlib compilation in docker image]

## Usage

### Trainer

seed faces and data of few people to db:

```
cargo run train path_to_dir/

# where directory "path_to_dir" has structure like:
# path_to_dir/person_name/1.jpg
# path_to_dir/person_name/2.jpg
# path_to_dir/john_dou/1.jpg
```
Uses last dir as a person name.

### CLI trainer
train app by using command line interface:
```
cargo run cli_trainer
```

### Recognition
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


