# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 279`

### `ast`

```javascript
JSRoot {
	body: [
		JSTryStatement {
			block: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation core/uncategorised/279/input.js 1:4-1:7
			}
			handler: JSCatchClause {
				body: JSBlockStatement {
					body: [
						JSExpressionStatement {
							expression: JSCallExpression {
								arguments: [
									JSReferenceIdentifier {
										name: "e"
										loc: SourceLocation core/uncategorised/279/input.js 1:24-1:25 (e)
									}
								]
								callee: JSReferenceIdentifier {
									name: "say"
									loc: SourceLocation core/uncategorised/279/input.js 1:20-1:23 (say)
								}
								loc: SourceLocation core/uncategorised/279/input.js 1:20-1:26
							}
							loc: SourceLocation core/uncategorised/279/input.js 1:20-1:26
						}
					]
					directives: []
					loc: SourceLocation core/uncategorised/279/input.js 1:18-1:28
				}
				param: JSBindingIdentifier {
					name: "e"
					loc: SourceLocation core/uncategorised/279/input.js 1:15-1:16 (e)
				}
				loc: SourceLocation core/uncategorised/279/input.js 1:8-1:28
			}
			loc: SourceLocation core/uncategorised/279/input.js 1:0-1:28
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/279/input.js>
	loc: SourceLocation core/uncategorised/279/input.js 1:0-1:28
}
```

### `diagnostics`

```

```