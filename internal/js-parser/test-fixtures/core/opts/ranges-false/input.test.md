# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > opts > ranges-false`

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
							name: "a"
							loc: SourceLocation core/opts/ranges-false/input.js 1:4-1:5 (a)
						}
						init: JSNumericLiteral {
							value: 1
							loc: SourceLocation core/opts/ranges-false/input.js 1:8-1:9
						}
						loc: SourceLocation core/opts/ranges-false/input.js 1:4-1:9
					}
				]
				loc: SourceLocation core/opts/ranges-false/input.js 1:0-1:10
			}
			loc: SourceLocation core/opts/ranges-false/input.js 1:0-1:10
		}
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "b"
							loc: SourceLocation core/opts/ranges-false/input.js 3:4-3:5 (b)
						}
						init: JSBinaryExpression {
							operator: "+"
							left: JSReferenceIdentifier {
								name: "a"
								loc: SourceLocation core/opts/ranges-false/input.js 3:8-3:9 (a)
							}
							right: JSNumericLiteral {
								value: 1
								loc: SourceLocation core/opts/ranges-false/input.js 3:12-3:13
							}
							loc: SourceLocation core/opts/ranges-false/input.js 3:8-3:13
						}
						loc: SourceLocation core/opts/ranges-false/input.js 3:4-3:13
					}
				]
				loc: SourceLocation core/opts/ranges-false/input.js 3:0-3:14
			}
			loc: SourceLocation core/opts/ranges-false/input.js 3:0-3:14
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<core/opts/ranges-false/input.js>
	loc: SourceLocation core/opts/ranges-false/input.js 1:0-4:0
}
```

### `diagnostics`

```

```