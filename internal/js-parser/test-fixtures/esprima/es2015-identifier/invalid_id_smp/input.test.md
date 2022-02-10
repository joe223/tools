# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-identifier > invalid_id_smp`

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
							loc: SourceLocation esprima/es2015-identifier/invalid_id_smp/input.js 2:0-2:0 ()
						}
						loc: SourceLocation esprima/es2015-identifier/invalid_id_smp/input.js 2:0-2:0
					}
				]
				loc: SourceLocation esprima/es2015-identifier/invalid_id_smp/input.js 1:0-2:0
			}
			loc: SourceLocation esprima/es2015-identifier/invalid_id_smp/input.js 1:0-2:0
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
				message: [RAW_MARKUP {value: "Unexpected character <emphasis>"}, "\u{1f012}", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<esprima/es2015-identifier/invalid_id_smp/input.js>
				end: Position 1:3
				start: Position 1:4
			}
		}
	]
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-identifier/invalid_id_smp/input.js>
	loc: SourceLocation esprima/es2015-identifier/invalid_id_smp/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-identifier/invalid_id_smp/input.js:1:4 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unexpected character 🀒

    var 🀒
        ^


```