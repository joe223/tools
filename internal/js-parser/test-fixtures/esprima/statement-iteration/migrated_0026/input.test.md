# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > statement-iteration > migrated_0026`

### `ast`

```javascript
JSRoot {
	body: [
		JSForInStatement {
			body: JSEmptyStatement {
				loc: SourceLocation esprima/statement-iteration/migrated_0026/input.js 1:15-1:16
			}
			left: JSMemberExpression {
				object: JSReferenceIdentifier {
					name: "a"
					loc: SourceLocation esprima/statement-iteration/migrated_0026/input.js 1:5-1:6 (a)
				}
				property: JSStaticMemberProperty {
					value: JSIdentifier {
						name: "in"
						loc: SourceLocation esprima/statement-iteration/migrated_0026/input.js 1:7-1:9 (in)
					}
					loc: SourceLocation esprima/statement-iteration/migrated_0026/input.js 1:7-1:9 (in)
				}
				loc: SourceLocation esprima/statement-iteration/migrated_0026/input.js 1:5-1:9
			}
			right: JSReferenceIdentifier {
				name: "a"
				loc: SourceLocation esprima/statement-iteration/migrated_0026/input.js 1:13-1:14 (a)
			}
			loc: SourceLocation esprima/statement-iteration/migrated_0026/input.js 1:0-1:16
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/statement-iteration/migrated_0026/input.js>
	loc: SourceLocation esprima/statement-iteration/migrated_0026/input.js 1:0-2:0
}
```

### `diagnostics`

```

```