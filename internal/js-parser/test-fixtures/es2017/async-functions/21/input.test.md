# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > 21`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSFunctionExpression {
				body: JSBlockStatement {
					body: [
						JSVariableDeclarationStatement {
							declaration: JSVariableDeclaration {
								kind: "var"
								declarations: [
									JSVariableDeclarator {
										id: JSBindingIdentifier {
											name: "async"
											loc: SourceLocation es2017/async-functions/21/input.js 1:18-1:23 (async)
										}
										loc: SourceLocation es2017/async-functions/21/input.js 1:18-1:23
									}
								]
								loc: SourceLocation es2017/async-functions/21/input.js 1:14-1:24
							}
							loc: SourceLocation es2017/async-functions/21/input.js 1:14-1:24
						}
						JSExpressionStatement {
							expression: JSAssignmentExpression {
								operator: "="
								left: JSAssignmentIdentifier {
									name: "async"
									loc: SourceLocation es2017/async-functions/21/input.js 1:25-1:30 (async)
								}
								right: JSNumericLiteral {
									value: 10
									loc: SourceLocation es2017/async-functions/21/input.js 1:33-1:35
								}
								loc: SourceLocation es2017/async-functions/21/input.js 1:25-1:35
							}
							loc: SourceLocation es2017/async-functions/21/input.js 1:25-1:35
						}
					]
					directives: []
					loc: SourceLocation es2017/async-functions/21/input.js 1:12-1:37
				}
				head: JSFunctionHead {
					async: false
					generator: false
					hasHoistedVars: true
					params: []
					loc: SourceLocation es2017/async-functions/21/input.js 1:9-1:11
				}
				loc: SourceLocation es2017/async-functions/21/input.js 1:1-1:37
			}
			loc: SourceLocation es2017/async-functions/21/input.js 1:0-1:38
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2017/async-functions/21/input.js>
	loc: SourceLocation es2017/async-functions/21/input.js 1:0-1:38
}
```

### `diagnostics`

```

```