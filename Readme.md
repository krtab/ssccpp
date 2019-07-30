SSCCPP, the Simple Switch Cases Configuration PreProcessor
==========================================================

This small utility makes it easy to have a single file that can be adapted to different cases.
Typical use is to manage dotfiles, by allowing to have only one git branch which is then preprocessed before being used as an actual configuration file.

It is written in Rust and provides a library as well as two binaries: `ssccpp` and `ssccpp-batch`. `ssccpp`  processes a single file while `ssccpp-batch` process all files in a directory, copying its structure to an other one.

Concept
--------

`ssccpp` works on a file where some lines have been turned into switch statements.
These switch statements are recognized by looking for a delimiter (by default `>>>>>>>>` ie. 8 times the '>' character).

A switch block is composed of, in order:

1. One or more "specfic" case(s): `>>>>>>>> ident1, ident2, ...`
2. One "otherwise" case: `>>>>>>>> *`
3. And ending delimiter: `>>>>>>>>`

When reaching such a block, ssccpp will check, for each *specific* case if one of the `ident` matches the searched ident (by default the hostname), and if yes, will emit the text that follows.
When reaching the *otherwise* block, it will emit the text that follows if none of the *specific* statements before were entered. Finally, when reaching the ending delimiter, it will print all following text, until a new block is encountered.

Examples
--------

```
This line will be displayed everywhere.
>>>>>>>> foo, bar
This line will be displayed on foo and bar only.
>>>>>>>> ga, bu, zo
This line will be displayed on ga, bu and zo, but not foo or bar.
>>>>>>>> foo, zo
This line will be displayed on foo and zo.
>>>>>>>> *
This line will be displayed on anything that is not foo, bar, ga, bu or zo.
>>>>>>>>
This line will be displayed everywhere.
```
