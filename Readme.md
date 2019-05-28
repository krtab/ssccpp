SSCCPP, the Simple Switch Cases Configuration PreProcessor
==========================================================

This small utility makes it easy to have a single file that can be adapted to different cases.
Typical use is to manage dotfiles, by allowing to have only one git branch which is then preprocessed before being use as an actual configuration file.

It is written in Rust and provides a library version on top of the binary.

Concept
--------

ssccpp works on file were some lines have been turned into switch statements.
These switch statements are recognized by looking for a delimiter (by default `>>>>>>>>` ie. 8 times the '>' character).
