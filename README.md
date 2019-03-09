# yaml-to
Convert YAML into database schemas, code, and not sure what else

## Goal

The goal of this project is to allow someone to generate a database schema for MySQL, Postgres and SQLite.

## targeted command line arguments

```
./yaml-to schema mysql|postgres|sqlite <path to yaml file> -o | --output db|file

and 

./yaml-to code go|rust <path to yaml file> -o | --output-dir <path to directory where code files should be saved>
```

## Stretch goal

...is to use this as a library for a webassembly tool that will generate schemas and code to interact with the schemas
