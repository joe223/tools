# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > categorized > 01-regex`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "const"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "x"
							loc: SourceLocation core/categorized/01-regex/input.js 1:6-1:7 (x)
						}
						init: JSBinaryExpression {
							operator: "/"
							left: JSBinaryExpression {
								operator: "/"
								left: JSFunctionExpression {
									id: JSBindingIdentifier {
										name: "foo"
										loc: SourceLocation core/categorized/01-regex/input.js 1:19-1:22 (foo)
									}
									body: JSBlockStatement {
										body: []
										directives: []
										loc: SourceLocation core/categorized/01-regex/input.js 1:25-1:27
									}
									head: JSFunctionHead {
										async: false
										generator: false
										hasHoistedVars: false
										params: []
										loc: SourceLocation core/categorized/01-regex/input.js 1:22-1:24
									}
									loc: SourceLocation core/categorized/01-regex/input.js 1:10-1:27
								}
								right: JSNumericLiteral {
									value: 42
									loc: SourceLocation core/categorized/01-regex/input.js 1:29-1:31
								}
								loc: SourceLocation core/categorized/01-regex/input.js 1:10-1:31
							}
							right: JSReferenceIdentifier {
								name: "i"
								loc: SourceLocation core/categorized/01-regex/input.js 1:32-1:33 (i)
							}
							loc: SourceLocation core/categorized/01-regex/input.js 1:10-1:33
						}
						loc: SourceLocation core/categorized/01-regex/input.js 1:6-1:33
					}
				]
				loc: SourceLocation core/categorized/01-regex/input.js 1:0-1:33
			}
			loc: SourceLocation core/categorized/01-regex/input.js 1:0-1:33
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/categorized/01-regex/input.js>
	loc: SourceLocation core/categorized/01-regex/input.js 1:0-2:0
}
```

### `diagnostics`

```

```