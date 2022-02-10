# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-for-of > unexpected-number`

### `ast`

```javascript
JSRoot {
	body: [
		JSForStatement {
			body: JSEmptyStatement {
				loc: SourceLocation esprima/es2015-for-of/unexpected-number/input.js 1:17-1:18
			}
			init: JSVariableDeclaration {
				kind: "const"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "of"
							loc: SourceLocation esprima/es2015-for-of/unexpected-number/input.js 1:11-1:13 (of)
						}
						loc: SourceLocation esprima/es2015-for-of/unexpected-number/input.js 1:11-1:13
					}
				]
				loc: SourceLocation esprima/es2015-for-of/unexpected-number/input.js 1:5-1:13
			}
			test: JSNumericLiteral {
				value: 42
				loc: SourceLocation esprima/es2015-for-of/unexpected-number/input.js 1:14-1:16
			}
			loc: SourceLocation esprima/es2015-for-of/unexpected-number/input.js 1:0-1:18
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
				message: RAW_MARKUP {value: "A constant must have an initializer"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/es2015-for-of/unexpected-number/input.js>
				end: Position 1:13
				start: Position 1:11
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-for-of/unexpected-number/input.js>
	loc: SourceLocation esprima/es2015-for-of/unexpected-number/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-for-of/unexpected-number/input.js:1:11 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ A constant must have an initializer

    for (const of 42);
               ^^


```