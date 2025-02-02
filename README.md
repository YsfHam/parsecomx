# Parsecomx

Parser combinator of monads implementation with Rust

## Goal

Parser combinators are parsers that can be combined together to make a more powerful parser. We are going to use the functional monad design pattern.

## Design

In this implementation we are going to define a trait Parser, which all combinators and specific parsers should implement. We are going to follow the same pattern the Iterator trait follows.

The trait will contain one mandatory method for parsing, and other methods for each combinator

## Combinators

[X] AndThen -> takes two parser, apply the input to the first one and the rest to the second and return both of them

[X] OrElse -> when the first parser fails, we take the result of the second parser

[X] Map -> apply a function to the result of a parser

[X] Many -> apply the parser as much as possible return a list of parsed elements

[X] ThenConsume -> same as AndThen but ignores the result of the second parser

[X] ThenParse -> same as AndThen but ignores the result of the first parser

[X] ParseIf -> takes a predicate and fallback error if the predicate is false
