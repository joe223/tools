# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > yield > inside-arrow-inside-generator`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "fn"
				loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 1:10-1:12 (fn)
			}
			body: JSBlockStatement {
				body: [
					JSExpressionStatement {
						expression: JSArrowFunctionExpression {
							body: JSReferenceIdentifier {
								name: "yield"
								loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 2:8-2:13 (yield)
							}
							head: JSFunctionHead {
								async: false
								hasHoistedVars: false
								params: []
								loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 2:2-2:7
							}
							loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 2:2-2:13
						}
						loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 2:2-2:14
					}
					JSExpressionStatement {
						expression: JSArrowFunctionExpression {
							body: JSBlockStatement {
								body: [
									JSExpressionStatement {
										expression: JSReferenceIdentifier {
											name: "yield"
											loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 3:10-3:15 (yield)
										}
										loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 3:10-3:15
									}
								]
								directives: []
								loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 3:8-3:17
							}
							head: JSFunctionHead {
								async: false
								hasHoistedVars: false
								params: []
								loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 3:2-3:7
							}
							loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 3:2-3:17
						}
						loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 3:2-3:18
					}
				]
				directives: []
				loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 1:15-4:1
			}
			head: JSFunctionHead {
				async: false
				generator: true
				hasHoistedVars: false
				params: []
				loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 1:12-1:14
			}
			loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 1:0-4:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/yield/inside-arrow-inside-generator/input.js>
	loc: SourceLocation es2015/yield/inside-arrow-inside-generator/input.js 1:0-4:1
}
```

### `diagnostics`

```

```