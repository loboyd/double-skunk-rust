# Things to use Rust traits for
* The user-interface and the core client logic should be completely separate
  from one another, as to allow either implementation to be swapped out for
  another with little difficulty.
* The "opponent" should also be sufficiently abstracted that an AI can be
  played against the same as another player over a socket connection.

# Data structures
* There should probably be a `Card` struct, and possibly a `Hand` struct
* It may be a good idea to explicit track game state, which might include the
  following items:
  * ID? May be able to recover from disconnects
  * hand
  * crib
  * table
  * player to play
  * both scores

