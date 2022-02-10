# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 263`

### `ast`

```javascript
JSRoot {
	body: [
		JSWithStatement {
			body: JSExpressionStatement {
				expression: JSAssignmentExpression {
					operator: "="
					left: JSAssignmentIdentifier {
						name: "foo"
						loc: SourceLocation core/uncategorised/263/input.js 1:26-1:29 (foo)
					}
					right: JSReferenceIdentifier {
						name: "bar"
						loc: SourceLocation core/uncategorised/263/input.js 1:32-1:35 (bar)
					}
					loc: SourceLocation core/uncategorised/263/input.js 1:26-1:35
				}
				loc: SourceLocation core/uncategorised/263/input.js 1:26-1:36
			}
			object: JSReferenceIdentifier {
				name: "x"
				loc: SourceLocation core/uncategorised/263/input.js 1:23-1:24 (x)
			}
			loc: SourceLocation core/uncategorised/263/input.js 1:17-1:36
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: [
		JSDirective {
			value: "use\\x20strict"
			loc: SourceLocation core/uncategorised/263/input.js 1:0-1:16
		}
	]
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/263/input.js>
	loc: SourceLocation core/uncategorised/263/input.js 1:0-1:36
}
```

### `diagnostics`

```

```