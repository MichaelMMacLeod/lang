;;; WORK IN PROGRESS ;;;

I want a safe programming language which is expressive enough to
represent anything I want to do. This is different from, for example,
C, which is not a safe programming language (e.g., adding two integers
could cause undefined behavior). This is also different from Rust,
which includes the 'unsafe' keyword, allowing (among other things) the
dereference of raw pointers which can lead to undefined behavior if
the pointer in question does not satisfy some rules that are not
checked by the compiler.

It becomes clear that the question of "which type system to use for
this language" is impossible to answer in the general case. 
