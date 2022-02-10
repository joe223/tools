# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-yield > invalid-yield-generator-function-declaration`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "g"
				loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js 1:10-1:11 (g)
			}
			body: JSBlockStatement {
				body: [
					JSFunctionDeclaration {
						id: JSBindingIdentifier {
							name: "yield"
							loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js 1:25-1:30 (yield)
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js 1:33-1:35
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js 1:30-1:32
						}
						loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js 1:16-1:35
					}
				]
				directives: []
				loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js 1:14-1:37
			}
			head: JSFunctionHead {
				async: false
				generator: true
				hasHoistedVars: false
				params: []
				loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js 1:11-1:13
			}
			loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js 1:0-1:37
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
				message: RAW_MARKUP {value: "Can not use 'yield' as identifier inside a generator"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js>
				end: Position 1:30
				start: Position 1:25
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js>
	loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-yield/invalid-yield-generator-function-declaration/input.js:1:25 parse(js) ━━━━━━━━━

  ✖ Can not use 'yield' as identifier inside a generator

    function *g() { function yield() {} }
                             ^^^^^


```