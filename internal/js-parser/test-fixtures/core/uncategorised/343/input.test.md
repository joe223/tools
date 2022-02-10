# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 343`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			trailingComments: ["0"]
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentIdentifier {
					name: "x"
					loc: SourceLocation core/uncategorised/343/input.js 1:0-1:1 (x)
				}
				right: JSBinaryExpression {
					operator: ">"
					left: JSUpdateExpression {
						operator: "--"
						prefix: false
						argument: JSReferenceIdentifier {
							name: "y"
							loc: SourceLocation core/uncategorised/343/input.js 1:4-1:5 (y)
						}
						loc: SourceLocation core/uncategorised/343/input.js 1:4-1:7
					}
					right: JSNumericLiteral {
						value: 10
						loc: SourceLocation core/uncategorised/343/input.js 1:8-1:10
					}
					loc: SourceLocation core/uncategorised/343/input.js 1:4-1:10
				}
				loc: SourceLocation core/uncategorised/343/input.js 1:0-1:10
			}
			loc: SourceLocation core/uncategorised/343/input.js 1:0-1:11
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: " nothing"
			loc: SourceLocation core/uncategorised/343/input.js 2:1-2:12
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/343/input.js>
	loc: SourceLocation core/uncategorised/343/input.js 1:0-2:12
}
```

### `diagnostics`

```

```