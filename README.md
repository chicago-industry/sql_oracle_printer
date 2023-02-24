# sql_oracle_printer
This program reads an SQL script file and executes the query using [Oracle Instant Client](https://www.oracle.com/database/technologies/instant-client.html), printing the resulting data to a console. 

## Usage
```
sql_oracle_printer [OPTIONS] --file <FILE>
```

```
Options:
- -c, --config <FILE>  .env file, which should contain ORACLE_USER, ORACLE_PASSWORD AND ORACLE_ADRES environment variables
- -f, --file <FILE>    SQL query file
- -h, --help           Print help
```
