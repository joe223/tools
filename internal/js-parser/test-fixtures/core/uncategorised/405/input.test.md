# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 405`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "a"
				loc: SourceLocation core/uncategorised/405/input.js 1:0-1:1 (a)
			}
			loc: SourceLocation core/uncategorised/405/input.js 1:0-1:1
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "b"
				loc: SourceLocation core/uncategorised/405/input.js 1:2-1:3 (b)
			}
			loc: SourceLocation core/uncategorised/405/input.js 1:2-1:4
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
				message: RAW_MARKUP {value: "Expected a semicolon or a line terminator"}
			}
			location: {
				language: "js"
				path: UIDPath<core/uncategorised/405/input.js>
				end: Position 1:1
				start: Position 1:2
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/405/input.js>
	loc: SourceLocation core/uncategorised/405/input.js 1:0-1:4
}
```

### `diagnostics`

```

 core/uncategorised/405/input.js:1:2 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected a semicolon or a line terminator

    a b;
      ^


```