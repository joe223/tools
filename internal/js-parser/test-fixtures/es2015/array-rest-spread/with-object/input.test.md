# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > array-rest-spread > with-object`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingArrayPattern {
							elements: []
							rest: JSBindingObjectPattern {
								properties: [
									JSBindingObjectPatternProperty {
										key: JSStaticPropertyKey {
											value: JSIdentifier {
												name: "length"
												loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:9-1:15 (length)
											}
											loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:9-1:15
										}
										value: JSBindingIdentifier {
											name: "length"
											loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:9-1:15 (length)
										}
										loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:9-1:15
									}
								]
								meta: JSPatternMeta {
									loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:8-1:16
								}
								loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:8-1:16
							}
							loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:4-1:17
						}
						init: JSArrayExpression {
							elements: [
								JSNumericLiteral {
									value: 1
									loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:22-1:23
								}
								JSNumericLiteral {
									value: 2
									loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:25-1:26
								}
								JSNumericLiteral {
									value: 3
									loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:28-1:29
								}
							]
							loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:20-1:30
						}
						loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:4-1:30
					}
				]
				loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:0-1:31
			}
			loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:0-1:31
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/array-rest-spread/with-object/input.js>
	loc: SourceLocation es2015/array-rest-spread/with-object/input.js 1:0-2:0
}
```

### `diagnostics`

```

```