# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 542`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSCallExpression {
				arguments: [
					JSStringLiteral {
						value: " "
						loc: SourceLocation core/uncategorised/542/input.js 2:7-2:10
					}
				]
				callee: JSMemberExpression {
					object: JSStringLiteral {
						value: "foo"
						loc: SourceLocation core/uncategorised/542/input.js 1:0-1:5
					}
					property: JSStaticMemberProperty {
						value: JSIdentifier {
							name: "split"
							loc: SourceLocation core/uncategorised/542/input.js 2:1-2:6 (split)
						}
						loc: SourceLocation core/uncategorised/542/input.js 2:1-2:6 (split)
					}
					loc: SourceLocation core/uncategorised/542/input.js 1:0-2:6
				}
				loc: SourceLocation core/uncategorised/542/input.js 1:0-2:11
			}
			loc: SourceLocation core/uncategorised/542/input.js 1:0-2:11
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/542/input.js>
	loc: SourceLocation core/uncategorised/542/input.js 1:0-3:0
}
```

### `diagnostics`

```

```