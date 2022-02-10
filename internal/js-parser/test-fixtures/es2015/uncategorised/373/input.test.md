# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 373`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "const"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingObjectPattern {
							properties: [
								JSBindingObjectPatternProperty {
									key: JSStaticPropertyKey {
										value: JSIdentifier {
											name: "enum"
											loc: SourceLocation es2015/uncategorised/373/input.js 1:8-1:12 (enum)
										}
										loc: SourceLocation es2015/uncategorised/373/input.js 1:8-1:12
									}
									value: JSBindingIdentifier {
										name: "enum"
										loc: SourceLocation es2015/uncategorised/373/input.js 1:8-1:12 (enum)
									}
									loc: SourceLocation es2015/uncategorised/373/input.js 1:8-1:12
								}
							]
							loc: SourceLocation es2015/uncategorised/373/input.js 1:6-1:14
						}
						init: JSCallExpression {
							arguments: []
							callee: JSReferenceIdentifier {
								name: "foo"
								loc: SourceLocation es2015/uncategorised/373/input.js 1:17-1:20 (foo)
							}
							loc: SourceLocation es2015/uncategorised/373/input.js 1:17-1:22
						}
						loc: SourceLocation es2015/uncategorised/373/input.js 1:6-1:22
					}
				]
				loc: SourceLocation es2015/uncategorised/373/input.js 1:0-1:23
			}
			loc: SourceLocation es2015/uncategorised/373/input.js 1:0-1:23
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: ["enum", RAW_MARKUP {value: " is a reserved word"}]}
			location: {
				language: "js"
				path: UIDPath<es2015/uncategorised/373/input.js>
				end: Position 1:12
				start: Position 1:8
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: []
	path: UIDPath<es2015/uncategorised/373/input.js>
	loc: SourceLocation es2015/uncategorised/373/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2015/uncategorised/373/input.js:1:8 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ enum is a reserved word

    const { enum } = foo();
            ^^^^


```