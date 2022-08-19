# Context-Free Grammars + Chomsky Hierarchy

- lexical grammar - the rules for how characters get grouped into tokens - was called a regular language.

- context-free grammar (CFG) -

- formal grammar - takes a set of atmoic pieces it calls its "alphabet". Then it defines a (usually infinite) set of "strings" that are "in" the grammar. Each string is a sequence of "letters" in the alphabet.

> in our scanner's grammar, the alphabet consists of individual characters and the strings are the valid lexems - roughly "words". .. Now each "letter" in the alphabet is an entire token and a "string" is a sequence of `tokesn` - an entire expression

# Rules for grammars

- you create a grammar from a finite set of rules. If you start with the rules, you can use them to generate strings that are in the grammar.

- rules are called **productions**

- **head** - the name of the **production**
- **body** - describes what the name(it) generates
- **terminal** - is a letter from the grammar's alphabet. You can think of it like a literal value. In the syntactic grammar we're defining, the terminals are individual lexemes - tokesn coming from the scanner like `if` or `1234`
- **nonterminal** - is a named reference to another rules in the grammar.

_you may have multiple rules with the same name. Whne you reach a nonterminal with that name, you are allowed to pick any of the rules for it, whichever floats your boat._

_Backus-Naur form (BNF)_ - a way to codify a grammar - everyone uses this or some flavor of it. (even probably Rust)

> recursion in the grammar is a good sign that the language being defined is context-free instead of regular

# Enhancing Our Notation

- nothing really in here... mostly just stuff...

# A Grammar of Lox expressions

- only worrying about a handful of expressions (statements don't return anything expressions do)

- **literals** - number, strings, bools, and nil
- unary expressions - a prefix `!` to perform a logical not, and `-` to negate a number
- binary expressions - the infix arithmetic (+,-,\*/) and logical operators (==, !=, <,<=.>,>=)
- parentheses - a pair of `(` and `)` wrapped around an expression

so we can do something like

`1 - (2*3) < 4 == false`

# implementing syntax trees

we're defining an **abstract syntax tree**. In a **parse tree**, every single grammar production becomes a node in the tree. An AST elides productions that aren't needed by later phases.

# Disoriented objects

- basically the code we wrote doesn't have any behavior it's basically being marked? Idk the behavior will have to come somehow though.

# metaprogamming the trees

- basically we wrote a script to generate some trees for us. Idk if this data model will even work for rust so we'll see 🙃

# The Visitor Pattern

the expression problem - we have a handful of types (classes) and a handful of high-level operations like "interpret". for each pair of type and operation, we need a specific implementation. Picutre a table

an object-oriented langaugae like java assumes that all of the code in one row naturally hangs together. It figure all the thing you do with a type are likely related to each other, and the language makes it easy to define them together as methods inside the same class.

functional paradigm languages in the ML family flip that around. There, you don't have classes with methods. Types and functions are totally distinct. To implement an operation for a number of different types, you define a single function. In the body of that function, you use pattern matching -- sort of a type-based switch on steroids -- to implement the operation for each type all in one place.

so the difference between OOP styles vs functional style for adding new behavior is in

- OOP you modify every single class - you add that mehtod to every single class
- in functional you have one method and do "pattern matching" in that function on the types and implement the operation for each type in one place. - but if you add a new type then you have to go and modify all of your functions to add the new behavior for that type.

too add new behavior

- OOP - you need to modify all of your classes by adding a new function
- Functional - you just create a new method with your pattern matching and do it on all the types - the catch here is that adding a new type is annoying, because then you have to go modify all of the methods to account for this type now

- So visitor pattern is add functional style to OOP? I don't really get it. Will need to re-read

e.g. pastries: beignets and crullers

- The visitor pattern is all about allowing us to approximate the functional style in an OOP language.

# Questions

- what is the difference between a lexical grammar vs a syntactic grammar?
- what is a context-free grammar and how is it different from other grammars?
- the grammar for expressions that we come up with is `ambigious` what does that mean? I think it means that there is more than one way to evaluate a parse tree, and that can lead to different results - I'm assuming we will want to make it unambigious.
- what is the difference between "overriding" and "overloading" in OOP - I've forgotten - overloading is simply when you have methods with the same name, but different parameters / arguments. When you have the exact same signaute in a subclass and superclass that's overriding.

# random

- we are going to use a tree, but he's saying that bytecode is another representation - and I mean I've written something like this before. I didn't need to ... well ... have a tree to come up with the bytecdoe - and why would I? If I can just directly convert it easily?

- I just had an idea - the way I like to learn is by doing projects - it's by learning the theory and then actually doing a project - that either is assigned to me or I already have something in mind - so why don't I make courses for things that I want to learn about anyways? e.g. learning how to build a programming language, databases, and honestly anything else. Learning is fucking cool, and it's way cooler when you can actually apply it to something.
- what about a language for twitter or something like that? I mean idk you tweet it and it evaluates it?
