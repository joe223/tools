# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2018 > object-rest-spread > 25`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingObjectPattern {
							properties: [
								JSBindingObjectPatternProperty {
									key: JSStaticPropertyKey {
										value: JSIdentifier {
											name: ""
											loc: SourceLocation es2018/object-rest-spread/25/input.js 1:9-1:10 ()
										}
										loc: SourceLocation es2018/object-rest-spread/25/input.js 1:9-1:10
									}
									value: JSBindingIdentifier {
										name: ""
										loc: SourceLocation es2018/object-rest-spread/25/input.js 1:9-1:10 ()
									}
									loc: SourceLocation es2018/object-rest-spread/25/input.js 1:9-1:10
								}
							]
							rest: JSBindingIdentifier {
								name: ""
								loc: SourceLocation es2018/object-rest-spread/25/input.js 1:8-1:9 ()
							}
							loc: SourceLocation es2018/object-rest-spread/25/input.js 1:4-1:11
						}
						init: JSObjectExpression {
							properties: []
							loc: SourceLocation es2018/object-rest-spread/25/input.js 1:14-1:16
						}
						loc: SourceLocation es2018/object-rest-spread/25/input.js 1:4-1:16
					}
				]
				loc: SourceLocation es2018/object-rest-spread/25/input.js 1:0-1:16
			}
			loc: SourceLocation es2018/object-rest-spread/25/input.js 1:0-1:16
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "Expected an identifier"}
			}
			location: {
				language: "js"
				path: UIDPath<es2018/object-rest-spread/25/input.js>
				end: Position 1:8
				start: Position 1:8
			}
		}
	]
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<es2018/object-rest-spread/25/input.js>
	loc: SourceLocation es2018/object-rest-spread/25/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2018/object-rest-spread/25/input.js:1:8 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected an identifier

    var {...[]} = {}
            ^


```