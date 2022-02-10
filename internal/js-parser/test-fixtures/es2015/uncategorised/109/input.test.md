# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 109`

### `ast`

```javascript
JSRoot {
	body: [
		JSForOfStatement {
			await: false
			body: JSExpressionStatement {
				expression: JSCallExpression {
					arguments: [
						JSReferenceIdentifier {
							name: "x"
							loc: SourceLocation es2015/uncategorised/109/input.js 1:33-1:34 (x)
						}
					]
					callee: JSReferenceIdentifier {
						name: "process"
						loc: SourceLocation es2015/uncategorised/109/input.js 1:25-1:32 (process)
					}
					loc: SourceLocation es2015/uncategorised/109/input.js 1:25-1:35
				}
				loc: SourceLocation es2015/uncategorised/109/input.js 1:25-1:36
			}
			left: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "x"
							loc: SourceLocation es2015/uncategorised/109/input.js 1:9-1:10 (x)
						}
						init: JSNumericLiteral {
							value: 42
							loc: SourceLocation es2015/uncategorised/109/input.js 1:13-1:15
						}
						loc: SourceLocation es2015/uncategorised/109/input.js 1:9-1:15
					}
				]
				loc: SourceLocation es2015/uncategorised/109/input.js 1:5-1:15
			}
			right: JSReferenceIdentifier {
				name: "list"
				loc: SourceLocation es2015/uncategorised/109/input.js 1:19-1:23 (list)
			}
			loc: SourceLocation es2015/uncategorised/109/input.js 1:0-1:36
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "Loop variable declaration may not have an initializer"}
			}
			location: {
				language: "js"
				path: UIDPath<es2015/uncategorised/109/input.js>
				end: Position 1:15
				start: Position 1:5
			}
		}
	]
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/109/input.js>
	loc: SourceLocation es2015/uncategorised/109/input.js 1:0-1:36
}
```

### `diagnostics`

```

 es2015/uncategorised/109/input.js:1:5 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Loop variable declaration may not have an initializer

    for (var x = 42 of list) process(x);
         ^^^^^^^^^^


```