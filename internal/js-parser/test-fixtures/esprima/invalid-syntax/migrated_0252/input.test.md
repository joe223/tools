# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0252`

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
							name: ""
							loc: SourceLocation esprima/invalid-syntax/migrated_0252/input.js 2:0-2:0 ()
						}
						loc: SourceLocation esprima/invalid-syntax/migrated_0252/input.js 2:0-2:0
					}
				]
				loc: SourceLocation esprima/invalid-syntax/migrated_0252/input.js 1:0-2:0
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0252/input.js 1:0-2:0
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
				path: UIDPath<esprima/invalid-syntax/migrated_0252/input.js>
				end: Position 1:3
				start: Position 2:0
			}
		}
	]
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0252/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0252/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0252/input.js:2 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected an identifier

    var


```