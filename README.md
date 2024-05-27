# pear-search
it's a local search engine to index your bookmarks and maybe RSS feeds?

prototypes
- [X] given a URL, fetch text
- [ ] given text, build an index that points to a URL
- [ ] read a bookmarks file and convert that to URLs
  progress! able to pull JSON from firefox bookmarks
- [ ] run a local server that displays exciting search results


# pieces
- database should be the indexes
- hash response contents, per uri AND by time!
- save the entire page? not everyone has space
  - content defined chunking? that would make updates easy!
- sqlite? boring old file?
- search: reverse index
  - can this be fuzzy? levenshtein distance?
	DFA that traverses the search space (like a prefix tree)
  - vector search? EXPERIMENTAL

# why bookmarks?

probably should not be bookmarks?
browser extension would be better?
as long as there's some way to say "add this to my local search collection" with *one click*

# blue sky ideas

- search your local index, and your friends local indexes!
- find more friends by local index similarity!
