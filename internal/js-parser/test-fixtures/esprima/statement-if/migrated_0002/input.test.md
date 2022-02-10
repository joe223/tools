# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > statement-if > migrated_0002`

### `ast`

```javascript
JSRoot {
	body: [
		JSIfStatement {
			consequent: JSVariableDeclarationStatement {
				declaration: JSVariableDeclaration {
					kind: "var"
					declarations: [
						JSVariableDeclarator {
							id: JSBindingIdentifier {
								name: "x"
								loc: SourceLocation esprima/statement-if/migrated_0002/input.js 1:17-1:18 (x)
							}
							init: JSNumericLiteral {
								value: 0
								loc: SourceLocation esprima/statement-if/migrated_0002/input.js 1:21-1:22
							}
							loc: SourceLocation esprima/statement-if/migrated_0002/input.js 1:17-1:22
						}
					]
					loc: SourceLocation esprima/statement-if/migrated_0002/input.js 1:13-1:23
				}
				loc: SourceLocation esprima/statement-if/migrated_0002/input.js 1:13-1:23
			}
			test: JSReferenceIdentifier {
				name: "morning"
				loc: SourceLocation esprima/statement-if/migrated_0002/input.js 1:4-1:11 (morning)
			}
			loc: SourceLocation esprima/statement-if/migrated_0002/input.js 1:0-1:23
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/statement-if/migrated_0002/input.js>
	loc: SourceLocation esprima/statement-if/migrated_0002/input.js 1:0-2:0
}
```

### `diagnostics`

```

```