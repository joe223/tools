# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 10`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentIdentifier {
					name: "x"
					loc: SourceLocation core/uncategorised/10/input.js 1:0-1:1 (x)
				}
				right: JSArrayExpression {
					elements: [
						JSNumericLiteral {
							value: 42
							loc: SourceLocation core/uncategorised/10/input.js 1:6-1:8
						}
					]
					loc: SourceLocation core/uncategorised/10/input.js 1:4-1:11
				}
				loc: SourceLocation core/uncategorised/10/input.js 1:0-1:11
			}
			loc: SourceLocation core/uncategorised/10/input.js 1:0-1:11
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/10/input.js>
	loc: SourceLocation core/uncategorised/10/input.js 1:0-1:11
}
```

### `diagnostics`

```

```