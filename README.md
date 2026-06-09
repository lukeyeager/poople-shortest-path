# poople shortest path

## Background

A friend sent me a link to this silly game: https://poople.io/

I had recently watched this Veritasium video about shortest path algorithms: https://youtu.be/kS-CGkiPetQ

And, finally, I once read this XKCD blog article where he mentioned that linux ships with a list of english words: https://blog.xkcd.com/2007/12/31/ghost/

So, naturally, I had to write this.

## Approach

I don't think A* will work here because there's not an objective way to measure the distance between nodes.
You could try to set a minimum distance between nodes by calculating how many characters are different, but I think we'll accomplish the same thing by doing a bidirectional Dijkstra.

## Usage

```bash
# utility to list all valid words at the given distance
# implemented with a breadth-first search
$ poople list 0
POOP
$ poople list 1
COOP
POOL
...

# utility to play the poople game, implemented with a bidirectional dijkstra
$ poople play POOP
0 POOP
$ poople play POOL
0 POOL
1 POOP
```
