# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-meta-property > new-target-invoke`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "f"
				loc: SourceLocation esprima/es2015-meta-property/new-target-invoke/input.js 1:9-1:10 (f)
			}
			body: JSBlockStatement {
				body: [
					JSExpressionStatement {
						expression: JSCallExpression {
							arguments: []
							callee: JSMetaProperty {
								meta: JSIdentifier {
									name: "new"
									loc: SourceLocation esprima/es2015-meta-property/new-target-invoke/input.js 2:4-2:7 (new)
								}
								property: JSIdentifier {
									name: "target"
									loc: SourceLocation esprima/es2015-meta-property/new-target-invoke/input.js 2:8-2:14 (target)
								}
								loc: SourceLocation esprima/es2015-meta-property/new-target-invoke/input.js 2:4-2:14
							}
							loc: SourceLocation esprima/es2015-meta-property/new-target-invoke/input.js 2:4-2:16
						}
						loc: SourceLocation esprima/es2015-meta-property/new-target-invoke/input.js 2:4-2:17
					}
				]
				directives: []
				loc: SourceLocation esprima/es2015-meta-property/new-target-invoke/input.js 1:13-3:1
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: []
				loc: SourceLocation esprima/es2015-meta-property/new-target-invoke/input.js 1:10-1:12
			}
			loc: SourceLocation esprima/es2015-meta-property/new-target-invoke/input.js 1:0-3:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-meta-property/new-target-invoke/input.js>
	loc: SourceLocation esprima/es2015-meta-property/new-target-invoke/input.js 1:0-4:0
}
```

### `diagnostics`

```

```