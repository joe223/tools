# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/codec-browsers/index.test.ts --update-snapshots` to update.

## `diagnostic > version`

### `diagnostics`

```

 diagnostic/version/input.txt:2 parse(browserquery) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected a browser version

    firefox >=


```

### `tokens`

```javascript
[
	String {
		value: "firefox"
		end: ZeroIndexedNumber<7>
		start: ZeroIndexedNumber<0>
	}
	GE {
		end: ZeroIndexedNumber<10>
		start: ZeroIndexedNumber<8>
	}
	EOF {
		end: ZeroIndexedNumber<11>
		start: ZeroIndexedNumber<11>
	}
]
```