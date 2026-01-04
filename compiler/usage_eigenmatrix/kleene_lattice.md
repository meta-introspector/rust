# Kleene Lattice - Regex Feature Hierarchy

```
Level 6: Advanced     (L|L)*  L\1  L(?=L)
         │              │      │      │
Level 5: Groups       (L)    L|L   (L|L)
         │              │      │      │
Level 4: Anchors      ^L     L$    ^L$
         │              │      │      │
Level 3: Quantifiers  L?     L*     L+
         │              │      │      │
Level 2: Classes      [L]    \d     \w
         │              │      │      │
Level 1: Atoms         L      .      
         │              │      │      
Level 0: Empty         ∅             
```

## Language Hierarchy

### Level 0

| Language | Features | Examples |
|----------|----------|----------|
| ∅ | empty | (no matches) |

### Level 1

| Language | Features | Examples |
|----------|----------|----------|
| L | literal | a, hello, 123 |
| . | dot | ., any single char |

### Level 2

| Language | Features | Examples |
|----------|----------|----------|
| [L] | char_class, literal | [abc], [0-9], [a-z] |
| \d | digit, literal | \d, 0, 9 |
| \w | literal, word | \w, a, Z, _ |

### Level 3

| Language | Features | Examples |
|----------|----------|----------|
| L? | literal, optional | a?, colou?r |
| L* | literal, star | a*, .*, \w* |
| L+ | literal, plus | a+, .+, \w+ |
| [L]* | char_class, star | [abc]*, \d* |
| \w+ | plus, word | \w+, hello, var123 |

### Level 4

| Language | Features | Examples |
|----------|----------|----------|
| ^L | literal, start_anchor | ^hello, ^\w+ |
| L$ | end_anchor, literal | world$, \d+$ |
| ^L$ | end_anchor, literal, start_anchor | ^hello$, ^\w+$ |

### Level 5

| Language | Features | Examples |
|----------|----------|----------|
| (L) | group, literal | (abc), (\w+) |
| L|L | alternation, literal | a|b, cat|dog, \d+|\w+ |
| (L|L) | alternation, group | (a|b), (cat|dog)+ |

### Level 6

| Language | Features | Examples |
|----------|----------|----------|
| (L|L)* | alternation, group, star | (a|b)*, (\w+|\d+)* |
| L\1 | backreference, literal | (\w+)\1, ([a-z])\1 |
| L(?=L) | literal, lookahead | \w+(?=@), test(?=ing) |

