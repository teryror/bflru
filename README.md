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

We use copies to physically move the cached values around in an array. The trick
to doing this in a branch free manner is to allocate more slots than will actually
be filled.

This technique was pointed out to me by Fabian Giesen, who in turn credits
Charles Bloom with it.

Note that the practicality of this approach very much depends on the size of the
cached type. It'll be fine for register-sized values, but for a more generalized
technique, have a look at the history of this repository.

In a previous version, I used a finite state machine to model permutations as
states and move-to-front operations as state transitions. I still like this
approach, but it requires a lookup table that has to be generated first, and is
tedious to prove correct.

The LUT also grows impractically fast as the cache size increases. If you reach
a point where the copies required for the current approach become too expensive,
a very similar FSM technique can be used, where new states are determined with
bitwise operations, rather than a table.