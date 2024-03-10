phorge
------
phorge is an open source entropy mutator. It takes an input file and produces 1 KB of white noise, along with a phext coordinate.

sample.data.txt: an idealized log of player activity for a text adventure game (just an example)
main.rs: phorge cli utility

nova fox white noise
--------------------
white noise in nova fox is designed to drive world geometry in a deterministic fashion. this is used as a sort of 'quantum foam' around the rest of the game. a record of the player's actions is transmuted with the help of 24 SHA-512 hashes to produce a 1 KB block of bits to drive entropy generation in a verifiable way.

hash dimensions
---------------
there are a total of 24 hash dimensions, detailed below. each dimension produces a string that is then hashed by SHA-512. The first two digits are dropped (some of which are reclaimed with hash 21). We then pack the remaining 126 bytes into a 42-byte base-48 encoding using characters a-x and A-X. This gives us 1,008 bytes of entropy with 16 bytes reserved for the "NovaFox1........" header.

1. nova fox version - allows us to track when rooms were generated - the full major.minor.revision.build number is used as input.
2. 

example hash
------------

1. "Nova Fox Version 1.0.0.1" => 80aaa1b17f92c6a8a02f2ced84cf17dd3d3e1b443b7d16d26e2088578b0c2247945147ab0ef87149b0aab31dd3b6b7178cd898d46291c5bb6883a6d9c4d59fc4 => 
