# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 243`

### `ast`

```javascript
JSRoot {
	body: [
		JSForStatement {
			body: JSEmptyStatement {
				loc: SourceLocation core/uncategorised/243/input.js 1:19-1:20
			}
			init: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentIdentifier {
					name: "x"
					loc: SourceLocation core/uncategorised/243/input.js 1:4-1:5 (x)
				}
				right: JSNumericLiteral {
					value: 0
					loc: SourceLocation core/uncategorised/243/input.js 1:8-1:9
				}
				loc: SourceLocation core/uncategorised/243/input.js 1:4-1:9
			}
			test: JSBinaryExpression {
				operator: "<"
				left: JSReferenceIdentifier {
					name: "x"
					loc: SourceLocation core/uncategorised/243/input.js 1:11-1:12 (x)
				}
				right: JSNumericLiteral {
					value: 42
					loc: SourceLocation core/uncategorised/243/input.js 1:15-1:17
				}
				loc: SourceLocation core/uncategorised/243/input.js 1:11-1:17
			}
			loc: SourceLocation core/uncategorised/243/input.js 1:0-1:20
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/243/input.js>
	loc: SourceLocation core/uncategorised/243/input.js 1:0-1:20
}
```

### `diagnostics`

```

```