# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0075`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSObjectExpression {
				properties: [
					JSObjectMethod {
						kind: "set"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "s"
								loc: SourceLocation esprima/invalid-syntax/migrated_0075/input.js 1:7-1:8 (s)
							}
							loc: SourceLocation esprima/invalid-syntax/migrated_0075/input.js 1:7-1:8
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation esprima/invalid-syntax/migrated_0075/input.js 1:11-1:14
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation esprima/invalid-syntax/migrated_0075/input.js 1:8-1:10
						}
						loc: SourceLocation esprima/invalid-syntax/migrated_0075/input.js 1:3-1:14
					}
				]
				loc: SourceLocation esprima/invalid-syntax/migrated_0075/input.js 1:1-1:16
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0075/input.js 1:0-1:17
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
				message: RAW_MARKUP {value: "setter should have exactly one param"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0075/input.js>
				end: Position 1:14
				start: Position 1:3
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0075/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0075/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0075/input.js:1:3 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ setter should have exactly one param

    ({ set s() { } })
       ^^^^^^^^^^^


```