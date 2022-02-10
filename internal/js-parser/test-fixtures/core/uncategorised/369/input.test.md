# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 369`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentIdentifier {
					name: "INVALID_PLACEHOLDER"
					loc: SourceLocation core/uncategorised/369/input.js 1:8-1:7
				}
				right: JSNumericLiteral {
					value: 10
					loc: SourceLocation core/uncategorised/369/input.js 1:10-1:12
				}
				loc: SourceLocation core/uncategorised/369/input.js 1:0-1:12
			}
			loc: SourceLocation core/uncategorised/369/input.js 1:0-1:12
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: [RAW_MARKUP {value: "Invalid left-hand side in "}, "assignment expression"]
			}
			location: {
				language: "js"
				path: UIDPath<core/uncategorised/369/input.js>
				end: Position 1:6
				start: Position 1:1
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/369/input.js>
	loc: SourceLocation core/uncategorised/369/input.js 1:0-1:12
}
```

### `diagnostics`

```

 core/uncategorised/369/input.js:1:1 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Invalid left-hand side in assignment expression

    (1 + 1) = 10
     ^^^^^


```