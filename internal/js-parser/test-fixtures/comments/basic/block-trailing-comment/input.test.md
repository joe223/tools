# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `comments > basic > block-trailing-comment`

### `ast`

```javascript
JSRoot {
	body: [
		JSBlockStatement {
			body: [
				JSExpressionStatement {
					trailingComments: ["0"]
					expression: JSCallExpression {
						arguments: []
						callee: JSReferenceIdentifier {
							name: "a"
							loc: SourceLocation comments/basic/block-trailing-comment/input.js 2:4-2:5 (a)
						}
						loc: SourceLocation comments/basic/block-trailing-comment/input.js 2:4-2:7
					}
					loc: SourceLocation comments/basic/block-trailing-comment/input.js 2:4-2:8
				}
			]
			directives: []
			loc: SourceLocation comments/basic/block-trailing-comment/input.js 1:0-4:1
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: "comment"
			loc: SourceLocation comments/basic/block-trailing-comment/input.js 3:4-3:13
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<comments/basic/block-trailing-comment/input.js>
	loc: SourceLocation comments/basic/block-trailing-comment/input.js 1:0-5:0
}
```

### `diagnostics`

```

```