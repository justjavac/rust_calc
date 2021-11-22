input: `"1 + 2 * 3 + 4"`

output:

```
- ROOT
  - OPERATION
    - OPERATION
      - "1" NUMBER
      - "+" ADD
      - OPERATION
        - "2" NUMBER
        - "*" MUL
        - "3" NUMBER
    - "+" ADD
    - "4" NUMBER
```