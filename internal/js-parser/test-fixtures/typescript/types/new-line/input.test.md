# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > types > new-line`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "type"
				loc: SourceLocation typescript/types/new-line/input.ts 1:0-1:4 (type)
			}
			loc: SourceLocation typescript/types/new-line/input.ts 1:0-1:4
		}
		JSExpressionStatement {
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentIdentifier {
					name: "Foo"
					loc: SourceLocation typescript/types/new-line/input.ts 2:0-2:3 (Foo)
				}
				right: JSReferenceIdentifier {
					name: "string"
					loc: SourceLocation typescript/types/new-line/input.ts 2:6-2:12 (string)
				}
				loc: SourceLocation typescript/types/new-line/input.ts 2:0-2:12
			}
			loc: SourceLocation typescript/types/new-line/input.ts 2:0-2:13
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/types/new-line/input.ts>
	loc: SourceLocation typescript/types/new-line/input.ts 1:0-3:0
}
```

### `diagnostics`

```

```