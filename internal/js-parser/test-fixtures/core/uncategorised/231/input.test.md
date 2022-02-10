# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 231`

### `ast`

```javascript
JSRoot {
	body: [
		JSIfStatement {
			alternate: JSExpressionStatement {
				expression: JSCallExpression {
					arguments: []
					callee: JSReferenceIdentifier {
						name: "goodDay"
						loc: SourceLocation core/uncategorised/231/input.js 1:33-1:40 (goodDay)
					}
					loc: SourceLocation core/uncategorised/231/input.js 1:33-1:42
				}
				loc: SourceLocation core/uncategorised/231/input.js 1:33-1:42
			}
			consequent: JSExpressionStatement {
				expression: JSCallExpression {
					arguments: []
					callee: JSReferenceIdentifier {
						name: "goodMorning"
						loc: SourceLocation core/uncategorised/231/input.js 1:13-1:24 (goodMorning)
					}
					loc: SourceLocation core/uncategorised/231/input.js 1:13-1:26
				}
				loc: SourceLocation core/uncategorised/231/input.js 1:13-1:27
			}
			test: JSReferenceIdentifier {
				name: "morning"
				loc: SourceLocation core/uncategorised/231/input.js 1:4-1:11 (morning)
			}
			loc: SourceLocation core/uncategorised/231/input.js 1:0-1:42
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/231/input.js>
	loc: SourceLocation core/uncategorised/231/input.js 1:0-1:42
}
```

### `diagnostics`

```

```