# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 246`

### `ast`

```javascript
JSRoot {
	body: [
		JSForInStatement {
			body: JSExpressionStatement {
				expression: JSCallExpression {
					arguments: [
						JSReferenceIdentifier {
							name: "x"
							loc: SourceLocation core/uncategorised/246/input.js 1:23-1:24 (x)
						}
					]
					callee: JSReferenceIdentifier {
						name: "process"
						loc: SourceLocation core/uncategorised/246/input.js 1:15-1:22 (process)
					}
					loc: SourceLocation core/uncategorised/246/input.js 1:15-1:25
				}
				loc: SourceLocation core/uncategorised/246/input.js 1:15-1:26
			}
			left: JSAssignmentIdentifier {
				name: "x"
				loc: SourceLocation core/uncategorised/246/input.js 1:4-1:5 (x)
			}
			right: JSReferenceIdentifier {
				name: "list"
				loc: SourceLocation core/uncategorised/246/input.js 1:9-1:13 (list)
			}
			loc: SourceLocation core/uncategorised/246/input.js 1:0-1:26
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/246/input.js>
	loc: SourceLocation core/uncategorised/246/input.js 1:0-1:26
}
```

### `diagnostics`

```

```