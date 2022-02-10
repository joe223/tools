# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 388`

### `ast`

```javascript
JSRoot {
	body: [
		JSBlockStatement {
			body: [
				JSExpressionStatement {
					expression: JSReferenceIdentifier {
						name: "set"
						loc: SourceLocation core/uncategorised/388/input.js 1:2-1:5 (set)
					}
					loc: SourceLocation core/uncategorised/388/input.js 1:2-1:5
				}
				JSExpressionStatement {
					expression: JSNumericLiteral {
						value: 1
						loc: SourceLocation core/uncategorised/388/input.js 1:6-1:7
					}
					loc: SourceLocation core/uncategorised/388/input.js 1:6-1:7
				}
			]
			directives: []
			loc: SourceLocation core/uncategorised/388/input.js 1:0-1:9
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
				path: UIDPath<core/uncategorised/388/input.js>
				end: Position 1:5
				start: Position 1:6
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/388/input.js>
	loc: SourceLocation core/uncategorised/388/input.js 1:0-1:9
}
```

### `diagnostics`

```

 core/uncategorised/388/input.js:1:6 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected a semicolon or a line terminator

    { set 1 }
          ^


```