## Extended Construction Statement
NTS_MANGO -> NTS_STATEMENT_SUITE

## Block Statement Definition
NTS_STATEMENT_SUITE -> NTS_STATEMENT_LIST TS_NEWLINE

## Recursive Statement Definition
# Standard Statement List
NTS_STATEMENT_LIST -> NTS_STATEMENT TS_NEWLINE NTS_STATEMENT_LIST
NTS_STATEMENT_LIST -> NTS_STATEMENT

NTS_STATEMENT -> NTS_STATEMENT_SIMPLE

## Defining Statements
NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_ASSIGNMENT

## Assignment Statement
NTS_STATEMENT_ASSIGNMENT -> TS_IDENTIFIER TS_EQUALS TS_TERM