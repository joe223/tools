# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > 24`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "obj"
							loc: SourceLocation es2017/async-functions/24/input.js 1:4-1:7 (obj)
						}
						init: JSObjectExpression {
							properties: [
								JSObjectMethod {
									kind: "method"
									key: JSStaticPropertyKey {
										value: JSIdentifier {
											name: "async"
											loc: SourceLocation es2017/async-functions/24/input.js 1:12-1:17 (async)
										}
										loc: SourceLocation es2017/async-functions/24/input.js 1:12-1:17 (async)
									}
									body: JSBlockStatement {
										body: []
										directives: []
										loc: SourceLocation es2017/async-functions/24/input.js 1:20-1:22
									}
									head: JSFunctionHead {
										async: false
										generator: false
										hasHoistedVars: false
										params: []
										loc: SourceLocation es2017/async-functions/24/input.js 1:17-1:19
									}
									loc: SourceLocation es2017/async-functions/24/input.js 1:12-1:22
								}
							]
							loc: SourceLocation es2017/async-functions/24/input.js 1:10-1:24
						}
						loc: SourceLocation es2017/async-functions/24/input.js 1:4-1:24
					}
				]
				loc: SourceLocation es2017/async-functions/24/input.js 1:0-1:25
			}
			loc: SourceLocation es2017/async-functions/24/input.js 1:0-1:25
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<es2017/async-functions/24/input.js>
	loc: SourceLocation es2017/async-functions/24/input.js 1:0-1:25
}
```

### `diagnostics`

```

```