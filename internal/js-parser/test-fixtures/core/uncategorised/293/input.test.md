# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 293`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "hello"
				loc: SourceLocation core/uncategorised/293/input.js 1:9-1:14 (hello)
			}
			body: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation core/uncategorised/293/input.js 1:27-1:30
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: [
					JSBindingIdentifier {
						name: "a"
						meta: JSPatternMeta {
							loc: SourceLocation core/uncategorised/293/input.js 1:15-1:16
						}
						loc: SourceLocation core/uncategorised/293/input.js 1:15-1:16 (a)
					}
				]
				rest: JSBindingIdentifier {
					name: "rest"
					meta: JSPatternMeta {
						loc: SourceLocation core/uncategorised/293/input.js 1:21-1:25
					}
					loc: SourceLocation core/uncategorised/293/input.js 1:21-1:25 (rest)
				}
				loc: SourceLocation core/uncategorised/293/input.js 1:14-1:26
			}
			loc: SourceLocation core/uncategorised/293/input.js 1:0-1:30
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/293/input.js>
	loc: SourceLocation core/uncategorised/293/input.js 1:0-1:30
}
```

### `diagnostics`

```

```