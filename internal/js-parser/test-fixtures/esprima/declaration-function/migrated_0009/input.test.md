# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > declaration-function > migrated_0009`

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
							name: "hi"
							loc: SourceLocation esprima/declaration-function/migrated_0009/input.js 1:4-1:6 (hi)
						}
						init: JSFunctionExpression {
							id: JSBindingIdentifier {
								name: "eval"
								loc: SourceLocation esprima/declaration-function/migrated_0009/input.js 1:18-1:22 (eval)
							}
							body: JSBlockStatement {
								body: []
								directives: []
								loc: SourceLocation esprima/declaration-function/migrated_0009/input.js 1:25-1:28
							}
							head: JSFunctionHead {
								async: false
								generator: false
								hasHoistedVars: false
								params: []
								loc: SourceLocation esprima/declaration-function/migrated_0009/input.js 1:22-1:24
							}
							loc: SourceLocation esprima/declaration-function/migrated_0009/input.js 1:9-1:28
						}
						loc: SourceLocation esprima/declaration-function/migrated_0009/input.js 1:4-1:28
					}
				]
				loc: SourceLocation esprima/declaration-function/migrated_0009/input.js 1:0-1:29
			}
			loc: SourceLocation esprima/declaration-function/migrated_0009/input.js 1:0-1:29
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/declaration-function/migrated_0009/input.js>
	loc: SourceLocation esprima/declaration-function/migrated_0009/input.js 1:0-2:0
}
```

### `diagnostics`

```

```