Pinewood Derby
==============

A toy race condition detector for a toy language.

The ultimate goal is to have a C-like language where we can statically detect
if race conditions will occur and (ideally) present a proof of race condition.

Of course, in general, static deadlock detection is impossible and race condition
detection is NP-hard in programs that use multiple semaphores, so this problem
is intractable. I hope to be able to at least work around this a little by the
following:

- Restricting the maximum size of programs so that some brute-force searches 
  can be feasibly carried out.
- Using a restricted subset of the C language to simplify analysis.
- Possibly require annotations from the user.

Am I embarking on a fool's task? Quite probably. Let's see what happens.
