Branch-free 4-element LRU cache for use with LZ77-style de/compressors

Use Cases
=========

This code will be useful to very few people. As mentioned above, it is intended
for use in LZ77-style coders. It is, of course, inspired by LZMA, which keeps
track of the four most recently used match offsets, and uses a
least-recently-used eviction policy.

In principle, you want to keep the elements ordered by their last access time,
and index them by this ordering. That allows you to always evict the last
element, and yields best compression, because recently used matches are more
likely to be used.

Because the decoder never actually 'searches' the cache, that particular
operation is not optimized as well, making it hard to imagine this being used
in other contexts.

How it Works
============

Even for small caches, using a linked list or shifting around arrays seems
wasteful, which is why this implementation does not do that.

Instead, the four most recently used elements are stored in a flat array, with
no guarantees made about their order. Since there are only `4!` permutations
possible, we keep track of their conceptual order using a finite state machine.

The integer representations of the individual states are chosen such that the
two least significant bits can be used to index the unordered array to retrieve
the most recently used element.

Since retrieving any other element will make that the new most recently used
element, we just transition to the corresponding new state before using the
same bits.

When inserting a new element, the least recently used one is evicted, and its
slot reused for the new element, which is now the most recently used. Thus we
can use the same transition as if we wanted to reuse the evicted element,
before using the two lowest bits to determine which slot to overwrite.

New states are determined from the old state and the conceptual index, using a
constant two dimensional array, which was generated using a simple C program
(see `tablegen.c`). Since it is 24 * 4 = 96 bytes large, it will fit in one or
two cache lines on most modern CPUs.

Alternative Cache Sizes
=======================

* If you want a two element cache, just use a boolean and roll the 'transition
  table' by hand -- there's no need for a lookup table then!
* If you're implementing LZX, which has a 3-element cache, or want a 5-element
  cache for your codec, this code is pretty easy to modify for those cases.
  It's noteworthy that you can no longer retrieve the slot index using bitwise
  operations in these cases (at least, not without taking a penalty on the
  table size); instead, a modulo operation is required.
* If you want larger cache sizes, you're probably looking at the wrong library.
  There are `n!` permutations, all of which need a state ID, and the lookup
  table contains `n! * n` elements -- each of which needs to be large enough to
  hold a state ID! Thus, the lookup table for an 8-element cache would be
  almost a megabyte in size, at which point you're better off using a different
  approach.
* If you want arbitrary `n`, you'll have to store multiple tables, or generate
  them at runtime. This library will _definitely_ not be useful to you.

Thus, I didn't bother generalizing this code any further than it already is.