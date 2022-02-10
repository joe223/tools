# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > await-inside-parameters-of-nested-function`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "foo"
				loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 1:15-1:18 (foo)
			}
			body: JSBlockStatement {
				body: [
					JSFunctionDeclaration {
						id: JSBindingIdentifier {
							name: "bar"
							loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:11-2:14 (bar)
						}
						body: JSBlockStatement {
							body: [
								JSExpressionStatement {
									expression: JSNumericLiteral {
										value: 2
										loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:25-2:26
									}
									loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:25-2:26
								}
								JSExpressionStatement {
									expression: JSReferenceIdentifier {
										name: "INVALID_PLACEHOLDER"
										loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:26-2:27
									}
									loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:26-2:27
								}
								JSBlockStatement {
									body: []
									directives: []
									loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:28-2:30
								}
							]
							directives: []
							loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:25-3:1
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: [
								JSBindingAssignmentPattern {
									left: JSBindingIdentifier {
										name: "x"
										meta: JSPatternMeta {
											loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:15-2:16
										}
										loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:15-2:16 (x)
									}
									right: JSReferenceIdentifier {
										name: "await"
										loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:19-2:24 (await)
									}
									loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:15-2:24
								}
							]
							loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:14-2:24
						}
						loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 2:2-3:1
					}
				]
				directives: []
				loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 1:21-3:1
			}
			head: JSFunctionHead {
				async: true
				generator: false
				hasHoistedVars: false
				params: []
				loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 1:18-1:20
			}
			loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 1:0-3:1
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: [RAW_MARKUP {value: "Expected a comma to separate items in "}, "function params"]
			}
			location: {
				language: "js"
				path: UIDPath<es2017/async-functions/await-inside-parameters-of-nested-function/input.js>
				end: Position 2:24
				start: Position 2:25
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2017/async-functions/await-inside-parameters-of-nested-function/input.js>
	loc: SourceLocation es2017/async-functions/await-inside-parameters-of-nested-function/input.js 1:0-3:1
}
```

### `diagnostics`

```

 es2017/async-functions/await-inside-parameters-of-nested-function/input.js:2:25 parse(js) ━━━━━━━━━

  ✖ Expected a comma to separate items in function params

    1 │ async function foo() {
  > 2 │   function bar(x = await 2) {}
      │                          ^
    3 │ }


```