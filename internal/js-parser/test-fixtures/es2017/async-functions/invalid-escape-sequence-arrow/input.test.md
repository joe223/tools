# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > invalid-escape-sequence-arrow`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "async"
				loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:0-1:10 (async)
			}
			loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:0-1:10
		}
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSBlockStatement {
					body: [
						JSExpressionStatement {
							expression: JSReferenceIdentifier {
								name: "await"
								loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:18-1:23 (await)
							}
							loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:18-1:23
						}
						JSExpressionStatement {
							expression: JSReferenceIdentifier {
								name: "x"
								loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:24-1:25 (x)
							}
							loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:24-1:25
						}
					]
					directives: []
					loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:16-1:27
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "x"
							loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:11-1:12 (x)
						}
					]
					loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:11-1:15
				}
				loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:11-1:27
			}
			loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:11-1:27
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
				message: RAW_MARKUP {value: "Expected a semicolon or a line terminator"}
			}
			location: {
				language: "js"
				path: UIDPath<es2017/async-functions/invalid-escape-sequence-arrow/input.js>
				end: Position 1:10
				start: Position 1:11
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2017/async-functions/invalid-escape-sequence-arrow/input.js>
	loc: SourceLocation es2017/async-functions/invalid-escape-sequence-arrow/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2017/async-functions/invalid-escape-sequence-arrow/input.js:1:11 parse(js) ━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected a semicolon or a line terminator

    \u0061sync x => { await x }
               ^


```