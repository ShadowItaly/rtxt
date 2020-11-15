# rtxt
![CI](https://github.com/ShadowItaly/rtxt/workflows/CI/badge.svg)

A text editor written in rust, the editor is heavily inspired by vim and should
provide a lightweight terminal editor with a basic set of features used by
almost every programmer today. Fuzzy file finding, git integration and a well
designed plug in system.

## Implementation idea

### Plugin design
Most plugins should be written in rust, to have full access to the internals,
but also to provide the speed needed for a fine text editor.

### Buffer design
Every buffer is designed to consist of a linked list of lines and every line is
implemented by a gap buffer to increase the speed at which one can edit lines.
The performance for long lines is very bad, but long lines are a bad habit in
programming anyways.


