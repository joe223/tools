# `harness.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/compiler/lint/rules/harness.test.ts --update-snapshots` to update.

## `js/noUnusedTemplateLiteral`

### `0`

```

 lint/js/noUnusedTemplateLiteral/reject/1/file.ts:1:12 lint/js/noUnusedTemplateLiteral  FIXABLE  ━━━

  ✖ Do not use template literals if interpolation and special-character handling are not needed.

    const foo = `bar`
                ^^^^^

  ℹ Safe fix

  - `bar`
  + "bar"


```

### `0: formatted`

```ts
const foo = "bar";

```

### `1`

```

 lint/js/noUnusedTemplateLiteral/reject/2/file.ts:1:12 lint/js/noUnusedTemplateLiteral  FIXABLE  ━━━

  ✖ Do not use template literals if interpolation and special-character handling are not needed.

    const foo = `bar `
                ^^^^^^

  ℹ Safe fix

  - `bar·`
  + "bar·"


```

### `1: formatted`

```ts
const foo = "bar ";

```

### `2`

```

```

### `2: formatted`

```ts
const foo = `bar
`;

```

### `3`

```

```

### `3: formatted`

```ts
const foo = `"bar"`;

```

### `4`

```

```

### `4: formatted`

```ts
const foo = `'bar'`;

```