# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > arrow-functions > object-rest-spread`

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
							name: "foo"
							loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:4-1:7 (foo)
						}
						init: JSArrowFunctionExpression {
							body: JSBlockStatement {
								body: []
								directives: []
								loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:37-3:1
							}
							head: JSFunctionHead {
								async: false
								hasHoistedVars: false
								params: [
									JSBindingObjectPattern {
										properties: [
											JSBindingObjectPatternProperty {
												key: JSStaticPropertyKey {
													value: JSIdentifier {
														name: "title"
														loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:14-1:19 (title)
													}
													loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:14-1:19
												}
												value: JSBindingIdentifier {
													name: "title"
													loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:14-1:19 (title)
												}
												loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:14-1:19
											}
										]
										rest: JSBindingIdentifier {
											name: "other"
											loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:24-1:29 (other)
										}
										loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:12-1:31
									}
								]
								loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:10-1:36
							}
							loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:10-3:1
						}
						loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:4-3:1
					}
				]
				loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:0-3:2
			}
			loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:0-3:2
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/arrow-functions/object-rest-spread/input.js>
	loc: SourceLocation es2015/arrow-functions/object-rest-spread/input.js 1:0-4:0
}
```

### `diagnostics`

```

```