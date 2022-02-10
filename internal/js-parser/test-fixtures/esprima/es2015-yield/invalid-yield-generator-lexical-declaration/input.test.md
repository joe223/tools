# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-yield > invalid-yield-generator-lexical-declaration`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "g"
				loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js 1:10-1:11 (g)
			}
			body: JSBlockStatement {
				body: [
					JSVariableDeclarationStatement {
						declaration: JSVariableDeclaration {
							kind: "let"
							declarations: [
								JSVariableDeclarator {
									id: JSBindingIdentifier {
										name: "yield"
										loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js 1:20-1:25 (yield)
									}
									loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js 1:20-1:25
								}
							]
							loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js 1:16-1:26
						}
						loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js 1:16-1:26
					}
				]
				directives: []
				loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js 1:14-1:28
			}
			head: JSFunctionHead {
				async: false
				generator: true
				hasHoistedVars: false
				params: []
				loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js 1:11-1:13
			}
			loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js 1:0-1:28
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
				path: UIDPath<esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js>
				end: Position 1:25
				start: Position 1:20
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js>
	loc: SourceLocation esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-yield/invalid-yield-generator-lexical-declaration/input.js:1:20 parse(js) ━━━━━━━━━━

  ✖ Can not use 'yield' as identifier inside a generator

    function *g() { let yield; }
                        ^^^^^


```