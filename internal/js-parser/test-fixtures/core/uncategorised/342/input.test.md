# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 342`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSBinaryExpression {
				operator: "+"
				left: JSReferenceIdentifier {
					name: "foo"
					trailingComments: ["0"]
					loc: SourceLocation core/uncategorised/342/input.js 1:0-1:3 (foo)
				}
				right: JSReferenceIdentifier {
					name: "baz"
					leadingComments: ["0"]
					loc: SourceLocation core/uncategorised/342/input.js 2:1-2:4 (baz)
				}
				loc: SourceLocation core/uncategorised/342/input.js 1:0-2:4
			}
			loc: SourceLocation core/uncategorised/342/input.js 1:0-2:4
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: "bar"
			loc: SourceLocation core/uncategorised/342/input.js 1:4-1:11
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/342/input.js>
	loc: SourceLocation core/uncategorised/342/input.js 1:0-2:4
}
```

### `diagnostics`

```

```