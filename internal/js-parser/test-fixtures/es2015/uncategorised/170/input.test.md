# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 170`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSFunctionExpression {
				id: JSBindingIdentifier {
					name: "x"
					loc: SourceLocation es2015/uncategorised/170/input.js 1:10-1:11 (x)
				}
				body: JSBlockStatement {
					body: []
					directives: []
					loc: SourceLocation es2015/uncategorised/170/input.js 1:21-1:23
				}
				head: JSFunctionHead {
					async: false
					generator: false
					hasHoistedVars: false
					params: [
						JSBindingObjectPattern {
							properties: [
								JSBindingObjectPatternProperty {
									key: JSStaticPropertyKey {
										value: JSIdentifier {
											name: "a"
											loc: SourceLocation es2015/uncategorised/170/input.js 1:14-1:15 (a)
										}
										loc: SourceLocation es2015/uncategorised/170/input.js 1:14-1:15
									}
									value: JSBindingIdentifier {
										name: "a"
										loc: SourceLocation es2015/uncategorised/170/input.js 1:14-1:15 (a)
									}
									loc: SourceLocation es2015/uncategorised/170/input.js 1:14-1:15
								}
								JSBindingObjectPatternProperty {
									key: JSStaticPropertyKey {
										value: JSIdentifier {
											name: "b"
											loc: SourceLocation es2015/uncategorised/170/input.js 1:17-1:18 (b)
										}
										loc: SourceLocation es2015/uncategorised/170/input.js 1:17-1:18
									}
									value: JSBindingIdentifier {
										name: "b"
										loc: SourceLocation es2015/uncategorised/170/input.js 1:17-1:18 (b)
									}
									loc: SourceLocation es2015/uncategorised/170/input.js 1:17-1:18
								}
							]
							meta: JSPatternMeta {
								loc: SourceLocation es2015/uncategorised/170/input.js 1:12-1:20
							}
							loc: SourceLocation es2015/uncategorised/170/input.js 1:12-1:20
						}
					]
					loc: SourceLocation es2015/uncategorised/170/input.js 1:11-1:21
				}
				loc: SourceLocation es2015/uncategorised/170/input.js 1:1-1:23
			}
			loc: SourceLocation es2015/uncategorised/170/input.js 1:0-1:24
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/170/input.js>
	loc: SourceLocation es2015/uncategorised/170/input.js 1:0-1:24
}
```

### `diagnostics`

```

```