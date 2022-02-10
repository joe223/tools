# `harness.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/compiler/lint/rules/harness.test.ts --update-snapshots` to update.

## `js/noNestedTernary`

### `0`

```

 lint/js/noNestedTernary/reject/1/file.ts:1:24 lint/js/noNestedTernary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Nesting ternary expressions can make code more difficult to understand.

    let thing = foo ? bar : baz === qux ? quxx : foobar;
                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^


```

### `0: formatted`

```ts
let thing = foo ? bar : baz === qux ? quxx : foobar;

```

### `1`

```

 lint/js/noNestedTernary/reject/2/file.ts:1:36 lint/js/noNestedTernary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Nesting ternary expressions can make code more difficult to understand.

    let thing = foo ? bar ? boo : foo : baz ? boo : foo;
                                        ^^^^^^^^^^^^^^^

 lint/js/noNestedTernary/reject/2/file.ts:1:18 lint/js/noNestedTernary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Nesting ternary expressions can make code more difficult to understand.

    let thing = foo ? bar ? boo : foo : baz ? boo : foo;
                      ^^^^^^^^^^^^^^^


```

### `1: formatted`

```ts
let thing = foo ? bar ? boo : foo : baz ? boo : foo;

```

### `2`

```

 lint/js/noNestedTernary/reject/3/file.ts:1:6 lint/js/noNestedTernary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Nesting ternary expressions can make code more difficult to understand.

    foo ? baz === qux ? quxx() : foobar() : bar();
          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


```

### `2: formatted`

```ts
foo ? baz === qux ? quxx() : foobar() : bar();

```

### `3`

```

```

### `3: formatted`

```ts
let thing = foo ? bar : foobar;

```

### `4`

```

```

### `4: formatted`

```ts
let thing = foo ? bar || boo : foo || bar;

```

### `5`

```

```

### `5: formatted`

```ts
let thing = foo ? bar && boo : foo && bar;

```

### `6`

```

```

### `6: formatted`

```ts
let thing = foo || baz ? bar || boo : foo || bar;

```

### `7`

```

```

### `7: formatted`

```ts
let thing = foo && baz ? bar || boo : foo && bar;

```

### `8`

```

```

### `8: formatted`

```ts
let thing = foo || baz ? bar || boo : foo && bar;

```

### `9`

```

```

### `9: formatted`

```ts
if (foo) {
	thing = bar;
} else if (baz === qux) {
	thing = quxx;
} else {
	thing = foobar;
}

```