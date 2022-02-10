# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > numeric-separator > invalid-31`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSNumericLiteral {
				value: 11
				loc: SourceLocation experimental/numeric-separator/invalid-31/input.js 1:0-1:4
			}
			loc: SourceLocation experimental/numeric-separator/invalid-31/input.js 1:0-1:5
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "Invalid or unexpected int token"}
			}
			location: {
				language: "js"
				path: UIDPath<experimental/numeric-separator/invalid-31/input.js>
				end: Position 1:0
				start: Position 1:0
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<experimental/numeric-separator/invalid-31/input.js>
	loc: SourceLocation experimental/numeric-separator/invalid-31/input.js 1:0-2:0
}
```

### `diagnostics`

```

 experimental/numeric-separator/invalid-31/input.js:1 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Invalid or unexpected int token

    1__1;
    ^


```