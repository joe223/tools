# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > statement-iteration > migrated_0013`

### `ast`

```javascript
JSRoot {
	body: [
		JSForStatement {
			body: JSEmptyStatement {
				loc: SourceLocation esprima/statement-iteration/migrated_0013/input.js 1:23-1:24
			}
			init: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "x"
							loc: SourceLocation esprima/statement-iteration/migrated_0013/input.js 1:8-1:9 (x)
						}
						init: JSNumericLiteral {
							value: 0
							loc: SourceLocation esprima/statement-iteration/migrated_0013/input.js 1:12-1:13
						}
						loc: SourceLocation esprima/statement-iteration/migrated_0013/input.js 1:8-1:13
					}
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "y"
							loc: SourceLocation esprima/statement-iteration/migrated_0013/input.js 1:15-1:16 (y)
						}
						init: JSNumericLiteral {
							value: 1
							loc: SourceLocation esprima/statement-iteration/migrated_0013/input.js 1:19-1:20
						}
						loc: SourceLocation esprima/statement-iteration/migrated_0013/input.js 1:15-1:20
					}
				]
				loc: SourceLocation esprima/statement-iteration/migrated_0013/input.js 1:4-1:20
			}
			loc: SourceLocation esprima/statement-iteration/migrated_0013/input.js 1:0-1:24
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/statement-iteration/migrated_0013/input.js>
	loc: SourceLocation esprima/statement-iteration/migrated_0013/input.js 1:0-2:0
}
```

### `diagnostics`

```

```