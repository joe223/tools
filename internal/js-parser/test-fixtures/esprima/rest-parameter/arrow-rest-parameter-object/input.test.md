# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > rest-parameter > arrow-rest-parameter-object`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSReferenceIdentifier {
					name: "c"
					loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:15-1:16 (c)
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "a"
							loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:1-1:2 (a)
						}
					]
					rest: JSBindingObjectPattern {
						properties: [
							JSBindingObjectPatternProperty {
								key: JSStaticPropertyKey {
									value: JSIdentifier {
										name: "b"
										loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:8-1:9 (b)
									}
									loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:8-1:9
								}
								value: JSBindingIdentifier {
									name: "b"
									loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:8-1:9 (b)
								}
								loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:8-1:9
							}
						]
						loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:7-1:10
					}
					loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:0-1:14
				}
				loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:0-1:16
			}
			loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:0-1:16
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/rest-parameter/arrow-rest-parameter-object/input.js>
	loc: SourceLocation esprima/rest-parameter/arrow-rest-parameter-object/input.js 1:0-2:0
}
```

### `diagnostics`

```

```