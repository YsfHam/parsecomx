# Parsecomx

Parser combinator of monads implementation with Rust

## Goal

Parser combinators are parsers that can be combined together to make a more powerful parser. We are going to use the functional monad design pattern.

## Design

In this implementation we are going to define a trait Parser, which all combinators and specific parsers should implement. We are going to follow the same pattern the Iterator trait follows.

The trait will contain one mandatory method for parsing, and other methods for each combinator

## Combinators

### AndThen

This combinator takes two parsers as arguments.
It applies the input to the first and the rest to the other, the returns a tuple of the results

### OrElse

Takes two parsers. If the first fails, it tries the second

### Map

Maps the result of the parser

### MapError

Maps the error when the parser fails

### MapResult

Maps the result of the parser to a result, then flatten it

### Many1

Applies the parser as much as possible and returns a list of the parsed elements. At least one element is parsed.

### Many

Applies the parser as much as possible and returns a list of the parsed elements. It never fails!

### ThenConsume

Takes two parsers, and ignores the result of the second parser

### ThenParse

Takes two parsers, and ignores the result of the first parser

### ParseIf

Takes a predicate which returns a result and an error.
When the predicate returns false the parser fails.

### Optional

Returns an option of the parser's result.
If the parser succeded returns Some(result)
Otherwise returns None.

### Flatten

Flats the parser result when it is a parser.

### FlatMap

This is equivalent to parser.map(f).flatten()

### SepBy

Takes a parser as a separator and returns a list of the parsed elements that are separated by the result of the second element
